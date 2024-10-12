use crate::actor;
use crate::ai;
use crate::game;
use crate::ui;

enum GameState {
    Init,
    TurnStarts,
    GetPlayerAction,
    ExecutePlayerAction(game::GameAction),
    ChooseColour,
    EndTurn,
    EndGame,
    End,
}

pub struct GameFlow {
    game: game::Game,
    state: GameState,
    actors: Vec<Box<dyn actor::Actor>>,
}

impl GameFlow {
    pub fn new(num_of_players: usize, num_of_cards: usize) -> Self {
        let game = game::Game::new(num_of_players, num_of_cards);
        let mut actors: Vec<Box<dyn actor::Actor>> =
            vec![Box::new(ui::HumanActor::new(game.get_player(0).get_id()))];
        actors.extend((1..num_of_players).map(|i| {
            Box::new(ai::AiActor::new(game.get_player(i).get_id())) as Box<dyn actor::Actor>
        }));

        GameFlow {
            game,
            state: GameState::Init,
            actors,
        }
    }

    pub fn start_game(&mut self) {
        let mut continue_game = true;
        while continue_game {
            self.run_game_phase();
            continue_game = !matches!(self.state, GameState::End);
        }
    }

    pub fn run_game_phase(&mut self) {
        self.state = match self.state {
            GameState::Init => self.handle_init(),
            GameState::TurnStarts => self.handle_turn_start(),
            GameState::GetPlayerAction => self.handle_get_player_action(),
            GameState::ExecutePlayerAction(action) => self.handle_execute_player_action(&action),
            GameState::ChooseColour => self.handle_choose_colour(),
            GameState::EndTurn => self.handle_end_turn(),
            GameState::EndGame => self.handle_end_game(),
            GameState::End => GameState::End,
        };
    }

    fn handle_init(&mut self) -> GameState {
        self.game.deal_cards_to_players();
        GameState::TurnStarts
    }

    fn handle_turn_start(&mut self) -> GameState {
        let player = self.game.get_current_player();
        self.actors[player.get_id()].pre_turn_action(&self.game);
        GameState::GetPlayerAction
    }

    fn handle_get_player_action(&mut self) -> GameState {
        let player = self.game.get_current_player();
        let action = self.actors[player.get_id()].get_turn_action(&self.game);
        match self.game.get_player_action(player, action) {
            Ok(game::GameAction::PlayerDraw) => {
                GameState::ExecutePlayerAction(game::GameAction::PlayerDraw)
            }
            Ok(game::GameAction::PlayerPlaysCard(i)) => {
                GameState::ExecutePlayerAction(game::GameAction::PlayerPlaysCard(i))
            }
            _ => GameState::GetPlayerAction,
        }
    }

    fn handle_execute_player_action(&mut self, action: &game::GameAction) -> GameState {
        let player = self.game.get_current_player();
        let action = self.game.execute_player_action(player.get_id(), action);
        match action {
            Ok(game::GameAction::ChooseColour) => GameState::ChooseColour,
            _ => GameState::EndTurn,
        }
    }

    fn handle_choose_colour(&mut self) -> GameState {
        let player = self.game.get_current_player();
        let colour = self.actors[player.get_id()].get_color_choice(&self.game);
        self.game.change_wild_color(&colour);
        GameState::EndTurn
    }

    fn handle_end_turn(&mut self) -> GameState {
        let player = self.game.get_current_player();
        if self.game.has_player_won(player.get_id()) {
            return GameState::EndGame;
        }
        self.actors[player.get_id()].post_turn_action(&self.game);
        self.game.set_next_player();
        GameState::TurnStarts
    }

    fn handle_end_game(&mut self) -> GameState {
        let player = self.game.get_current_player();
        ui::announce_winner(player);
        GameState::End
    }
}
