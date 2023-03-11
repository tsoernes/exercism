use enum_iterator::{all, Sequence};

#[derive(Debug, PartialEq, Eq, Sequence, PartialOrd, Ord)]
pub enum ResistorColor {
    Black,
    Brown,
    Red,
    Orange,
    Yellow,
    Green,
    Blue,
    Violet,
    Grey,
    White,
}

pub fn color_to_value(color: ResistorColor) -> u32 {
    color as u32
}

pub fn value_to_color_string(value: u32) -> String {
    let maybe_resistor = all::<ResistorColor>().nth(value as usize);
    match maybe_resistor {
        Some(resistor) => format!("{:?}", resistor),
        None => "value out of range".to_string(),
    }
}

pub fn colors() -> Vec<ResistorColor> {
    let mut all_colors: Vec<ResistorColor> = all::<ResistorColor>().collect();
    all_colors.sort();
    all_colors
}
