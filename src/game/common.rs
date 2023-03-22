#[derive(Clone, Copy)]
pub struct CanvasSize {
    pub width: f64,
    pub height: f64,
}

#[derive(Clone, Copy, Debug)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
}

impl ToString for Color {
    fn to_string(&self) -> String {
        format!("#{:02X?}{:02X?}{:02X?}", self.r, self.g, self.b)
    }
}

impl Color {
    pub fn BROWN() -> Color {
        Color {
            r: 226, g: 132, b: 19
        }
    }
    pub fn BLACK() -> Color {
        Color {
            r: 0, g: 0, b: 34
        }
    }
    pub fn MILDEW() -> Color {
        Color {
            r: 51, g: 101, b: 138
        }
    }
    pub fn YELLOW() -> Color {
        Color {
            r: 246, g: 174, b: 45
        }
    }
    pub fn RED() -> Color {
        Color {
            r: 107, g: 39, b: 55
        }
    }

    pub fn darkened(self) -> Color {
        Color {
            r: self.r / 2, g: self.g / 2, b: self.b / 2
        }
    }
}

#[derive(Debug)]
pub enum Command {
    GoRight,
    GoLeft,
    GoUp,
    GoDown,
}
