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