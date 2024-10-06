use crate::card::{Card, CardValue, Colour};
use crate::deck::{Deck, DeckError};
use crate::player::{Player, PlayerError};
use crate::ui::{
    announce_winner, get_game_context, get_user_turn_action, get_user_wild_colour, UserAction,
};

type GameResult<T> = Result<T, GameError>;

pub enum GameError {
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
    players: Vec<Player>,
    deck: Deck,
    player_index: usize,
    is_direction_ascending: bool,
    num_of_cards: usize,
}

impl Game {
    fn player_draws(
        player: &mut Player,
        deck: &mut Deck,
        refill_draw_pile_if_empty: bool,
    ) -> GameResult<GameAction> {
        match player.draw(deck) {
            Ok(_) => Ok(GameAction::PlayerDraw),
            Err(PlayerError::DrawPileIsEmpty) => {
                if refill_draw_pile_if_empty {
                    let _ = deck.refill_draw_pile(); // No need to check for DiscardPileIsEmpty
                    Self::player_draws(player, deck, false)
                } else {
                    Err(GameError::DrawPileIsEmpty)
                }
            }
            _ => Err(GameError::Unknown),
        }
    }

    fn make_player_draw(&mut self, player_index: usize, num_of_cards: usize) -> GameResult<()> {
        for _ in 0..num_of_cards {
            Self::player_draws(&mut self.players[player_index], &mut self.deck, true)?;
        }

        Ok(())
    }

    fn change_wild_color(&mut self, card: &mut Card) {
        let colour = get_user_wild_colour();
        card.colour = colour;
    }

    fn choose_colur_and_draw(
        &mut self,
        next_player_index: usize,
        num_of_cards: usize,
        card: &mut Card,
    ) {
        match self.make_player_draw(next_player_index, num_of_cards) {
            Ok(_) => {}
            Err(GameError::DrawPileIsEmpty) => {
                todo!("There are not enough cards on the draw and discard piles to take two cards")
            }
            _ => {}
        }
        self.change_wild_color(card);
    }

    fn handle_draw_two(&mut self, next_player_index: usize) {
        match self.make_player_draw(next_player_index, 2) {
            Ok(_) => {}
            Err(GameError::DrawPileIsEmpty) => {
                todo!("There are not enough cards on the draw and discard piles to take two cards")
            }
            _ => {}
        }
    }

    fn execute_card_action(&mut self, player_index: usize, card: &mut Card) {
        match card.value {
            CardValue::DrawTwo => self.handle_draw_two(self.get_next_player(player_index)),
            CardValue::Skip => self.set_next_player(),
            CardValue::Reverse => self.revese_direction(),
            CardValue::Wild => self.change_wild_color(card),
            CardValue::WildDraw(n) => {
                self.choose_colur_and_draw(self.get_next_player(player_index), n, card);
            }
            _ => {}
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

    fn is_valid_play(&self, card: &Card) -> bool {
        match self.deck.get_top_card() {
            Ok(card_on_top) => {
                card.colour == card_on_top.colour
                    || card.value == card_on_top.value
                    || card.colour == Colour::Wild
                    || card_on_top.colour == Colour::Wild
            }
            Err(DeckError::DiscardPileIsEmpty) => {
                // There is no card on top of the discard pile (for some reason)
                // So might as well play whatever the player wants
                true
            }
            Err(_) => true,
        }
    }

    fn get_player_action(&self, player: &Player) -> GameResult<GameAction> {
        match get_user_turn_action() {
            UserAction::Draw => Ok(GameAction::PlayerDraw),
            UserAction::Play(i) => {
                if let Ok(card) = player.get_card(i) {
                    if self.is_valid_play(card) {
                        Ok(GameAction::PlayerPlaysCard(i))
                    } else {
                        Err(GameError::InvalidPlay)
                    }
                } else {
                    Err(GameError::InvalidPlay)
                }
            }
        }
    }

    fn wait_for_player_action(&self, player: &Player) -> GameAction {
        loop {
            if let Ok(a) = self.get_player_action(player) {
                break a;
            }
        }
    }

    fn play_turn(&mut self, player_index: usize, action: GameAction) -> GameResult<GameAction> {
        match action {
            GameAction::PlayerDraw => {
                Self::player_draws(&mut self.players[player_index], &mut self.deck, true)
            }
            GameAction::PlayerPlaysCard(index) => {
                if let Ok(mut card) = self.players[player_index].play_card(index) {
                    self.execute_card_action(player_index, &mut card);
                    self.deck.discard(card);
                    Ok(GameAction::PlayerPlaysCard(index))
                } else {
                    Err(GameError::Unknown)
                }
            }
        }
    }

    fn deal_cards_to_players(&mut self, num_of_cards: usize) {
        let num_of_players = self.players.len();
        for i in 0..num_of_players {
            if self.make_player_draw(i, num_of_cards).is_err() {
                panic!("Failed to deal cards at the start of the game");
            }
        }
    }

    fn has_player_won(&self, player_index: usize) -> bool {
        self.players[player_index].is_hand_empty()
    }

    pub fn new(num_of_players: usize, num_of_cards: usize) -> Self {
        let players = (0..num_of_players).map(Player::new).collect();
        Game {
            players,
            deck: Deck::new(),
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
            let _ = self.play_turn(self.player_index, action);
            if self.has_player_won(self.player_index) {
                break self.player_index;
            }
            self.set_next_player();
        };

        announce_winner(&self.players[winner]);
    }
}
