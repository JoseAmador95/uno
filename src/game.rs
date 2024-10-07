use crate::card;
use crate::deck;
use crate::player;
use crate::ui;

type GameResult<T> = Result<T, Error>;

pub enum Error {
    DrawPileIsEmpty,
    InvalidPlay,
    Unknown,
}

#[derive(Clone, Copy)]
pub enum GameAction {
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
    fn player_draws(player: &mut player::Player, deck: &mut deck::Deck) -> GameResult<GameAction> {
        match player.draw(deck) {
            Ok(()) => Ok(GameAction::PlayerDraw),
            Err(player::Error::DrawPileIsEmpty) => Err(Error::DrawPileIsEmpty),
            _ => Err(Error::Unknown),
        }
    }

    fn player_draws_with_pile_check(
        player: &mut player::Player,
        deck: &mut deck::Deck,
    ) -> GameResult<GameAction> {
        match player.draw(deck) {
            Ok(()) => Ok(GameAction::PlayerDraw),
            Err(player::Error::DrawPileIsEmpty) => {
                let _ = deck.refill_draw_pile(); // No need to check for DiscardPileIsEmpty
                Self::player_draws(player, deck)
            }
            _ => Err(Error::Unknown),
        }
    }

    fn player_draws_multiple(
        &mut self,
        player_index: usize,
        num_of_cards: usize,
    ) -> GameResult<()> {
        for _ in 0..num_of_cards {
            Self::player_draws_with_pile_check(&mut self.players[player_index], &mut self.deck)?;
        }

        Ok(())
    }

    fn change_wild_color(card: &mut card::Card) {
        let colour = ui::get_user_wild_colour();
        card.colour = colour;
    }

    fn choose_colur_and_draw(
        &mut self,
        next_player_index: usize,
        num_of_cards: usize,
        card: &mut card::Card,
    ) {
        if let Err(Error::DrawPileIsEmpty) =
            self.player_draws_multiple(next_player_index, num_of_cards)
        {
            // There are not enough cards on the draw and discard piles to take two cards
        }

        Self::change_wild_color(card);
    }

    fn handle_draw_two(&mut self, next_player_index: usize) {
        if let Err(Error::DrawPileIsEmpty) = self.player_draws_multiple(next_player_index, 2) {
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
        match ui::get_user_turn_action() {
            ui::UserAction::Draw => Ok(GameAction::PlayerDraw),
            ui::UserAction::Play(i) => {
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

    pub fn wait_for_player_action(&self, player: &player::Player) -> GameAction {
        loop {
            if let Ok(a) = self.get_player_action(player) {
                break a;
            }
        }
    }

    pub fn execute_player_action(
        &mut self,
        player_index: usize,
        action: &GameAction,
    ) -> GameResult<GameAction> {
        match action {
            GameAction::PlayerDraw => {
                Self::player_draws_with_pile_check(&mut self.players[player_index], &mut self.deck)
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

    pub fn deal_cards_to_players(&mut self) {
        let num_of_players = self.players.len();
        for i in 0..num_of_players {
            assert!(
                self.player_draws_multiple(i, self.num_of_cards).is_ok(),
                "Failed to deal cards at the start of the game"
            );
        }
    }

    pub fn set_next_player(&mut self) {
        self.player_index = self.get_next_player(self.player_index);
    }

    pub fn has_player_won(&self, player_index: usize) -> bool {
        self.players[player_index].is_hand_empty()
    }

    pub fn get_deck(&self) -> &deck::Deck {
        &self.deck
    }

    pub fn get_current_player(&self) -> &player::Player {
        &self.players[self.player_index]
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
        self.deal_cards_to_players();

        let winner = loop {
            ui::get_game_context(&self.players[self.player_index], &self.deck);
            let action = self.wait_for_player_action(&self.players[self.player_index]);
            let _ = self.execute_player_action(self.player_index, &action);
            if self.has_player_won(self.player_index) {
                break self.player_index;
            }
            self.set_next_player();
        };

        ui::announce_winner(&self.players[winner]);
    }
}
