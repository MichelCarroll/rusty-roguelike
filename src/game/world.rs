use super::common::{CanvasPosition, CanvasSize, UIEvent};

pub const CELL_SIZE: f64 = 50.0;

#[derive(Default)]
pub struct LastUserEvent {
    pub events: Vec<UIEvent>,
}

#[derive(Debug, Default)]
pub struct WorldParameters {
    pub width: u64,
    pub height: u64,
}

impl WorldParameters {
    pub fn max_position(&self) -> WorldPosition {
        WorldPosition {
            x: self.width - 1,
            y: self.height - 1,
        }
    }

    pub fn from_canvas_size(canvas_size: CanvasSize) -> WorldParameters {
        WorldParameters {
            width: (canvas_size.width as f64 / CELL_SIZE) as u64,
            height: (canvas_size.height as f64 / CELL_SIZE) as u64,
        }
    }
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct WorldPosition {
    pub x: u64,
    pub y: u64,
}

#[derive(Default)]
pub struct WorldTime {
    pub tick: u64,
}

#[derive(Default)]
pub struct UIState {
    pub mouse_over: Option<CanvasPosition>,
}
