use super::components::player_controlled::PlayerCommand;

#[derive(Default)]
pub struct LastUserEvent {
    pub event: Option<PlayerCommand>,
}
