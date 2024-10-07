use cli::parse_input;
use game::check_game_attributes;

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
    check_game_attributes(args.num_of_players, args.num_of_cards)?;
    let mut flow = flow::GameFlow::new(args.num_of_players, args.num_of_cards);
    flow.start_game();
    Ok(())
}
