use crate::card::{Card, CardValue};
use crate::deck::Deck;
use crate::player::Player;
use std::io;

pub struct Game {
    players: Vec<Player>,
    deck: Deck,
    player_index: usize,
    is_direction_ascending: bool,
}

impl Game {
    fn player_turn(&mut self, player_index: usize, card: &Card) {
        match card.value {
            CardValue::Number(_) => {}
            CardValue::DrawTwo => {
                self.players[player_index].draw(&mut self.deck);
                self.players[player_index].draw(&mut self.deck);
            }
            CardValue::Skip => self.next_player(),
            CardValue::Reverse => self.revese_direction(),
        };
        self.deck.discard(*card);
    }

    fn next_player(&mut self) {
        if !self.is_direction_ascending && self.player_index == 0 {
            self.player_index = self.players.len() - 1;
        } else {
            let index_increment: isize = if self.is_direction_ascending { 1 } else { -1 };
            self.player_index =
                (self.player_index.wrapping_add_signed(index_increment)) % self.players.len();
        }
    }

    fn revese_direction(&mut self) {
        self.is_direction_ascending = !self.is_direction_ascending;
    }

    fn is_valid_play(&self, card: Card) -> bool {
        if let Some(card_on_top) = self.deck.get_top_card() {
            card.colour == card_on_top.colour || card.value == card_on_top.value
        } else {
            // There is no card on top of the discard pile (for some reason)
            // So might as well play whatever the player wants
            true
        }
    }

    pub fn new(num_of_players: usize) -> Self {
        let players = (0..num_of_players).map(Player::new).collect();
        Game {
            players,
            deck: Deck::new(),
            player_index: 0,
            is_direction_ascending: true,
        }
    }

    pub fn start_game(&mut self) {
        let num_of_players = self.players.len();
        for i in 0..num_of_players {
            for _ in 0..7 {
                self.players[i].draw(&mut self.deck);
            }
        }

        let winner = loop {
            println!("Player {player}'s turn", player = self.player_index);
            println!(
                "Number of cards in the draw pile: {}",
                self.deck.number_of_cards_in_draw_pile()
            );
            if let Some(card) = self.deck.get_top_card() {
                println!("card on top: {card}");
            } else {
                print!("No card on top... somehow...")
            }
            self.players[self.player_index].print_hand();
            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");
            let mut index: usize = input.trim().parse().expect("Failed to parse number");

            let player_draws = index == 99;

            if player_draws {
                self.players[self.player_index].draw(&mut self.deck);
            } else {
                let card = loop {
                    if let Ok(card) = self.players[self.player_index].play_card(index) {
                        break card;
                    } else {
                        let mut input = String::new();
                        io::stdin()
                            .read_line(&mut input)
                            .expect("Failed to read line");
                        index = input.trim().parse().expect("Failed to parse number");
                    }
                };
                let is_valid_play = self.is_valid_play(card);
                if is_valid_play {
                    self.player_turn(self.player_index, &card);
                }
                if self.players[self.player_index].is_hand_empty() {
                    println!("Game over");
                    break self.player_index;
                }
                if self.deck.draw_pile_is_empty() {
                    self.deck.refill_draw_pile();
                }
            }

            self.next_player();
        };

        println!("Player {winner} wins!");
    }
}
