use card::{Card, CardValue, Colour};
use rand::{seq::SliceRandom, thread_rng};
use std::io;

mod card;

#[rustfmt::skip]
const GAME_DECK: [Card; 100] = [
    Card { colour: Colour::Red,    value: CardValue::Number(0) },
    Card { colour: Colour::Red,    value: CardValue::Number(1) },
    Card { colour: Colour::Red,    value: CardValue::Number(1) },
    Card { colour: Colour::Red,    value: CardValue::Number(2) },
    Card { colour: Colour::Red,    value: CardValue::Number(2) },
    Card { colour: Colour::Red,    value: CardValue::Number(3) },
    Card { colour: Colour::Red,    value: CardValue::Number(3) },
    Card { colour: Colour::Red,    value: CardValue::Number(4) },
    Card { colour: Colour::Red,    value: CardValue::Number(4) },
    Card { colour: Colour::Red,    value: CardValue::Number(5) },
    Card { colour: Colour::Red,    value: CardValue::Number(5) },
    Card { colour: Colour::Red,    value: CardValue::Number(6) },
    Card { colour: Colour::Red,    value: CardValue::Number(6) },
    Card { colour: Colour::Red,    value: CardValue::Number(7) },
    Card { colour: Colour::Red,    value: CardValue::Number(7) },
    Card { colour: Colour::Red,    value: CardValue::Number(8) },
    Card { colour: Colour::Red,    value: CardValue::Number(8) },
    Card { colour: Colour::Red,    value: CardValue::Number(9) },
    Card { colour: Colour::Red,    value: CardValue::Number(9) },
    Card { colour: Colour::Red,    value: CardValue::Skip },
    Card { colour: Colour::Red,    value: CardValue::Skip },
    Card { colour: Colour::Red,    value: CardValue::DrawTwo },
    Card { colour: Colour::Red,    value: CardValue::DrawTwo },
    Card { colour: Colour::Red,    value: CardValue::Reverse },
    Card { colour: Colour::Red,    value: CardValue::Reverse },
    Card { colour: Colour::Green,  value: CardValue::Number(0) },
    Card { colour: Colour::Green,  value: CardValue::Number(1) },
    Card { colour: Colour::Green,  value: CardValue::Number(1) },
    Card { colour: Colour::Green,  value: CardValue::Number(2) },
    Card { colour: Colour::Green,  value: CardValue::Number(2) },
    Card { colour: Colour::Green,  value: CardValue::Number(3) },
    Card { colour: Colour::Green,  value: CardValue::Number(3) },
    Card { colour: Colour::Green,  value: CardValue::Number(4) },
    Card { colour: Colour::Green,  value: CardValue::Number(4) },
    Card { colour: Colour::Green,  value: CardValue::Number(5) },
    Card { colour: Colour::Green,  value: CardValue::Number(5) },
    Card { colour: Colour::Green,  value: CardValue::Number(6) },
    Card { colour: Colour::Green,  value: CardValue::Number(6) },
    Card { colour: Colour::Green,  value: CardValue::Number(7) },
    Card { colour: Colour::Green,  value: CardValue::Number(7) },
    Card { colour: Colour::Green,  value: CardValue::Number(8) },
    Card { colour: Colour::Green,  value: CardValue::Number(8) },
    Card { colour: Colour::Green,  value: CardValue::Number(9) },
    Card { colour: Colour::Green,  value: CardValue::Number(9) },
    Card { colour: Colour::Green,  value: CardValue::Skip },
    Card { colour: Colour::Green,  value: CardValue::Skip },
    Card { colour: Colour::Green,  value: CardValue::DrawTwo },
    Card { colour: Colour::Green,  value: CardValue::DrawTwo },
    Card { colour: Colour::Green,  value: CardValue::Reverse },
    Card { colour: Colour::Green,  value: CardValue::Reverse },
    Card { colour: Colour::Blue,   value: CardValue::Number(0) },
    Card { colour: Colour::Blue,   value: CardValue::Number(1) },
    Card { colour: Colour::Blue,   value: CardValue::Number(1) },
    Card { colour: Colour::Blue,   value: CardValue::Number(2) },
    Card { colour: Colour::Blue,   value: CardValue::Number(2) },
    Card { colour: Colour::Blue,   value: CardValue::Number(3) },
    Card { colour: Colour::Blue,   value: CardValue::Number(3) },
    Card { colour: Colour::Blue,   value: CardValue::Number(4) },
    Card { colour: Colour::Blue,   value: CardValue::Number(4) },
    Card { colour: Colour::Blue,   value: CardValue::Number(5) },
    Card { colour: Colour::Blue,   value: CardValue::Number(5) },
    Card { colour: Colour::Blue,   value: CardValue::Number(6) },
    Card { colour: Colour::Blue,   value: CardValue::Number(6) },
    Card { colour: Colour::Blue,   value: CardValue::Number(7) },
    Card { colour: Colour::Blue,   value: CardValue::Number(7) },
    Card { colour: Colour::Blue,   value: CardValue::Number(8) },
    Card { colour: Colour::Blue,   value: CardValue::Number(8) },
    Card { colour: Colour::Blue,   value: CardValue::Number(9) },
    Card { colour: Colour::Blue,   value: CardValue::Number(9) },
    Card { colour: Colour::Blue,   value: CardValue::Skip },
    Card { colour: Colour::Blue,   value: CardValue::Skip },
    Card { colour: Colour::Blue,   value: CardValue::DrawTwo },
    Card { colour: Colour::Blue,   value: CardValue::DrawTwo },
    Card { colour: Colour::Blue,   value: CardValue::Reverse },
    Card { colour: Colour::Blue,   value: CardValue::Reverse },
    Card { colour: Colour::Yellow, value: CardValue::Number(0) },
    Card { colour: Colour::Yellow, value: CardValue::Number(1) },
    Card { colour: Colour::Yellow, value: CardValue::Number(1) },
    Card { colour: Colour::Yellow, value: CardValue::Number(2) },
    Card { colour: Colour::Yellow, value: CardValue::Number(2) },
    Card { colour: Colour::Yellow, value: CardValue::Number(3) },
    Card { colour: Colour::Yellow, value: CardValue::Number(3) },
    Card { colour: Colour::Yellow, value: CardValue::Number(4) },
    Card { colour: Colour::Yellow, value: CardValue::Number(4) },
    Card { colour: Colour::Yellow, value: CardValue::Number(5) },
    Card { colour: Colour::Yellow, value: CardValue::Number(5) },
    Card { colour: Colour::Yellow, value: CardValue::Number(6) },
    Card { colour: Colour::Yellow, value: CardValue::Number(6) },
    Card { colour: Colour::Yellow, value: CardValue::Number(7) },
    Card { colour: Colour::Yellow, value: CardValue::Number(7) },
    Card { colour: Colour::Yellow, value: CardValue::Number(8) },
    Card { colour: Colour::Yellow, value: CardValue::Number(8) },
    Card { colour: Colour::Yellow, value: CardValue::Number(9) },
    Card { colour: Colour::Yellow, value: CardValue::Number(9) },
    Card { colour: Colour::Yellow, value: CardValue::Skip },
    Card { colour: Colour::Yellow, value: CardValue::Skip },
    Card { colour: Colour::Yellow, value: CardValue::DrawTwo },
    Card { colour: Colour::Yellow, value: CardValue::DrawTwo },
    Card { colour: Colour::Yellow, value: CardValue::Reverse },
    Card { colour: Colour::Yellow, value: CardValue::Reverse },
];

