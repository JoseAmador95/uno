use crate::card::Card;
use crate::deck;

type PlayerResult<T> = Result<T, Error>;

#[derive(Debug, PartialEq)]
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
    pub fn draw(&mut self, deck: &mut impl deck::DeckTrait) -> PlayerResult<()> {
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
        if index < self.get_number_of_cards() {
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

    pub fn get_number_of_cards(&self) -> usize {
        self.hand.len()
    }

    pub fn get_hand(&self) -> &Vec<Card> {
        &self.hand
    }

    fn hand_to_string(&self) -> String {
        self.hand
            .iter()
            .enumerate()
            .map(|(i, card)| format!("{i:02}: {card}"))
            .collect::<Vec<_>>()
            .join("\n")
    }

    pub fn print_hand(&self) {
        println!("{}", self.hand_to_string());
    }

    pub fn new(id: usize) -> Self {
        Player {
            id,
            hand: Vec::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::card;

    #[test]
    fn test_draw_ok() {
        let mut player = Player::new(0);
        let mut deck = deck::MockDeckTrait::default();
        deck.expect_draw().returning(|| {
            Result::Ok(card::Card {
                colour: card::Colour::Red,
                value: card::Value::Number(1),
            })
        });
        let result = player.draw(&mut deck);
        assert_eq!(result, Ok(()));
    }

    #[test]
    fn test_draw_fail() {
        let mut player = Player::new(0);
        let mut deck = deck::MockDeckTrait::default();
        deck.expect_draw()
            .returning(|| Err(deck::Error::DrawPileIsEmpty));
        let result = player.draw(&mut deck);
        assert_eq!(result, Err(Error::DrawPileIsEmpty));
    }

    #[test]
    fn test_play_card_ok() {
        let mut player = Player::new(0);
        player.hand.push(card::Card {
            colour: card::Colour::Red,
            value: card::Value::Number(1),
        });
        let result = player.play_card(0);
        assert!(result.is_ok());
        assert!(player.hand.is_empty());
    }

    #[test]
    fn test_play_card_fail() {
        let mut player = Player::new(0);
        let result = player.play_card(1);
        assert_eq!(result, Err(Error::IndexOutOfBounds));
    }

    #[test]
    fn test_is_hand_empty_true() {
        let player = Player::new(0);
        assert!(player.is_hand_empty());
    }

    #[test]
    fn test_is_hand_empty_false() {
        let mut player = Player::new(0);
        player.hand.push(card::Card {
            colour: card::Colour::Red,
            value: card::Value::Number(1),
        });
        assert!(!player.is_hand_empty());
    }

    #[test]
    fn test_get_number_of_cards() {
        let mut player = Player::new(0);
        assert_eq!(player.get_number_of_cards(), 0);
        player.hand.push(card::Card {
            colour: card::Colour::Red,
            value: card::Value::Number(1),
        });
        assert_eq!(player.get_number_of_cards(), 1);
    }

    #[test]
    fn test_get_id() {
        let id = 10;
        let player = Player::new(id);
        assert_eq!(player.get_id(), id);
    }

    #[test]
    fn test_get_hand_ok() {
        let card = card::Card {
            colour: card::Colour::Red,
            value: card::Value::Number(1),
        };
        let mut player = Player::new(0);
        player.hand.push(card);
        assert_eq!(player.get_hand(), &vec![card]);
    }

    #[test]
    fn test_get_card_ok() {
        let card = card::Card {
            colour: card::Colour::Red,
            value: card::Value::Number(1),
        };
        let mut player = Player::new(0);
        player.hand.push(card);
        let result = player.get_card(0);
        assert_eq!(result, Ok(&card));
    }

    #[test]
    fn test_get_card_index_out_of_bounds() {
        let player = Player::new(0);
        let result = player.get_card(0);
        assert_eq!(result, Err(Error::IndexOutOfBounds));
    }

    #[test]
    fn test_print_hand() {
        let card = card::Card {
            colour: card::Colour::Red,
            value: card::Value::Number(1),
        };
        let mut player = Player::new(0);
        player.hand.push(card);
        player.hand.push(card);
        player.hand.push(card);
        let expected = format!("00: {card}\n01: {card}\n02: {card}", card = card);
        assert_eq!(player.hand_to_string(), expected);
    }
}
