use crate::{actor, card, deck, game, player};
use std::io;

const DRAW_INDEX: &str = "d";

pub struct HumanActor {
    player_index: usize,
}

impl actor::Actor for HumanActor {
    fn get_turn_action(&mut self, _game: &game::Game) -> actor::UserAction {
        get_user_turn_action()
    }

    fn get_color_choice(&mut self, _game: &game::Game) -> card::Colour {
        get_user_wild_colour()
    }

    fn pre_turn_action(&mut self, game: &game::Game) {
        get_game_context(game.get_player(self.player_index), game.get_deck());
    }

    fn post_turn_action(&mut self, _game: &game::Game) {
        // Do nothing
    }
}

impl HumanActor {
    pub fn new(player_index: usize) -> HumanActor {
        HumanActor { player_index }
    }
}

fn clear_terminal() {
    print!("{}[2J", 27 as char); // ANSI escape code to clear the terminal
    print!("{}[H", 27 as char); // Move the cursor to the top-left corner
}

pub fn get_game_context(player: &player::Player, deck: &impl deck::DeckTrait) {
    clear_terminal();
    println!("Player {player}'s turn", player = player.get_id());
    println!(
        "Number of cards in the draw pile: {}",
        deck.number_of_cards_in_draw_pile()
    );
    if let Ok(card) = deck.get_top_card() {
        println!("card on top: {card}");
    } else {
        print!("No card on top... somehow...");
    }
    player.print_hand();
    println!("{DRAW_INDEX:02}: Draw card");
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
        if str == DRAW_INDEX {
            return Ok(actor::UserAction::Draw);
        }
    }

    Err(())
}

pub fn announce_winner(player: &player::Player) {
    println!("Player {id} wins!", id = player.get_id());
}

pub fn get_user_wild_colour() -> card::Colour {
    let mut input = String::new();

    println!("r: Red");
    println!("g: Green");
    println!("b: Blue");
    println!("y: Yellow");

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
        "r" => Ok(card::Colour::Red),
        "g" => Ok(card::Colour::Green),
        "b" => Ok(card::Colour::Blue),
        "y" => Ok(card::Colour::Yellow),
        _ => Err(()),
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_check_turn_action_input_ok_index() {
        assert_eq!(check_turn_action_input("1"), Ok(actor::UserAction::Play(1)));
    }

    #[test]
    fn test_check_turn_action_input_ok_draw() {
        assert_eq!(check_turn_action_input("d"), Ok(actor::UserAction::Draw));
        assert_eq!(check_turn_action_input("a"), Err(()));
    }

    #[test]
    fn test_check_turn_action_input_err() {
        assert_eq!(check_turn_action_input("a"), Err(()));
    }

    #[test]
    fn test_check_colour_input_ok() {
        assert_eq!(check_colour_input("r"), Ok(card::Colour::Red));
        assert_eq!(check_colour_input("g"), Ok(card::Colour::Green));
        assert_eq!(check_colour_input("b"), Ok(card::Colour::Blue));
        assert_eq!(check_colour_input("y"), Ok(card::Colour::Yellow));
    }

    #[test]
    fn test_check_colour_input_err() {
        assert_eq!(check_colour_input("a"), Err(()));
    }
}