struct Deck {
    draw_pile: Vec<Card>,
    discard_pile: Vec<Card>,
}

impl Deck {
    fn draw(&mut self) -> Option<Card> {
        self.draw_pile.pop()
    }

    fn discard(&mut self, card: Card) {
        self.discard_pile.push(card);
    }

    fn shuffle(&mut self) {
        let mut rng = thread_rng();
        self.draw_pile.shuffle(&mut rng);
    }

    fn get_top_card(&self) -> Option<&Card> {
        self.discard_pile.last()
    }

    fn draw_pile_is_empty(&self) -> bool {
        self.draw_pile.is_empty()
    }

    fn refill_draw_pile(&mut self) {
        self.draw_pile.append(&mut self.discard_pile);
        self.discard_pile.clear();
        self.shuffle();
        self.discard_from_draw_pile();
    }

    fn discard_from_draw_pile(&mut self) {
        if let Some(card) = self.draw_pile.pop() {
            self.discard(card);
        }
    }

    fn new() -> Self {
        let mut deck = Deck {
            draw_pile: GAME_DECK.to_vec(),
            discard_pile: Vec::new(),
        };
        deck.shuffle();
        deck.discard_from_draw_pile();
        deck
    }
}

struct Player {
    id: usize,
    hand: Vec<Card>,
}

