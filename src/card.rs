#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Colour {
    Red,
    Yellow,
    Green,
    Blue,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum CardValue {
    Reverse,
    Skip,
    DrawTwo,
    Number(u8),
}

#[derive(Clone, Copy, Debug)]
pub struct Card {
    pub colour: Colour,
    pub value: CardValue,
}

impl std::fmt::Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let colour = match self.colour {
            Colour::Blue => "blue",
            Colour::Red => "red",
            Colour::Yellow => "yellow",
            Colour::Green => "green",
        };
        let value = match self.value {
            CardValue::DrawTwo => "Draw Two".to_string(),
            CardValue::Reverse => "Reverse".to_string(),
            CardValue::Skip => "Skip".to_string(),
            CardValue::Number(n) => format!("Number {n}", n = n),
        };
        write!(f, "{colour} {value}")
    }
}
