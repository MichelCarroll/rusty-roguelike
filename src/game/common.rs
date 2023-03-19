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
    pub fn BLUE() -> Color { Color { rbg_code: "#1F7A8C".to_string() }}
}