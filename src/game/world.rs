use super::common::Command;

#[derive(Default)]
pub struct LastUserEvent {
    pub event: Option<Command>,
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