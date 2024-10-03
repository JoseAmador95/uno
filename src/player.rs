use crate::card::Card;
use crate::deck::Deck;

pub struct Player {
    id: usize,
    hand: Vec<Card>,
}

impl Player {
    pub fn draw(&mut self, deck: &mut Deck) {
        if let Some(card) = deck.draw() {
            self.hand.push(card);
        }
    }

    pub fn play_card(&mut self, index: usize) -> Result<Card, &str> {
        if index < self.hand.len() {
            Ok(self.hand.remove(index))
        } else {
            Err("Index out of bounds")
        }
    }

    pub fn is_hand_empty(&self) -> bool {
        self.hand.is_empty()
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
