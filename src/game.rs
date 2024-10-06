use crate::card;
use crate::deck;
use crate::player;
use crate::ui::{
    announce_winner, get_game_context, get_user_turn_action, get_user_wild_colour, UserAction,
};

type GameResult<T> = Result<T, Error>;

pub enum Error {
    DrawPileIsEmpty,
    InvalidPlay,
    Unknown,
}

enum GameAction {
    PlayerDraw,
    PlayerPlaysCard(usize),
}

pub fn check_game_attributes(num_of_players: usize, num_of_cards: usize) -> Result<(), String> {
    if num_of_cards > 10 {
        return Err("The maximum number of cards is 10".to_string());
    }

    if num_of_players > 10 {
        return Err("The maximum number of players is 10".to_string());
    }

    Ok(())
}

pub struct Game {
    players: Vec<player::Player>,
    deck: deck::Deck,
    player_index: usize,
    is_direction_ascending: bool,
    num_of_cards: usize,
}

impl Game {
    fn player_draws(
        player: &mut player::Player,
        deck: &mut deck::Deck,
        refill_draw_pile_if_empty: bool,
    ) -> GameResult<GameAction> {
        match player.draw(deck) {
            Ok(()) => Ok(GameAction::PlayerDraw),
            Err(player::Error::DrawPileIsEmpty) => {
                if refill_draw_pile_if_empty {
                    let _ = deck.refill_draw_pile(); // No need to check for DiscardPileIsEmpty
                    Self::player_draws(player, deck, false)
                } else {
                    Err(Error::DrawPileIsEmpty)
                }
            }
            _ => Err(Error::Unknown),
        }
    }

    fn make_player_draw(&mut self, player_index: usize, num_of_cards: usize) -> GameResult<()> {
        for _ in 0..num_of_cards {
            Self::player_draws(&mut self.players[player_index], &mut self.deck, true)?;
        }

        Ok(())
    }

    fn change_wild_color(card: &mut card::Card) {
        let colour = get_user_wild_colour();
        card.colour = colour;
    }

    fn choose_colur_and_draw(
        &mut self,
        next_player_index: usize,
        num_of_cards: usize,
        card: &mut card::Card,
    ) {
        if let Err(Error::DrawPileIsEmpty) = self.make_player_draw(next_player_index, num_of_cards)
        {
            // There are not enough cards on the draw and discard piles to take two cards
        }

        Self::change_wild_color(card);
    }

    fn handle_draw_two(&mut self, next_player_index: usize) {
        if let Err(Error::DrawPileIsEmpty) = self.make_player_draw(next_player_index, 2) {
            // There are not enough cards on the draw and discard piles to take two cards
        }
    }

    fn execute_card_action(&mut self, player_index: usize, card: &mut card::Card) {
        match card.value {
            card::Value::DrawTwo => self.handle_draw_two(self.get_next_player(player_index)),
            card::Value::Skip => self.set_next_player(),
            card::Value::Reverse => self.revese_direction(),
            card::Value::Wild => Self::change_wild_color(card),
            card::Value::WildDraw(n) => {
                self.choose_colur_and_draw(self.get_next_player(player_index), n, card);
            }
            card::Value::Number(_) => {}
        };
    }

    fn get_next_player(&self, current_player_index: usize) -> usize {
        if !self.is_direction_ascending && current_player_index == 0 {
            self.players.len() - 1
        } else {
            let index_increment: isize = if self.is_direction_ascending { 1 } else { -1 };
            (current_player_index.wrapping_add_signed(index_increment)) % self.players.len()
        }
    }

    fn set_next_player(&mut self) {
        self.player_index = self.get_next_player(self.player_index);
    }

    fn revese_direction(&mut self) {
        self.is_direction_ascending = !self.is_direction_ascending;
    }

    fn is_valid_play(&self, card: &card::Card) -> bool {
        match self.deck.get_top_card() {
            Ok(card_on_top) => {
                card.colour == card_on_top.colour
                    || card.value == card_on_top.value
                    || card.colour == card::Colour::Wild
                    || card_on_top.colour == card::Colour::Wild
            }
            Err(deck::Error::DiscardPileIsEmpty) => {
                // There is no card on top of the discard pile (for some reason)
                // So might as well play whatever the player wants
                true
            }
            Err(_) => unimplemented!(),
        }
    }

    fn get_player_action(&self, player: &player::Player) -> GameResult<GameAction> {
        match get_user_turn_action() {
            UserAction::Draw => Ok(GameAction::PlayerDraw),
            UserAction::Play(i) => {
                if let Ok(card) = player.get_card(i) {
                    if self.is_valid_play(card) {
                        Ok(GameAction::PlayerPlaysCard(i))
                    } else {
                        Err(Error::InvalidPlay)
                    }
                } else {
                    Err(Error::InvalidPlay)
                }
            }
        }
    }

    fn wait_for_player_action(&self, player: &player::Player) -> GameAction {
        loop {
            if let Ok(a) = self.get_player_action(player) {
                break a;
            }
        }
    }

    fn play_turn(&mut self, player_index: usize, action: &GameAction) -> GameResult<GameAction> {
        match action {
            GameAction::PlayerDraw => {
                Self::player_draws(&mut self.players[player_index], &mut self.deck, true)
            }
            GameAction::PlayerPlaysCard(index) => {
                if let Ok(mut card) = self.players[player_index].play_card(*index) {
                    self.execute_card_action(player_index, &mut card);
                    self.deck.discard(card);
                    Ok(GameAction::PlayerPlaysCard(*index))
                } else {
                    Err(Error::Unknown)
                }
            }
        }
    }

    fn deal_cards_to_players(&mut self, num_of_cards: usize) {
        let num_of_players = self.players.len();
        for i in 0..num_of_players {
            assert!(
                self.make_player_draw(i, num_of_cards).is_ok(),
                "Failed to deal cards at the start of the game"
            );
        }
    }

    fn has_player_won(&self, player_index: usize) -> bool {
        self.players[player_index].is_hand_empty()
    }

    pub fn new(num_of_players: usize, num_of_cards: usize) -> Self {
        let players = (0..num_of_players).map(player::Player::new).collect();
        Game {
            players,
            deck: deck::Deck::new(),
            player_index: 0,
            is_direction_ascending: true,
            num_of_cards,
        }
    }

    pub fn start_game(&mut self) {
        self.deal_cards_to_players(self.num_of_cards);

        let winner = loop {
            get_game_context(&self.players[self.player_index], &self.deck);
            let action = self.wait_for_player_action(&self.players[self.player_index]);
            let _ = self.play_turn(self.player_index, &action);
            if self.has_player_won(self.player_index) {
                break self.player_index;
            }
            self.set_next_player();
        };

        announce_winner(&self.players[winner]);
    }
}
