use crate::game;
use crate::ui;

enum GameState {
    Init,
    TurnStarts,
    GetPlayerAction,
    ExecutePlayerAction(game::GameAction),
    EndTurn,
    EndGame,
}

pub struct GameFlow {
    game: game::Game,
    state: GameState,
}

impl GameFlow {
    pub fn new(num_of_players: usize, num_of_cards: usize) -> Self {
        GameFlow {
            game: game::Game::new(num_of_players, num_of_cards),
            state: GameState::Init,
        }
    }

    pub fn start_game(&mut self) {
        let mut continue_game = true;
        while continue_game {
            self.run_game_phase();
            continue_game = !matches!(self.state, GameState::EndGame);
        }
    }

    pub fn run_game_phase(&mut self) {
        self.state = match self.state {
            GameState::Init => self.handle_init(),
            GameState::TurnStarts => self.handle_turn_start(),
            GameState::GetPlayerAction => self.handle_get_player_action(),
            GameState::ExecutePlayerAction(action) => self.handle_execute_player_action(&action),
            GameState::EndTurn => self.handle_end_turn(),
            GameState::EndGame => self.handle_end_game(),
        };
    }

    fn handle_init(&mut self) -> GameState {
        self.game.deal_cards_to_players();
        GameState::TurnStarts
    }

    fn handle_turn_start(&mut self) -> GameState {
        let player = self.game.get_current_player();
        let deck = self.game.get_deck();
        ui::get_game_context(player, deck);
        GameState::GetPlayerAction
    }

    fn handle_get_player_action(&mut self) -> GameState {
        let player = self.game.get_current_player();
        let action = self.game.wait_for_player_action(player);
        GameState::ExecutePlayerAction(action)
    }

    fn handle_execute_player_action(&mut self, action: &game::GameAction) -> GameState {
        let player = self.game.get_current_player();
        let _ = self.game.execute_player_action(player.get_id(), action);
        GameState::EndTurn
    }

    fn handle_end_turn(&mut self) -> GameState {
        let player = self.game.get_current_player();
        if self.game.has_player_won(player.get_id()) {
            return GameState::EndGame;
        }
        self.game.set_next_player();
        GameState::TurnStarts
    }

    fn handle_end_game(&mut self) -> GameState {
        let player = self.game.get_current_player();
        ui::announce_winner(player);
        GameState::EndGame
    }
}
