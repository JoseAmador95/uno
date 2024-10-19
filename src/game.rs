use crate::actor;
use crate::ai;
use crate::card;
use crate::deck;
use crate::deck::DeckTrait;
use crate::flow;
use crate::player;
use crate::ui;

type GameResult<T> = Result<T, Error>;
type GameActor = Box<dyn actor::Actor>;

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
    state: flow::GameState,
    // players: Vec<player::Player>,
    actors: Vec<GameActor>,
    deck: deck::Deck,
    actor_index: usize,
    is_flow_clockwise: bool,
    num_of_cards: usize,
}

impl Game {
    fn player_draws(&mut self, actor_index: usize) -> GameResult<GameAction> {
        match self.deck.draw() {
            Ok(card) => {
                self.get_actor_mut(actor_index)
                    .get_player_mut()
                    .take_card(card);
                Ok(GameAction::PlayerDraw)
            }
            Err(deck::Error::DrawPileIsEmpty) => Err(Error::DrawPileIsEmpty),
            _ => Err(Error::Unknown),
        }
    }

    fn player_draws_with_pile_check(&mut self, actor_index: usize) -> GameResult<GameAction> {
        match self.player_draws(actor_index) {
            Ok(_) => Ok(GameAction::PlayerDraw),
            Err(Error::DrawPileIsEmpty) => {
                let _ = self.deck.refill_draw_pile(); // No need to check for DiscardPileIsEmpty
                self.player_draws(actor_index)
            }
            _ => Err(Error::Unknown),
        }
    }

    fn player_draws_multiple(&mut self, actor_index: usize, num_of_cards: usize) -> GameResult<()> {
        for _ in 0..num_of_cards {
            self.player_draws_with_pile_check(actor_index)?;
        }

        Ok(())
    }

    pub fn change_wild_color(&mut self, colour: &card::Colour) {
        self.deck.change_colour_of_top_card_in_discard(colour);
    }

    fn handle_wild_draw(&mut self, affected_actor_index: usize, num_of_cards: usize) -> GameAction {
        if let Err(Error::DrawPileIsEmpty) =
            self.player_draws_multiple(affected_actor_index, num_of_cards)
        {
            // There are not enough cards on the draw and discard piles to take two cards
        }

        self.handle_wild()
    }

    fn handle_wild(&mut self) -> GameAction {
        GameAction::ChooseColour
    }

    fn handle_reverse(&mut self) -> GameAction {
        self.revese_direction();
        GameAction::None
    }

    fn handle_skip(&mut self) -> GameAction {
        self.set_next_actor();
        GameAction::None
    }

    fn handle_draw_two(&mut self, affected_actor_index: usize) -> GameAction {
        if let Err(Error::DrawPileIsEmpty) = self.player_draws_multiple(affected_actor_index, 2) {
            // There are not enough cards on the draw and discard piles to take two cards
        }
        GameAction::None
    }

    fn execute_card_action(&mut self, actor_index: usize, card: &mut card::Card) -> GameAction {
        match card.value {
            card::Value::DrawTwo => self.handle_draw_two(self.get_next_player(actor_index)),
            card::Value::Skip => self.handle_skip(),
            card::Value::Reverse => self.handle_reverse(),
            card::Value::Wild => self.handle_wild(),
            card::Value::WildDraw(n) => self.handle_wild_draw(self.get_next_player(actor_index), n),
            card::Value::Number(_) => GameAction::None,
        }
    }

