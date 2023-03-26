use specs::{Component, Entity, HashMapStorage};

#[derive(Default)]
pub struct Level {
    pub generated: bool,
}

impl Component for Level {
    type Storage = HashMapStorage<Self>;
}
