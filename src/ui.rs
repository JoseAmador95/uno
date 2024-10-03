use crate::{deck::Deck, player::Player};
use std::io;

pub enum UserAction {
    Draw,
    Play(usize),
}

pub fn get_game_context(player: &Player, deck: &Deck) {
    println!("Player {player}'s turn", player = player.get_id());
    println!(
        "Number of cards in the draw pile: {}",
        deck.number_of_cards_in_draw_pile()
    );
    if let Some(card) = deck.get_top_card() {
        println!("card on top: {card}");
    } else {
        print!("No card on top... somehow...")
    }
}

pub fn get_user_action() -> UserAction {
    let mut input = String::new();

    let index = loop {
        if io::stdin().read_line(&mut input).is_ok() {
            if let Ok(index) = input.trim().parse::<usize>() {
                break index;
            }
        }
        input.clear();
    };

    if index == 99 {
        UserAction::Draw
    } else {
        UserAction::Play(index)
    }
}

pub fn announce_winner(player: &Player) {
    println!("Player {id} wins!", id = player.get_id());
}
