use crate::card;
use crate::default_deck::GAME_DECK;
use mockall::automock;
use rand::{seq::SliceRandom, thread_rng};
use std::collections::VecDeque;

type DeckResult<T> = Result<T, Error>;

#[derive(Debug, PartialEq)]
pub enum Error {
    DrawPileIsEmpty,
    DiscardPileIsEmpty,
}

pub struct Deck {
    draw_pile: VecDeque<card::Card>,
    discard_pile: VecDeque<card::Card>,
}

#[automock]
pub trait DeckTrait {
    fn draw(&mut self) -> DeckResult<card::Card>;
    fn discard(&mut self, card: card::Card);
    fn get_top_card<'a>(&'a self) -> DeckResult<&'a card::Card>;
    fn refill_draw_pile(&mut self) -> DeckResult<()>;
    fn number_of_cards_in_draw_pile(&self) -> usize;
    fn change_colour_of_top_card_in_discard(&mut self, colour: &card::Colour);
    fn new() -> Self;
}

impl DeckTrait for Deck {
    fn draw(&mut self) -> DeckResult<card::Card> {
        if let Some(c) = self.draw_pile.pop_front() {
            return Ok(c);
        }

        Err(Error::DrawPileIsEmpty)
    }

    fn discard(&mut self, card: card::Card) {
        self.discard_pile.push_back(card);
    }
    fn refill_draw_pile(&mut self) -> DeckResult<()> {
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
    fn get_top_card<'a>(&'a self) -> DeckResult<&'a card::Card> {
        if let Some(c) = self.discard_pile.back() {
            return Ok(c);
        }

        Err(Error::DiscardPileIsEmpty)
    }
    fn number_of_cards_in_draw_pile(&self) -> usize {
        self.draw_pile.len()
    }

    fn change_colour_of_top_card_in_discard(&mut self, colour: &card::Colour) {
        if let Some(c) = self.discard_pile.back_mut() {
            c.colour = *colour;
        }
    }

    fn new() -> Self {
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

impl Deck {
    fn shuffle(&mut self) {
        let mut rng = thread_rng();
        let mut card_vec: Vec<card::Card> = self.draw_pile.drain(..).collect();
        card_vec.shuffle(&mut rng);
        self.draw_pile = VecDeque::from(card_vec);
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
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::card::{Card, Colour, Value};

    fn create_test_deck() -> Deck {
        Deck {
            draw_pile: VecDeque::new(),
            discard_pile: VecDeque::new(),
        }
    }

    #[test]
    fn test_draw_card_success() {
        let card = Card {
            colour: Colour::Red,
            value: Value::Number(5),
        };
        let mut deck = create_test_deck();
        deck.draw_pile.push_back(card);

        let result = deck.draw();
        assert_eq!(result, DeckResult::Ok(card));
    }

    #[test]
    fn test_draw_card_empty_draw_pile() {
        let mut deck = create_test_deck();
        assert_eq!(deck.draw().unwrap_err(), Error::DrawPileIsEmpty);
    }

    #[test]
    fn test_discard_card() {
        let mut deck = create_test_deck();
        let card = Card {
            colour: Colour::Blue,
            value: Value::Skip,
        };
        deck.discard(card);
        assert_eq!(deck.discard_pile.back().unwrap(), &card);
    }

    #[test]
    fn test_refill_draw_pile_success() {
        let mut deck = create_test_deck();
        let card1 = Card {
            colour: Colour::Green,
            value: Value::Reverse,
        };
        let card2 = Card {
            colour: Colour::Green,
            value: Value::Reverse,
        };
        deck.discard(card1);
        deck.discard(card2);
        deck.refill_draw_pile().unwrap();
        assert_eq!(deck.draw_pile.len(), 1);
        assert_eq!(deck.discard_pile.len(), 1);
        assert_eq!(deck.discard_pile.back().unwrap(), &card2);
    }

    #[test]
    fn test_refill_draw_pile_single_card() {
        let mut deck = create_test_deck();
        let card = Card {
            colour: Colour::Green,
            value: Value::Reverse,
        };
        deck.discard(card);
        deck.refill_draw_pile().unwrap();
        assert_eq!(deck.draw_pile.len(), 0);
        assert_eq!(deck.discard_pile.len(), 1);
        assert_eq!(deck.discard_pile.back().unwrap(), &card);
    }

    #[test]
    fn test_refill_draw_pile_empty_discard_pile() {
        let mut deck = create_test_deck();
        assert_eq!(
            deck.refill_draw_pile().unwrap_err(),
            Error::DiscardPileIsEmpty
        );
    }

    #[test]
    fn test_get_top_card_success() {
        let mut deck = create_test_deck();
        let card = Card {
            colour: Colour::Yellow,
            value: Value::DrawTwo,
        };
        deck.discard(card);
        let top_card = deck.get_top_card().unwrap();
        assert_eq!(top_card, &card);
    }

    #[test]
    fn test_get_top_card_empty_discard_pile() {
        let deck = create_test_deck();
        assert_eq!(deck.get_top_card().unwrap_err(), Error::DiscardPileIsEmpty);
    }

    #[test]
    fn test_number_of_cards_in_draw_pile() {
        let mut deck = create_test_deck();
        assert_eq!(deck.number_of_cards_in_draw_pile(), 0);
        deck.draw_pile.push_back(Card {
            colour: Colour::Red,
            value: Value::Number(3),
        });
        assert_eq!(deck.number_of_cards_in_draw_pile(), 1);
    }

    #[test]
    fn test_change_colour_of_top_card_in_discard() {
        let mut deck = create_test_deck();
        let card = Card {
            colour: Colour::Wild,
            value: Value::Wild,
        };
        deck.discard(card);
        deck.change_colour_of_top_card_in_discard(&Colour::Blue);
        assert_eq!(deck.discard_pile.back().unwrap().colour, Colour::Blue);
    }

    #[test]
    fn test_new_deck() {
        let deck = Deck::new();
        assert!(!deck.draw_pile.is_empty());
        assert_eq!(deck.discard_pile.len(), 1);
    }

    #[test]
    fn test_shuffle_deck() {
        let mut deck = create_test_deck();
        let card1 = Card {
            colour: Colour::Red,
            value: Value::Number(1),
        };
        let card2 = Card {
            colour: Colour::Blue,
            value: Value::Number(2),
        };
        deck.draw_pile.push_back(card1);
        deck.draw_pile.push_back(card2);
        deck.shuffle();
        assert!(deck.draw_pile.contains(&card1));
        assert!(deck.draw_pile.contains(&card2));
    }

    #[test]
    fn test_rever_wild_cards_in_discard_pile() {
        let mut deck = create_test_deck();
        let card = Card {
            colour: Colour::Blue,
            value: Value::Wild,
        };
        deck.discard(card);
        deck.rever_wild_cards_in_discard_pile();
        assert_eq!(deck.discard_pile.back().unwrap().colour, Colour::Wild);
    }

    #[test]
    fn test_discard_from_draw_pile_success() {
        let mut deck = create_test_deck();
        let card = Card {
            colour: Colour::Red,
            value: Value::Number(5),
        };
        deck.draw_pile.push_back(card);
        deck.discard_from_draw_pile().unwrap();
        assert_eq!(deck.discard_pile.back().unwrap(), &card);
    }

    #[test]
    fn test_discard_from_draw_pile_empty_draw_pile() {
        let mut deck = create_test_deck();
        assert_eq!(
            deck.discard_from_draw_pile().unwrap_err(),
            Error::DrawPileIsEmpty
        );
    }
}