    fn get_next_player(&self, current_actor_index: usize) -> usize {
        if !self.is_flow_clockwise && current_actor_index == 0 {
            self.actors.len() - 1
        } else {
            let index_increment: isize = if self.is_flow_clockwise { 1 } else { -1 };
            (current_actor_index.wrapping_add_signed(index_increment)) % self.actors.len()
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
        actor_index: usize,
        action: &GameAction,
    ) -> GameResult<GameAction> {
        match action {
            GameAction::PlayerDraw => self.player_draws_with_pile_check(actor_index),
            GameAction::PlayerPlaysCard(index) => {
                if let Ok(mut card) = self
                    .get_actor_mut(actor_index)
                    .get_player_mut()
                    .play_card(*index)
                {
                    let result = self.execute_card_action(actor_index, &mut card);
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
        let num_of_players = self.actors.len();
        for i in 0..num_of_players {
            assert!(
                self.player_draws_multiple(i, self.num_of_cards).is_ok(),
                "Failed to deal cards at the start of the game"
            );
        }
    }

    pub fn set_next_actor(&mut self) {
        self.actor_index = self.get_next_player(self.actor_index);
    }

    pub fn has_player_won(&self, actor_index: usize) -> bool {
        self.get_actor(actor_index).get_player().is_hand_empty()
    }

    pub fn get_actor_mut(&mut self, index: usize) -> &mut GameActor {
        &mut self.actors[index]
    }

    pub fn get_current_actor_mut(&mut self) -> &mut GameActor {
        self.get_actor_mut(self.actor_index)
    }

    pub fn get_actor(&self, index: usize) -> &GameActor {
        &self.actors[index]
    }

    pub fn get_current_actor(&self) -> &GameActor {
        self.get_actor(self.actor_index)
    }

    pub fn new(num_of_players: usize, num_of_cards: usize) -> Self {
        let mut actors: Vec<Box<dyn actor::Actor>> = vec![Box::new(ui::HumanActor::new(0))];
        actors.extend(
            (1..num_of_players).map(|i| Box::new(ai::AiActor::new(i)) as Box<dyn actor::Actor>),
        );

        Game {
            state: flow::GameState::Init,
            actors,
            deck: deck::Deck::new(None),
            actor_index: 0,
            is_flow_clockwise: true,
            num_of_cards,
        }
    }
}

impl flow::GameFlow for Game {
    fn get_state(&self) -> flow::GameState {
        self.state
    }

    fn set_state(&mut self, state: flow::GameState) {
        self.state = state;
    }

    fn handle_init(&mut self) -> flow::GameState {
        self.deal_cards_to_players();
        flow::GameState::TurnStarts
    }

    fn handle_turn_start(&mut self) -> flow::GameState {
        ui::get_game_context(self.actor_index, &self.deck);
        self.get_current_actor_mut().pre_turn_action();
        flow::GameState::GetPlayerAction
    }

    fn handle_get_player_action(&mut self) -> flow::GameState {
        let action = self.get_current_actor_mut().get_turn_action();
        match self.get_player_action(self.get_current_actor().get_player(), action) {
            Ok(GameAction::PlayerDraw) => {
                flow::GameState::ExecutePlayerAction(GameAction::PlayerDraw)
            }
            Ok(GameAction::PlayerPlaysCard(i)) => {
                flow::GameState::ExecutePlayerAction(GameAction::PlayerPlaysCard(i))
            }
            _ => flow::GameState::GetPlayerAction,
        }
    }

    fn handle_execute_player_action(&mut self, action: &GameAction) -> flow::GameState {
        let actor = self.get_current_actor();
        let action = self.execute_player_action(actor.get_id(), action);
        match action {
            Ok(GameAction::ChooseColour) => flow::GameState::ChooseColour,
            _ => flow::GameState::EndTurn,
        }
    }

    fn handle_choose_colour(&mut self) -> flow::GameState {
        let actor = self.get_current_actor_mut();
        let colour = actor.get_color_choice();
        self.change_wild_color(&colour);
        flow::GameState::EndTurn
    }

    fn handle_end_turn(&mut self) -> flow::GameState {
        let player = self.get_current_actor();
        if self.has_player_won(player.get_id()) {
            return flow::GameState::EndGame;
        }
        self.get_current_actor_mut().post_turn_action();
        self.set_next_actor();
        flow::GameState::TurnStarts
    }

    fn handle_end_game(&mut self) -> flow::GameState {
        let actor = self.get_current_actor();
        ui::announce_winner(actor.get_id());
        flow::GameState::End
    }
}
