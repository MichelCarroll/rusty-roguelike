use std::collections::HashSet;

use super::components::player_controlled::PlayerCommand;

#[derive(Default)]
pub struct LastUserEvent {
    pub event: Option<PlayerCommand>,
}

#[derive(Default)]
pub struct WorldParameters {
    pub width: u64,
    pub height: u64
} 