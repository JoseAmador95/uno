use crate::card::Card;
use crate::default_deck::GAME_DECK;
use rand::{seq::SliceRandom, thread_rng};

pub struct Deck {
    draw_pile: Vec<Card>,
    discard_pile: Vec<Card>,
}

impl Deck {
    pub fn draw(&mut self) -> Option<Card> {
        self.draw_pile.pop()
    }

    pub fn discard(&mut self, card: Card) {
        self.discard_pile.push(card);
    }

    pub fn shuffle(&mut self) {
        let mut rng = thread_rng();
        self.draw_pile.shuffle(&mut rng);
    }

    pub fn get_top_card(&self) -> Option<&Card> {
        self.discard_pile.last()
    }

    pub fn draw_pile_is_empty(&self) -> bool {
        self.draw_pile.is_empty()
    }

    pub fn refill_draw_pile(&mut self) {
        self.draw_pile.append(&mut self.discard_pile);
        self.discard_pile.clear();
        self.shuffle();
        self.discard_from_draw_pile();
    }

    pub fn discard_from_draw_pile(&mut self) {
        if let Some(card) = self.draw_pile.pop() {
            self.discard(card);
        }
    }

    pub fn number_of_cards_in_draw_pile(&self) -> usize {
        self.draw_pile.len()
    }

    pub fn new() -> Self {
        let mut deck = Deck {
            draw_pile: GAME_DECK.to_vec(),
            discard_pile: Vec::new(),
        };
        deck.shuffle();
        deck.discard_from_draw_pile();
        deck
    }
}
