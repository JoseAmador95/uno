use crate::game;

#[derive(Clone, Copy)]
pub enum GameState {
    Init,
    TurnStarts,
    GetPlayerAction,
    ExecutePlayerAction(game::GameAction),
    ChooseColour,
    EndTurn,
    EndGame,
    End,
}

pub trait GameFlow {
    fn start_game(&mut self) {
        let mut continue_game = true;
        while continue_game {
            let state = self.get_state();
            self.run_game_phase(state);
            let new_state = self.get_state();
            continue_game = !matches!(new_state, GameState::End);
        }
    }

    fn run_game_phase(&mut self, state: GameState) {
        let new_state = match state {
            GameState::Init => self.handle_init(),
            GameState::TurnStarts => self.handle_turn_start(),
            GameState::GetPlayerAction => self.handle_get_player_action(),
            GameState::ExecutePlayerAction(action) => self.handle_execute_player_action(&action),
            GameState::ChooseColour => self.handle_choose_colour(),
            GameState::EndTurn => self.handle_end_turn(),
            GameState::EndGame => self.handle_end_game(),
            GameState::End => GameState::End,
        };
        self.set_state(new_state);
    }

    fn set_state(&mut self, state: GameState);
    fn get_state(&self) -> GameState;
    fn handle_init(&mut self) -> GameState;
    fn handle_turn_start(&mut self) -> GameState;
    fn handle_get_player_action(&mut self) -> GameState;
    fn handle_execute_player_action(&mut self, action: &game::GameAction) -> GameState;
    fn handle_choose_colour(&mut self) -> GameState;
    fn handle_end_turn(&mut self) -> GameState;
    fn handle_end_game(&mut self) -> GameState;
}
