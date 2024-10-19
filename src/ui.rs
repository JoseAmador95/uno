use crate::{actor, card, deck::DeckTrait, player};
use std::io;

const DRAW: &str = "d";
const R: &str = "r";
const G: &str = "g";
const B: &str = "b";
const Y: &str = "y";

pub struct HumanActor {
    id: usize,
    player: player::Player,
}

impl actor::Actor for HumanActor {
    fn get_id(&self) -> usize {
        self.id
    }
    fn get_player(&self) -> &player::Player {
        &self.player
    }

    fn get_player_mut(&mut self) -> &mut player::Player {
        &mut self.player
    }

    fn get_turn_action(&mut self) -> actor::UserAction {
        get_user_turn_action()
    }

    fn get_color_choice(&mut self) -> card::Colour {
        get_user_wild_colour()
    }

    fn pre_turn_action(&mut self) {
        print_player_context(&self.player);
    }

    fn post_turn_action(&mut self) {
        // Do nothing
    }
}

impl HumanActor {
    pub fn new(id: usize) -> HumanActor {
        HumanActor {
            id,
            player: player::Player::new(),
        }
    }
}

fn clear_terminal() {
    print!("{}[2J", 27 as char); // ANSI escape code to clear the terminal
    print!("{}[H", 27 as char); // Move the cursor to the top-left corner
}

pub fn get_game_context(player_index: usize, deck: &impl DeckTrait) {
    clear_terminal();
    println!("Player {player_index}'s turn");
    println!(
        "Number of cards in the draw pile: {}",
        deck.number_of_cards_in_draw_pile()
    );
    if let Ok(card) = deck.get_top_card() {
        println!("card on top: {card}");
    } else {
        print!("No card on top... somehow...");
    }
}

fn print_player_context(player: &player::Player) {
    player.print_hand();
    println!("{DRAW:02}: Draw card");
}

pub fn get_user_turn_action() -> actor::UserAction {
    let mut input = String::new();

    loop {
        if io::stdin().read_line(&mut input).is_ok() {
            if let Ok(action) = check_turn_action_input(&input) {
                return action;
            }
        }
        input.clear();
    }
}

fn check_turn_action_input(input: &str) -> Result<actor::UserAction, ()> {
    if let Ok(index) = input.trim().parse::<usize>() {
        return Ok(actor::UserAction::Play(index));
    } else if let Ok(str) = input.trim().parse::<String>() {
        if str == DRAW {
            return Ok(actor::UserAction::Draw);
        }
    }

    Err(())
}

pub fn announce_winner(id: usize) {
    println!("Player {id} wins!");
}

pub fn get_user_wild_colour() -> card::Colour {
    let mut input = String::new();

    println!("{R}: Red");
    println!("{G}: Green");
    println!("{B}: Blue");
    println!("{Y}: Yellow");

    loop {
        if io::stdin().read_line(&mut input).is_ok() {
            if let Ok(str) = input.trim().parse::<String>() {
                if let Ok(c) = check_colour_input(&str) {
                    return c;
                }
            }
        }
        input.clear();
    }
}

fn check_colour_input(input: &str) -> Result<card::Colour, ()> {
    match input {
        R => Ok(card::Colour::Red),
        G => Ok(card::Colour::Green),
        B => Ok(card::Colour::Blue),
        Y => Ok(card::Colour::Yellow),
        _ => Err(()),
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_check_turn_action_input_ok_index() {
        assert_eq!(check_turn_action_input("1"), Ok(actor::UserAction::Play(1)));
        assert_eq!(
            check_turn_action_input("001"),
            Ok(actor::UserAction::Play(1))
        );
        assert_eq!(
            check_turn_action_input(" 1 "),
            Ok(actor::UserAction::Play(1))
        );
    }

    #[test]
    fn test_check_turn_action_input_ok_draw() {
        assert_eq!(check_turn_action_input(DRAW), Ok(actor::UserAction::Draw));
    }

    #[test]
    fn test_check_turn_action_input_err() {
        assert_eq!(check_turn_action_input("a"), Err(()));
    }

    #[test]
    fn test_check_colour_input_ok() {
        assert_eq!(check_colour_input(R), Ok(card::Colour::Red));
        assert_eq!(check_colour_input(G), Ok(card::Colour::Green));
        assert_eq!(check_colour_input(B), Ok(card::Colour::Blue));
        assert_eq!(check_colour_input(Y), Ok(card::Colour::Yellow));
    }

    #[test]
    fn test_check_colour_input_err() {
        assert_eq!(check_colour_input("a"), Err(()));
    }
}
