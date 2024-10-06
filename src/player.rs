use crate::card::Card;
use crate::deck;

type PlayerResult<T> = Result<T, Error>;

pub enum Error {
    IndexOutOfBounds,
    DrawPileIsEmpty,
    Unknown,
}

pub struct Player {
    id: usize,
    hand: Vec<Card>,
}

impl Player {
    pub fn draw(&mut self, deck: &mut deck::Deck) -> PlayerResult<()> {
        match deck.draw() {
            Ok(c) => {
                self.hand.push(c);
                Ok(())
            }
            Err(deck::Error::DrawPileIsEmpty) => Err(Error::DrawPileIsEmpty),
            Err(_) => Err(Error::Unknown),
        }
    }

    pub fn play_card(&mut self, index: usize) -> PlayerResult<Card> {
        if index < self.hand.len() {
            return Ok(self.hand.remove(index));
        }

        Err(Error::IndexOutOfBounds)
    }

    pub fn get_card(&self, index: usize) -> PlayerResult<&Card> {
        if let Some(c) = self.hand.get(index) {
            return Ok(c);
        }

        Err(Error::IndexOutOfBounds)
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
