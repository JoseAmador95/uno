use crate::{card, player};

/// Represents the possible actions a user can take.
#[derive(PartialEq, Debug)]
pub enum UserAction {
    Draw,
    Play(usize),
}

/// Defines the behavior of an actor in the game.
pub trait Actor {
    /// Determines the action the actor will take on their turn.
    ///
    /// Returns a `UserAction` representing the chosen action.
    fn get_turn_action(&mut self) -> UserAction;

    /// Determines the color choice for the actor.
    ///
    /// Returns a `card::Colour` representing the chosen color.
    fn get_color_choice(&mut self) -> card::Colour;

    /// Performs any actions required before the actor's turn.
    fn pre_turn_action(&mut self);

    /// Performs any actions required after the actor's turn.
    fn post_turn_action(&mut self);

    /// Retrieves a reference to the player associated with the actor.
    ///
    /// Returns a reference to a `player::Player`.
    fn get_player(&self) -> &player::Player;

    /// Retrieves a mutable reference to the player associated with the actor.
    ///
    /// Returns a mutable reference to a `player::Player`.
    fn get_player_mut(&mut self) -> &mut player::Player;

    /// Retrieves the ID of the actor.
    ///
    /// Returns a `usize` representing the actor's ID.
    fn get_id(&self) -> usize;
}
