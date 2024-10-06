use crate::{card::Colour, deck::Deck, player::Player};
use std::io;

const DRAW_INDEX: &str = "d";

pub enum UserAction {
    Draw,
    Play(usize),
}

fn clear_terminal() {
    print!("{}[2J", 27 as char); // ANSI escape code to clear the terminal
    print!("{}[H", 27 as char); // Move the cursor to the top-left corner
}

pub fn get_game_context(player: &Player, deck: &Deck) {
    clear_terminal();
    println!("Player {player}'s turn", player = player.get_id());
    println!(
        "Number of cards in the draw pile: {}",
        deck.number_of_cards_in_draw_pile()
    );
    if let Ok(card) = deck.get_top_card() {
        println!("card on top: {card}");
    } else {
        print!("No card on top... somehow...")
    }
    player.print_hand();
    println!("{DRAW_INDEX:02}: Draw card")
}

pub fn get_user_turn_action() -> UserAction {
    let mut input = String::new();

    loop {
        if io::stdin().read_line(&mut input).is_ok() {
            if let Ok(index) = input.trim().parse::<usize>() {
                return UserAction::Play(index);
            } else if let Ok(str) = input.trim().parse::<String>() {
                if str == DRAW_INDEX {
                    return UserAction::Draw;
                }
            }
        }
        input.clear();
    }
}

pub fn announce_winner(player: &Player) {
    println!("Player {id} wins!", id = player.get_id());
}

pub fn get_user_wild_colour() -> Colour {
    let mut input = String::new();

    println!("r: Red");
    println!("g: Green");
    println!("b: Blue");
    println!("y: Yellow");

    loop {
        if io::stdin().read_line(&mut input).is_ok() {
            if let Ok(str) = input.trim().parse::<String>() {
                let colour = match str.as_str() {
                    "r" => Ok(Colour::Red),
                    "g" => Ok(Colour::Green),
                    "b" => Ok(Colour::Blue),
                    "y" => Ok(Colour::Yellow),
                    _ => Err(()),
                };

                if let Ok(c) = colour {
                    return c;
                }
            }
        }
        input.clear();
    }
}
