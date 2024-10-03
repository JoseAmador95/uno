use crate::card::{Card, CardValue};
use crate::deck::Deck;
use crate::player::Player;
use crate::ui::{announce_winner, get_game_context, get_user_action, UserAction};

pub struct Game {
    players: Vec<Player>,
    deck: Deck,
    player_index: usize,
    is_direction_ascending: bool,
}

impl Game {
    fn action_draw_two(&mut self, player_index: usize) {
        self.players[player_index].draw(&mut self.deck);
        self.players[player_index].draw(&mut self.deck);
    }

    fn player_turn(&mut self, player_index: usize, card: &Card) {
        match card.value {
            CardValue::Number(_) => {}
            CardValue::DrawTwo => self.action_draw_two(player_index),
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
            get_game_context(&self.players[self.player_index], &self.deck);
            self.players[self.player_index].print_hand();

            loop {
                match get_user_action() {
                    UserAction::Draw => {
                        self.players[self.player_index].draw(&mut self.deck);
                        break;
                    }
                    UserAction::Play(index) => {
                        if let Ok(card) = self.players[self.player_index].play_card(index) {
                            if self.is_valid_play(card) {
                                self.player_turn(self.player_index, &card);
                                break;
                            }
                        }
                    }
                }
            }

            if self.players[self.player_index].is_hand_empty() {
                break self.player_index;
            }

            if self.deck.draw_pile_is_empty() {
                self.deck.refill_draw_pile();
            }

            self.next_player();
        };

        announce_winner(&self.players[winner]);
    }
}
