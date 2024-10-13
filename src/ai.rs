use crate::{actor, card, game};

pub struct AiActor {
    next_card_to_play: usize,
    player_index: usize,
}

impl actor::Actor for AiActor {
    fn get_turn_action(&mut self, game: &game::Game) -> actor::UserAction {
        let card_to_play = self.next_card_to_play;
        if card_to_play < game.get_player(self.player_index).get_number_of_cards() {
            self.next_card_to_play += 1;
            actor::UserAction::Play(card_to_play)
        } else {
            actor::UserAction::Draw
        }
    }

    fn get_color_choice(&mut self, game: &game::Game) -> card::Colour {
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
        for card in game.get_player(self.player_index).get_hand() {
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

    fn pre_turn_action(&mut self, _game: &game::Game) {
        // Do nothing
    }

    fn post_turn_action(&mut self, _game: &game::Game) {
        self.next_card_to_play = 0;
    }
}

impl AiActor {
    pub fn new(player_index: usize) -> AiActor {
        AiActor {
            next_card_to_play: 0,
            player_index,
        }
    }
}
