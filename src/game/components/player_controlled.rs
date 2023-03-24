use specs::{Component, VecStorage};

use crate::game::common::UIEvent;

#[derive(Default)]
pub struct PlayerControlled {
    pub unprocessed_action: Option<UIEvent>,
}

impl Component for PlayerControlled {
    type Storage = VecStorage<Self>;
}
