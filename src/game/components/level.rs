use specs::{Component, Entity, HashMapStorage};


#[derive(Default)]
pub struct Level {
    pub contents: Vec<Entity>
}

impl Component for Level {
    type Storage = HashMapStorage<Self>;
}
