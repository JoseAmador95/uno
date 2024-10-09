use crate::card;
use crate::default_deck::GAME_DECK;
use rand::{seq::SliceRandom, thread_rng};
use std::collections::VecDeque;

type DeckResult<T> = Result<T, Error>;

pub enum Error {
    DrawPileIsEmpty,
    DiscardPileIsEmpty,
}

pub struct Deck {
    draw_pile: VecDeque<card::Card>,
    discard_pile: VecDeque<card::Card>,
}

impl Deck {
    pub fn draw(&mut self) -> DeckResult<card::Card> {
        if let Some(c) = self.draw_pile.pop_front() {
            return Ok(c);
        }

        Err(Error::DrawPileIsEmpty)
    }

    pub fn discard(&mut self, card: card::Card) {
        self.discard_pile.push_back(card);
    }

    fn shuffle(&mut self) {
        let mut rng = thread_rng();
        let mut card_vec: Vec<card::Card> = self.draw_pile.drain(..).collect();
        card_vec.shuffle(&mut rng);
        self.draw_pile = VecDeque::from(card_vec);
    }

    pub fn get_top_card(&self) -> DeckResult<&card::Card> {
        if let Some(c) = self.discard_pile.back() {
            return Ok(c);
        }

        Err(Error::DiscardPileIsEmpty)
    }

    pub fn refill_draw_pile(&mut self) -> DeckResult<()> {
        if let Some(card) = self.discard_pile.pop_back() {
            self.rever_wild_cards_in_discard_pile();
            self.draw_pile.append(&mut self.discard_pile);
            self.discard_pile.clear();
            self.shuffle();
            self.discard(card);
            return Ok(());
        }

        Err(Error::DiscardPileIsEmpty)
    }

    fn rever_wild_cards_in_discard_pile(&mut self) {
        let _ = self
            .discard_pile
            .iter_mut()
            .filter(|card| matches!(card.value, card::Value::Wild | card::Value::WildDraw(_)))
            .map(|card| card.colour = card::Colour::Wild)
            .collect::<Vec<_>>();
    }

    fn discard_from_draw_pile(&mut self) -> DeckResult<()> {
        let card = self.draw()?;
        self.discard(card);
        Ok(())
    }

    pub fn number_of_cards_in_draw_pile(&self) -> usize {
        self.draw_pile.len()
    }

    pub fn change_colour_of_top_card_in_discard(&mut self, colour: &card::Colour) {
        if let Some(c) = self.discard_pile.back_mut() {
            c.colour = *colour;
        }
    }

    pub fn new() -> Self {
        let mut deck = Deck {
            draw_pile: VecDeque::from(GAME_DECK.to_vec()),
            discard_pile: VecDeque::new(),
        };
        deck.shuffle();
        assert!(
            deck.discard_from_draw_pile().is_ok(),
            "Error while creating the game decks"
        );
        deck
    }
}
