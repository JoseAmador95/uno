use cli::parse_cli;
use game::{check_game_attributes, Game};

mod card;
mod cli;
mod deck;
mod default_deck;
mod game;
mod player;
mod ui;

fn main() -> Result<(), String> {
    let args = parse_cli();
    check_game_attributes(args.num_of_players, args.num_of_cards)?;
    let mut game = Game::new(args.num_of_players, args.num_of_cards);
    game.start_game();
    Ok(())
}
