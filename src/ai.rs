use crate::{actor, card, player};

pub struct AiActor {
    id: usize,
    next_card_to_play: usize,
    player: player::Player,
}

impl actor::Actor for AiActor {
    fn get_id(&self) -> usize {
        self.id
    }
    fn get_player(&self) -> &player::Player {
        &self.player
    }

    fn get_player_mut(&mut self) -> &mut player::Player {
        &mut self.player
    }

    fn get_turn_action(&mut self) -> actor::UserAction {
        let card_to_play = self.next_card_to_play;
        if card_to_play < self.player.get_number_of_cards() {
            self.next_card_to_play += 1;
            actor::UserAction::Play(card_to_play)
        } else {
            actor::UserAction::Draw
        }
    }

    fn get_color_choice(&mut self) -> card::Colour {
        struct ColourCount {
            colour: card::Colour,
            count: usize,
        }
        let mut red = ColourCount {
            colour: card::Colour::Red,
            count: 0,
        };
        let mut green = ColourCount {
            colour: card::Colour::Green,
            count: 0,
        };
        let mut blue = ColourCount {
            colour: card::Colour::Blue,
            count: 0,
        };
        let mut yellow = ColourCount {
            colour: card::Colour::Yellow,
            count: 0,
        };
        for card in self.player.get_hand() {
            match card.colour {
                card::Colour::Red => red.count += 1,
                card::Colour::Green => green.count += 1,
                card::Colour::Blue => blue.count += 1,
                card::Colour::Yellow => yellow.count += 1,
                card::Colour::Wild => {}
            }
        }

        [red, green, blue, yellow]
            .iter()
            .max_by(|a, b| a.count.cmp(&b.count))
            .unwrap() // The iterator is never empty
            .colour
    }

    fn pre_turn_action(&mut self) {
        // Do nothing
    }

    fn post_turn_action(&mut self) {
        self.next_card_to_play = 0;
    }
}

impl AiActor {
    pub fn new(id: usize) -> AiActor {
        AiActor {
            id,
            next_card_to_play: 0,
            player: player::Player::new(),
        }
    }
}
