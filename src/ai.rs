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
        let mut red: (card::Colour, usize) = (card::Colour::Red, 0);
        let mut green: (card::Colour, usize) = (card::Colour::Green, 0);
        let mut blue: (card::Colour, usize) = (card::Colour::Blue, 0);
        let mut yellow: (card::Colour, usize) = (card::Colour::Yellow, 0);
        for card in game.get_player(self.player_index).get_hand() {
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