impl Player {
    fn draw(&mut self, deck: &mut Deck) {
        if let Some(card) = deck.draw() {
            self.hand.push(card);
        }
    }

    fn play_card(&mut self, index: usize) -> Result<Card, &str> {
        if index < self.hand.len() {
            Ok(self.hand.remove(index))
        } else {
            Err("Index out of bounds")
        }
    }

    fn is_hand_empty(&self) -> bool {
        self.hand.is_empty()
    }

    fn new(id: usize) -> Self {
        Player {
            id,
            hand: Vec::new(),
        }
    }
}

struct Game {
    players: Vec<Player>,
    deck: Deck,
    player_index: usize,
    is_direction_ascending: bool,
}

impl Game {
    fn player_turn(&mut self, player_index: usize, card: &Card) {
        match card.value {
            CardValue::Number(_) => {}
            CardValue::DrawTwo => {
                self.players[player_index].draw(&mut self.deck);
                self.players[player_index].draw(&mut self.deck);
            }
            CardValue::Skip => self.next_player(),
            CardValue::Reverse => self.revese_direction(),
        };
        self.deck.discard(*card);
    }

    fn next_player(&mut self) {
        if !self.is_direction_ascending && self.player_index == 0 {
            self.player_index = self.players.len() - 1;
        } else {
            let index_increment: isize = if self.is_direction_ascending { 1 } else { -1 };
            self.player_index =
                (self.player_index.wrapping_add_signed(index_increment)) % self.players.len();
        }
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

    fn new(num_of_players: usize) -> Self {
        let players = (0..num_of_players).map(Player::new).collect();
        Game {
            players,
            deck: Deck::new(),
            player_index: 0,
            is_direction_ascending: true,
        }
    }

    fn start_game(&mut self) {
        let num_of_players = self.players.len();
        for i in 0..num_of_players {
            for _ in 0..7 {
                self.players[i].draw(&mut self.deck);
            }
        }

        let winner = loop {
            println!("Player {player}'s turn", player = self.player_index);
            println!(
                "Number of cards in the draw pile: {}",
                self.deck.draw_pile.len()
            );
            if let Some(card) = self.deck.get_top_card() {
                println!("card on top: {card}");
            } else {
                print!("No card on top... somehow...")
            }
            print_hand(&self.players[self.player_index].hand);
            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");
            let mut index: usize = input.trim().parse().expect("Failed to parse number");

            let player_draws = index == 99;

            if player_draws {
                self.players[self.player_index].draw(&mut self.deck);
            } else {
                let card = loop {
                    if let Ok(card) = self.players[self.player_index].play_card(index) {
                        break card;
                    } else {
                        let mut input = String::new();
                        io::stdin()
                            .read_line(&mut input)
                            .expect("Failed to read line");
                        index = input.trim().parse().expect("Failed to parse number");
                    }
                };
                let is_valid_play = self.is_valid_play(card);
                if is_valid_play {
                    self.player_turn(self.player_index, &card);
                }
                if self.players[self.player_index].is_hand_empty() {
                    println!("Game over");
                    break self.player_index;
                }
                if self.deck.draw_pile_is_empty() {
                    self.deck.refill_draw_pile();
                }
            }

            self.next_player();
        };

        println!("Player {winner} wins!");
    }
}

fn print_hand(hand: &[Card]) {
    let _ = hand
        .iter()
        .enumerate()
        .map(|(i, card)| println!("{i:02}: {card}"))
        .collect::<Vec<_>>();
}

fn main() {
    println!("Hello world!");
    let mut game = Game::new(3);
    game.start_game();
}
