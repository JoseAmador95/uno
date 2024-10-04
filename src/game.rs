use crate::card::{Card, CardValue};
use crate::deck::Deck;
use crate::player::Player;
use crate::ui::{announce_winner, get_game_context, get_user_turn_action, UserAction};

pub struct Game {
    players: Vec<Player>,
    deck: Deck,
    player_index: usize,
    is_direction_ascending: bool,
}

impl Game {
    fn action_draw_two(&mut self, next_player_index: usize) {
        self.players[next_player_index].draw(&mut self.deck);
        self.players[next_player_index].draw(&mut self.deck);
    }

    fn player_turn(&mut self, player_index: usize, card: &Card) {
        match card.value {
            CardValue::Number(_) => {}
            CardValue::DrawTwo => self.action_draw_two(self.get_next_player(player_index)),
            CardValue::Skip => self.set_next_player(),
            CardValue::Reverse => self.revese_direction(),
        };
        self.deck.discard(*card);
    }

    fn get_next_player(&self, current_player_index: usize) -> usize {
        if !self.is_direction_ascending && current_player_index == 0 {
            self.players.len() - 1
        } else {
            let index_increment: isize = if self.is_direction_ascending { 1 } else { -1 };
            (current_player_index.wrapping_add_signed(index_increment)) % self.players.len()
        }
    }

    fn set_next_player(&mut self) {
        self.player_index = self.get_next_player(self.player_index);
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

    fn play_turn(&mut self, player_index: usize) {
        loop {
            match get_user_turn_action() {
                UserAction::Draw => {
                    self.players[player_index].draw(&mut self.deck);
                    break;
                }
                UserAction::Play(index) => {
                    if let Ok(card) = self.players[player_index].play_card(index) {
                        if self.is_valid_play(card) {
                            self.player_turn(player_index, &card);
                            break;
                        }
                    }
                }
            }
        }
    }

    fn deal_cards_to_players(&mut self, num_of_cards: usize) {
        let num_of_players = self.players.len();
        for i in 0..num_of_players {
            for _ in 0..num_of_cards {
                self.players[i].draw(&mut self.deck);
            }
        }
    }

    fn has_player_won(&self, player_index: usize) -> bool {
        self.players[player_index].is_hand_empty()
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
        self.deal_cards_to_players(7);

        let winner = loop {
            get_game_context(&self.players[self.player_index], &self.deck);

            self.play_turn(self.player_index);

            if self.has_player_won(self.player_index) {
                break self.player_index;
            }

            if self.deck.draw_pile_is_empty() {
                self.deck.refill_draw_pile();
            }

            self.set_next_player();
        };

        announce_winner(&self.players[winner]);
    }
}
