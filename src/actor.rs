use crate::{card, player};

#[derive(PartialEq, Debug)]
pub enum UserAction {
    Draw,
    Play(usize),
}

pub trait Actor {
    fn get_turn_action(&mut self) -> UserAction;
    fn get_color_choice(&mut self) -> card::Colour;
    fn pre_turn_action(&mut self);
    fn post_turn_action(&mut self);
    fn get_player(&self) -> &player::Player;
    fn get_player_mut(&mut self) -> &mut player::Player;
    fn get_id(&self) -> usize;
}
