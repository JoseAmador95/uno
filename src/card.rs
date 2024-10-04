use colored::Colorize;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Colour {
    Red,
    Yellow,
    Green,
    Blue,
    Wild,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum CardValue {
    Reverse,
    Skip,
    DrawTwo,
    Number(u8),
    Wild,
    WildDraw(u8),
}

#[derive(Clone, Copy, Debug)]
pub struct Card {
    pub colour: Colour,
    pub value: CardValue,
}

impl std::fmt::Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let colour = match self.colour {
            Colour::Blue => "blue".blue(),
            Colour::Red => "red".red(),
            Colour::Yellow => "yellow".yellow(),
            Colour::Green => "green".green(),
            _ => "".bold(),
        };
        let value = match self.value {
            CardValue::DrawTwo => "Draw Two".to_string(),
            CardValue::Reverse => "Reverse".to_string(),
            CardValue::Skip => "Skip".to_string(),
            CardValue::Number(n) => format!("Number {n}"),
            CardValue::Wild => "Wild".to_string(),
            CardValue::WildDraw(n) => format!("Wild Draw +{n}"),
        };
        write!(f, "{colour} {value}")
    }
}
