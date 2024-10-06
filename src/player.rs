use crate::card::Card;
use crate::deck::{Deck, DeckError};

type PlayerResult<T> = Result<T, PlayerError>;

pub enum PlayerError {
    IndexOutOfBounds,
    DrawPileIsEmpty,
    Unknown,
}

pub struct Player {
    id: usize,
    hand: Vec<Card>,
}

impl Player {
    pub fn draw(&mut self, deck: &mut Deck) -> PlayerResult<()> {
        match deck.draw() {
            Ok(c) => {
                self.hand.push(c);
                Ok(())
            }
            Err(DeckError::DrawPileIsEmpty) => Err(PlayerError::DrawPileIsEmpty),
            Err(_) => Err(PlayerError::Unknown),
        }
    }

    pub fn play_card(&mut self, index: usize) -> PlayerResult<Card> {
        if index < self.hand.len() {
            return Ok(self.hand.remove(index));
        }

        Err(PlayerError::IndexOutOfBounds)
    }

    pub fn is_hand_empty(&self) -> bool {
        self.hand.is_empty()
    }

    pub fn get_id(&self) -> usize {
        self.id
    }

    pub fn print_hand(&self) {
        let _ = self
            .hand
            .iter()
            .enumerate()
            .map(|(i, card)| println!("{i:02}: {card}"))
            .collect::<Vec<_>>();
    }

    pub fn new(id: usize) -> Self {
        Player {
            id,
            hand: Vec::new(),
        }
    }
}
