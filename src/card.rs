use colored::Colorize;

#[derive(Clone, Copy, PartialEq)]
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

#[derive(Clone, Copy)]
pub struct Card {
    pub colour: Colour,
    pub value: CardValue,
}

impl std::fmt::Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let colour = match self.colour {
            Colour::Blue => "Blue".blue(),
            Colour::Red => "Red".red(),
            Colour::Yellow => "Yellow".yellow(),
            Colour::Green => "Green".green(),
            Colour::Wild => format!(
                "{w}{i}{l}{d}",
                w = "W".red(),
                i = "i".yellow(),
                l = "l".green(),
                d = "d".blue()
            )
            .normal(),
        };
        let value = match self.value {
            CardValue::DrawTwo => "Draw Two".to_string(),
            CardValue::Reverse => "Reverse".to_string(),
            CardValue::Skip => "Skip".to_string(),
            CardValue::Number(n) => n.to_string(),
            CardValue::Wild => "Select Color".to_string(),
            CardValue::WildDraw(n) => format!("Draw +{n}"),
        };

        write!(f, "{colour} {value}")
    }
}
