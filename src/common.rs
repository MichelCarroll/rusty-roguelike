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
