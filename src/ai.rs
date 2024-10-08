use crate::{card, player::Player, ui::UserAction};

struct Ai {
    next_card_to_play: usize,
}
static mut AI: Ai = Ai {
    next_card_to_play: 0,
};

pub fn get_ai_turn_action(player: &Player) -> UserAction {
    unsafe {
        let card_to_play = AI.next_card_to_play;
        if card_to_play < player.get_number_of_cards() {
            AI.next_card_to_play += 1;
            UserAction::Play(card_to_play)
        } else {
            UserAction::Draw
        }
    }
}

pub fn reset_ai() {
    unsafe {
        AI.next_card_to_play = 0;
    }
}

pub fn choose_colour(player: &Player) -> card::Colour {
    let mut red: (card::Colour, usize) = (card::Colour::Red, 0);
    let mut green: (card::Colour, usize) = (card::Colour::Green, 0);
    let mut blue: (card::Colour, usize) = (card::Colour::Blue, 0);
    let mut yellow: (card::Colour, usize) = (card::Colour::Yellow, 0);
    for card in player.get_hand() {
        match card.colour {
            card::Colour::Red => red.1 += 1,
            card::Colour::Green => green.1 += 1,
            card::Colour::Blue => blue.1 += 1,
            card::Colour::Yellow => yellow.1 += 1,
            card::Colour::Wild => {}
        }
    }

    [red, green, blue, yellow]
        .iter()
        .max_by(|a, b| a.1.cmp(&b.1))
        .unwrap()
        .0
}
