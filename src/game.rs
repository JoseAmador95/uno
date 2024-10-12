use crate::actor;
use crate::card;
use crate::deck;
use crate::player;

type GameResult<T> = Result<T, Error>;

pub enum Error {
    DrawPileIsEmpty,
    InvalidPlay,
    Unknown,
}

#[derive(Clone, Copy)]
pub enum GameAction {
    None,
    PlayerDraw,
    PlayerPlaysCard(usize),
    ChooseColour,
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
    is_flow_clockwise: bool,
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

    pub fn change_wild_color(&mut self, colour: &card::Colour) {
        self.deck.change_colour_of_top_card_in_discard(colour);
    }

    fn handle_wild_draw(&mut self, next_player_index: usize, num_of_cards: usize) -> GameAction {
        if let Err(Error::DrawPileIsEmpty) =
            self.player_draws_multiple(next_player_index, num_of_cards)
        {
            // There are not enough cards on the draw and discard piles to take two cards
        }

        self.handle_wild()
    }

    fn handle_wild(&mut self) -> GameAction {
        // Self::change_wild_color(card);
        GameAction::ChooseColour
    }

    fn handle_reverse(&mut self) -> GameAction {
        self.revese_direction();
        GameAction::None
    }

    fn handle_skip(&mut self) -> GameAction {
        self.set_next_player();
        GameAction::None
    }

    fn handle_draw_two(&mut self, next_player_index: usize) -> GameAction {
        if let Err(Error::DrawPileIsEmpty) = self.player_draws_multiple(next_player_index, 2) {
            // There are not enough cards on the draw and discard piles to take two cards
        }
        GameAction::None
    }

    fn execute_card_action(&mut self, player_index: usize, card: &mut card::Card) -> GameAction {
        match card.value {
            card::Value::DrawTwo => self.handle_draw_two(self.get_next_player(player_index)),
            card::Value::Skip => self.handle_skip(),
            card::Value::Reverse => self.handle_reverse(),
            card::Value::Wild => self.handle_wild(),
            card::Value::WildDraw(n) => {
                self.handle_wild_draw(self.get_next_player(player_index), n)
            }
            card::Value::Number(_) => GameAction::None,
        }
    }

    fn get_next_player(&self, current_player_index: usize) -> usize {
        if !self.is_flow_clockwise && current_player_index == 0 {
            self.players.len() - 1
        } else {
            let index_increment: isize = if self.is_flow_clockwise { 1 } else { -1 };
            (current_player_index.wrapping_add_signed(index_increment)) % self.players.len()
        }
    }

    fn revese_direction(&mut self) {
        self.is_flow_clockwise = !self.is_flow_clockwise;
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

    pub fn get_player_action(
        &self,
        player: &player::Player,
        action: actor::UserAction,
    ) -> GameResult<GameAction> {
        match action {
            actor::UserAction::Draw => Ok(GameAction::PlayerDraw),
            actor::UserAction::Play(i) => {
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
                    let result = self.execute_card_action(player_index, &mut card);
                    self.deck.discard(card);
                    Ok(result)
                } else {
                    Err(Error::Unknown)
                }
            }
            _ => Ok(GameAction::None),
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

    pub fn get_player(&self, index: usize) -> &player::Player {
        &self.players[index]
    }

    pub fn get_current_player(&self) -> &player::Player {
        self.get_player(self.player_index)
    }

    pub fn new(num_of_players: usize, num_of_cards: usize) -> Self {
        let players = (0..num_of_players).map(player::Player::new).collect();
        Game {
            players,
            deck: deck::Deck::new(),
            player_index: 0,
            is_flow_clockwise: true,
            num_of_cards,
        }
    }
}
