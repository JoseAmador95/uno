use cli::parse_input;
use flow::GameFlow;

mod actor;
mod ai;
mod card;
mod cli;
mod deck;
mod default_deck;
mod flow;
mod game;
mod player;
mod ui;

fn main() -> Result<(), String> {
    let args = parse_input();
    game::check_game_attributes(args.num_of_players, args.num_of_cards)?;
    let mut game = game::Game::new(args.num_of_players, args.num_of_cards);
    game.start_game();
    Ok(())
}
