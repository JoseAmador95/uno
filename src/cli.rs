use clap::Parser;

const ABOUT:&str= "Uno is the highly popular card game played by millions around the globe. This game is played by matching and then discarding the cards in one’s hand untill none are left.";

#[derive(Parser, Debug)]
#[command(version, about, long_about = ABOUT)]
pub struct Args {
    #[arg(
        short = 'p',
        long,
        default_value_t = 2,
        help = "Number of players in the game"
    )]
    pub num_of_players: usize,

    #[arg(
        short = 'c',
        long,
        default_value_t = 7,
        help = "Numbers of cards dealt to each player at the start of the game"
    )]
    pub num_of_cards: usize,
}

pub fn parse_cli() -> Args {
    Args::parse_from(std::env::args())
}
