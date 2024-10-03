use crate::game::Game;

mod card;
mod deck;
mod default_deck;
mod game;
mod player;

fn main() {
    println!("Hello world!");
    let mut game = Game::new(3);
    game.start_game();
}
