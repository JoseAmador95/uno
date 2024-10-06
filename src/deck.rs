use crate::card::Card;
use crate::default_deck::GAME_DECK;
use rand::{seq::SliceRandom, thread_rng};
use std::collections::VecDeque;

type DeckResult<T> = Result<T, DeckError>;

pub enum DeckError {
    DrawPileIsEmpty,
    DiscardPileIsEmpty,
}

pub struct Deck {
    draw_pile: VecDeque<Card>,
    discard_pile: VecDeque<Card>,
}

impl Deck {
    pub fn draw(&mut self) -> DeckResult<Card> {
        if let Some(c) = self.draw_pile.pop_front() {
            return Ok(c);
        }

        Err(DeckError::DrawPileIsEmpty)
    }

    pub fn discard(&mut self, card: Card) {
        self.discard_pile.push_back(card);
    }

    fn shuffle(&mut self) {
        let mut rng = thread_rng();
        let mut card_vec: Vec<Card> = self.draw_pile.drain(..).collect();
        card_vec.shuffle(&mut rng);
        self.draw_pile = VecDeque::from(card_vec);
    }

    pub fn get_top_card(&self) -> DeckResult<&Card> {
        if let Some(c) = self.discard_pile.back() {
            return Ok(c);
        }

        Err(DeckError::DiscardPileIsEmpty)
    }

    pub fn refill_draw_pile(&mut self) -> DeckResult<()> {
        if let Some(c) = self.discard_pile.pop_back() {
            self.draw_pile.append(&mut self.discard_pile);
            self.discard_pile.clear();
            self.shuffle();
            self.discard(c);
            return Ok(());
        }

        Err(DeckError::DiscardPileIsEmpty)
    }

    fn discard_from_draw_pile(&mut self) -> DeckResult<()> {
        let card = self.draw()?;
        self.discard(card);
        Ok(())
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
        if deck.discard_from_draw_pile().is_err() {
            panic!("Error while creating the game decks")
        }
        deck
    }
}
