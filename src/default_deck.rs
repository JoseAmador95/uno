use crate::card::{Card, Value, Colour};

#[rustfmt::skip]
pub const GAME_DECK: [Card; 108] = [
    Card { colour: Colour::Red,    value: Value::Number(0) },
    Card { colour: Colour::Red,    value: Value::Number(1) },
    Card { colour: Colour::Red,    value: Value::Number(1) },
    Card { colour: Colour::Red,    value: Value::Number(2) },
    Card { colour: Colour::Red,    value: Value::Number(2) },
    Card { colour: Colour::Red,    value: Value::Number(3) },
    Card { colour: Colour::Red,    value: Value::Number(3) },
    Card { colour: Colour::Red,    value: Value::Number(4) },
    Card { colour: Colour::Red,    value: Value::Number(4) },
    Card { colour: Colour::Red,    value: Value::Number(5) },
    Card { colour: Colour::Red,    value: Value::Number(5) },
    Card { colour: Colour::Red,    value: Value::Number(6) },
    Card { colour: Colour::Red,    value: Value::Number(6) },
    Card { colour: Colour::Red,    value: Value::Number(7) },
    Card { colour: Colour::Red,    value: Value::Number(7) },
    Card { colour: Colour::Red,    value: Value::Number(8) },
    Card { colour: Colour::Red,    value: Value::Number(8) },
    Card { colour: Colour::Red,    value: Value::Number(9) },
    Card { colour: Colour::Red,    value: Value::Number(9) },
    Card { colour: Colour::Red,    value: Value::Skip },
    Card { colour: Colour::Red,    value: Value::Skip },
    Card { colour: Colour::Red,    value: Value::DrawTwo },
    Card { colour: Colour::Red,    value: Value::DrawTwo },
    Card { colour: Colour::Red,    value: Value::Reverse },
    Card { colour: Colour::Red,    value: Value::Reverse },
    Card { colour: Colour::Green,  value: Value::Number(0) },
    Card { colour: Colour::Green,  value: Value::Number(1) },
    Card { colour: Colour::Green,  value: Value::Number(1) },
    Card { colour: Colour::Green,  value: Value::Number(2) },
    Card { colour: Colour::Green,  value: Value::Number(2) },
    Card { colour: Colour::Green,  value: Value::Number(3) },
    Card { colour: Colour::Green,  value: Value::Number(3) },
    Card { colour: Colour::Green,  value: Value::Number(4) },
    Card { colour: Colour::Green,  value: Value::Number(4) },
    Card { colour: Colour::Green,  value: Value::Number(5) },
    Card { colour: Colour::Green,  value: Value::Number(5) },
    Card { colour: Colour::Green,  value: Value::Number(6) },
    Card { colour: Colour::Green,  value: Value::Number(6) },
    Card { colour: Colour::Green,  value: Value::Number(7) },
    Card { colour: Colour::Green,  value: Value::Number(7) },
    Card { colour: Colour::Green,  value: Value::Number(8) },
    Card { colour: Colour::Green,  value: Value::Number(8) },
    Card { colour: Colour::Green,  value: Value::Number(9) },
    Card { colour: Colour::Green,  value: Value::Number(9) },
    Card { colour: Colour::Green,  value: Value::Skip },
    Card { colour: Colour::Green,  value: Value::Skip },
    Card { colour: Colour::Green,  value: Value::DrawTwo },
    Card { colour: Colour::Green,  value: Value::DrawTwo },
    Card { colour: Colour::Green,  value: Value::Reverse },
    Card { colour: Colour::Green,  value: Value::Reverse },
    Card { colour: Colour::Blue,   value: Value::Number(0) },
    Card { colour: Colour::Blue,   value: Value::Number(1) },
    Card { colour: Colour::Blue,   value: Value::Number(1) },
    Card { colour: Colour::Blue,   value: Value::Number(2) },
    Card { colour: Colour::Blue,   value: Value::Number(2) },
    Card { colour: Colour::Blue,   value: Value::Number(3) },
    Card { colour: Colour::Blue,   value: Value::Number(3) },
    Card { colour: Colour::Blue,   value: Value::Number(4) },
    Card { colour: Colour::Blue,   value: Value::Number(4) },
    Card { colour: Colour::Blue,   value: Value::Number(5) },
    Card { colour: Colour::Blue,   value: Value::Number(5) },
    Card { colour: Colour::Blue,   value: Value::Number(6) },
    Card { colour: Colour::Blue,   value: Value::Number(6) },
    Card { colour: Colour::Blue,   value: Value::Number(7) },
    Card { colour: Colour::Blue,   value: Value::Number(7) },
    Card { colour: Colour::Blue,   value: Value::Number(8) },
    Card { colour: Colour::Blue,   value: Value::Number(8) },
    Card { colour: Colour::Blue,   value: Value::Number(9) },
    Card { colour: Colour::Blue,   value: Value::Number(9) },
    Card { colour: Colour::Blue,   value: Value::Skip },
    Card { colour: Colour::Blue,   value: Value::Skip },
    Card { colour: Colour::Blue,   value: Value::DrawTwo },
    Card { colour: Colour::Blue,   value: Value::DrawTwo },
    Card { colour: Colour::Blue,   value: Value::Reverse },
    Card { colour: Colour::Blue,   value: Value::Reverse },
    Card { colour: Colour::Yellow, value: Value::Number(0) },
    Card { colour: Colour::Yellow, value: Value::Number(1) },
    Card { colour: Colour::Yellow, value: Value::Number(1) },
    Card { colour: Colour::Yellow, value: Value::Number(2) },
    Card { colour: Colour::Yellow, value: Value::Number(2) },
    Card { colour: Colour::Yellow, value: Value::Number(3) },
    Card { colour: Colour::Yellow, value: Value::Number(3) },
    Card { colour: Colour::Yellow, value: Value::Number(4) },
    Card { colour: Colour::Yellow, value: Value::Number(4) },
    Card { colour: Colour::Yellow, value: Value::Number(5) },
    Card { colour: Colour::Yellow, value: Value::Number(5) },
    Card { colour: Colour::Yellow, value: Value::Number(6) },
    Card { colour: Colour::Yellow, value: Value::Number(6) },
    Card { colour: Colour::Yellow, value: Value::Number(7) },
    Card { colour: Colour::Yellow, value: Value::Number(7) },
    Card { colour: Colour::Yellow, value: Value::Number(8) },
    Card { colour: Colour::Yellow, value: Value::Number(8) },
    Card { colour: Colour::Yellow, value: Value::Number(9) },
    Card { colour: Colour::Yellow, value: Value::Number(9) },
    Card { colour: Colour::Yellow, value: Value::Skip },
    Card { colour: Colour::Yellow, value: Value::Skip },
    Card { colour: Colour::Yellow, value: Value::DrawTwo },
    Card { colour: Colour::Yellow, value: Value::DrawTwo },
    Card { colour: Colour::Yellow, value: Value::Reverse },
    Card { colour: Colour::Yellow, value: Value::Reverse },
    Card { colour: Colour::Wild,   value: Value::Wild },
    Card { colour: Colour::Wild,   value: Value::Wild },
    Card { colour: Colour::Wild,   value: Value::Wild },
    Card { colour: Colour::Wild,   value: Value::Wild },
    Card { colour: Colour::Wild,   value: Value::WildDraw(4) },
    Card { colour: Colour::Wild,   value: Value::WildDraw(4) },
    Card { colour: Colour::Wild,   value: Value::WildDraw(4) },
    Card { colour: Colour::Wild,   value: Value::WildDraw(4) },
];
