use crate::card::Card;
use crate::default_deck::GAME_DECK;
use rand::{seq::SliceRandom, thread_rng};
use std::collections::VecDeque;

pub struct Deck {
    draw_pile: VecDeque<Card>,
    discard_pile: VecDeque<Card>,
}

impl Deck {
    pub fn draw(&mut self) -> Option<Card> {
        self.draw_pile.pop_front()
    }

    pub fn discard(&mut self, card: Card) {
        self.discard_pile.push_back(card);
    }

    pub fn shuffle(&mut self) {
        let mut rng = thread_rng();
        let mut card_vec: Vec<Card> = self.draw_pile.drain(..).collect();
        card_vec.shuffle(&mut rng);
        self.draw_pile = VecDeque::from(card_vec);
    }

    pub fn get_top_card(&self) -> Option<&Card> {
        self.discard_pile.back()
    }

    pub fn draw_pile_is_empty(&self) -> bool {
        self.draw_pile.is_empty()
    }

    pub fn refill_draw_pile(&mut self) {
        assert!(
            !self.draw_pile_is_empty() || !self.discard_pile.is_empty(),
            "Both discard and draw piles are empty"
        );
        let top_card = self.discard_pile.pop_back();
        self.draw_pile.append(&mut self.discard_pile);
        self.discard_pile.clear();
        self.shuffle();

        if let Some(c) = top_card {
            self.discard(c);
        } else if !self.draw_pile_is_empty() {
            self.discard_from_draw_pile();
        }
    }

    pub fn discard_from_draw_pile(&mut self) {
        if let Some(card) = self.draw() {
            self.discard(card);
        }
    }

    pub fn number_of_cards_in_draw_pile(&self) -> usize {
        self.draw_pile.len()
    }

    pub fn new() -> Self {
        let mut deck = Deck {
            draw_pile: VecDeque::from(GAME_DECK.to_vec()),
            discard_pile: VecDeque::new(),
        };
        deck.shuffle();
        deck.discard_from_draw_pile();
        deck
    }
}
