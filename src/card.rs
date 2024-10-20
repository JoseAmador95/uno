use colored::Colorize;

/// Represents the possible colors of a card.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Colour {
    Red,
    Yellow,
    Green,
    Blue,
    Wild,
}

/// Represents the possible values of a card.
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Value {
    Reverse,
    Skip,
    DrawTwo,
    Number(usize),
    Wild,
    WildDraw(usize),
}

/// Represents a card and its attributes.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Card {
    pub colour: Colour,
    pub value: Value,
}

impl std::fmt::Display for Card {
    /// Formats the card as a string with colored output.
    ///
    /// Returns a `std::fmt::Result` indicating the success or failure of the formatting operation.
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
            Value::DrawTwo => "Draw Two".to_string(),
            Value::Reverse => "Reverse".to_string(),
            Value::Skip => "Skip".to_string(),
            Value::Number(n) => n.to_string(),
            Value::Wild => "Select Color".to_string(),
            Value::WildDraw(n) => format!("Draw +{n}"),
        };

        write!(f, "{colour} {value}")
    }
}
