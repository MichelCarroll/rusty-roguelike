use specs::{Component, VecStorage};

#[derive(Debug)]
pub enum PlayerCommand {
    GoRight,
    GoLeft,
    GoUp,
    GoDown,
}

#[derive(Default)]
pub struct PlayerControlled {
    pub unprocessed_action: Option<PlayerCommand>,
}

impl Component for PlayerControlled {
    type Storage = VecStorage<Self>;
}
