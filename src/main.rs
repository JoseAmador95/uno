enum Colour {
    Red,
    Yellow,
    Green,
    Blue,
}

enum CardValue {
    Reverse,
    Skip,
    DrawTwo,
    Number(u8),
}

struct Card {
    colour: Colour,
    value: CardValue,
}

struct Deck {
    draw_pile: Vec<Card>,
    discard_pile: Vec<Card>,
}

impl Deck {
    fn draw(&mut self) -> Option<Card> {
        todo!()
    }

    fn discard(&mut self, card: Card) {
        todo!()
    }

    fn shuffle(&mut self) {
        todo!()
    }

    fn get_top_card() -> Card {
        todo!()
    }
}

struct Player {
    id: usize,
    hand: Vec<Card>,
}

impl Player {
    fn draw(deck: Deck) {
        todo!()
    }
    fn play_card(index: usize) -> Card {
        todo!()
    }
}

struct Game {
    players: Vec<Player>,
    deck: Deck,
    player_index: usize,
    is_direction_ascending: bool,
}

impl Game {
    fn player_turn(player: Player, card: Card) -> bool {
        todo!()
    }

    fn next_player(&mut self) {
        todo!()
    }

    fn revese_direction(&mut self) {
        todo!()
    }

    fn is_valid_play(card: Card) -> bool {
        todo!()
    }
}

fn main() {
    println!("Hello world!")
}
