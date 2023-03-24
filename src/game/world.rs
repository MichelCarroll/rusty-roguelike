use super::common::{UIEvent, CanvasPosition};

#[derive(Default)]
pub struct LastUserEvent {
    pub event: Option<UIEvent>,
}

#[derive(Default)]
pub struct WorldParameters {
    pub width: u64,
    pub height: u64,
}

impl WorldParameters {
    pub fn max_position(&self) -> WorldPosition {
        WorldPosition { x: self.width - 1, y: self.height - 1 }
    }
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct WorldPosition {
    pub x: u64,
    pub y: u64,
}

#[derive(Default)]
pub struct WorldTime {
    pub tick: u64
}

#[derive(Default)]
pub struct UIState {
    pub mouse_over: Option<CanvasPosition>
}