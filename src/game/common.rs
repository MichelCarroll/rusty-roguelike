#[derive(Clone, Copy)]
pub struct CanvasSize {
    pub width: f64,
    pub height: f64,
}

#[derive(Debug, Clone, Copy)]
pub struct CanvasPosition {
    pub x: f64,
    pub y: f64,
}

impl CanvasPosition {
    pub fn from_real(
        mouse_position: CanvasPosition,
        actual_canvas_size: CanvasSize,
        target_canvas_size: CanvasSize,
    ) -> CanvasPosition {
        CanvasPosition {
            x: mouse_position.x / actual_canvas_size.width * target_canvas_size.width,
            y: mouse_position.y / actual_canvas_size.height * target_canvas_size.height,
        }
    }
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
    pub fn brown() -> Color {
        Color {
            r: 226,
            g: 132,
            b: 19,
        }
    }
    pub fn black() -> Color {
        Color { r: 0, g: 0, b: 34 }
    }
    pub fn mildew() -> Color {
        Color {
            r: 51,
            g: 101,
            b: 138,
        }
    }
    pub fn yellow() -> Color {
        Color {
            r: 246,
            g: 174,
            b: 45,
        }
    }
    pub fn deep_red() -> Color {
        Color {
            r: 107,
            g: 39,
            b: 55,
        }
    }
    pub fn bright_red() -> Color {
        Color {
            r: 220,
            g: 20,
            b: 20,
        }
    }

    pub fn darkened(self) -> Color {
        Color {
            r: self.r / 2,
            g: self.g / 2,
            b: self.b / 2,
        }
    }

    pub fn tinted(self) -> Color {
        Color {
            r: self.r + (255 - self.r) / 2,
            g: self.g + (255 - self.g) / 2,
            b: self.b + (255 - self.b) / 2,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum UIEvent {
    Right,
    Left,
    Up,
    Down,
    MouseOver(CanvasPosition),
    MousePress(CanvasPosition),
    MouseLeave,
}
