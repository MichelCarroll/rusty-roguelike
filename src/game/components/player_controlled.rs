use specs::{Component, VecStorage};

use crate::game::common::Command;

#[derive(Default)]
pub struct PlayerControlled {
    pub unprocessed_action: Option<Command>,
}

impl Component for PlayerControlled {
    type Storage = VecStorage<Self>;
}
