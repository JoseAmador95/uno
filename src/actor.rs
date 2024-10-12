use crate::{card, game};

pub enum UserAction {
    Draw,
    Play(usize),
}

pub trait Actor {
    fn get_turn_action(&mut self, game: &game::Game) -> UserAction;
    fn get_color_choice(&mut self, game: &game::Game) -> card::Colour;
    fn pre_turn_action(&mut self, game: &game::Game);
    fn post_turn_action(&mut self, game: &game::Game);
}
