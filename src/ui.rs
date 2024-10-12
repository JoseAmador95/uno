use crate::{actor, card, deck, game, player};
use core::str;
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
            if let Ok(index) = input.trim().parse::<usize>() {
                return actor::UserAction::Play(index);
            } else if let Ok(str) = input.trim().parse::<String>() {
                if str == DRAW_INDEX {
                    return actor::UserAction::Draw;
                }
            }
        }
        input.clear();
    }
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
                let colour = match str.as_str() {
                    "r" => Ok(card::Colour::Red),
                    "g" => Ok(card::Colour::Green),
                    "b" => Ok(card::Colour::Blue),
                    "y" => Ok(card::Colour::Yellow),
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
