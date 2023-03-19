#[derive(Clone, Copy)]
pub struct CanvasSize {
    pub width: f64,
    pub height: f64,
}

#[derive(Clone, Debug)]
pub struct Color {
    pub rbg_code: String,
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8) -> Color {
        Color {
            rbg_code: format!("#{:02X?}{:02X?}{:02X?}", r, g, b),
        }
    }
}

impl Color {
    pub fn BROWN() -> Color { Color { rbg_code: "#B7999C".to_string() }}
    pub fn BLACK() -> Color { Color { rbg_code: "#2D2D34".to_string() }}
    pub fn MILDEW() -> Color { Color { rbg_code: "#8EAF9D".to_string() }}
    pub fn YELLOW() -> Color { Color { rbg_code: "#F4D35E".to_string() }}
    pub fn RED() -> Color { Color { rbg_code: "#EA3546".to_string() }}
}


#[derive(Debug)]
pub enum Command {
    GoRight,
    GoLeft,
    GoUp,
    GoDown,
}