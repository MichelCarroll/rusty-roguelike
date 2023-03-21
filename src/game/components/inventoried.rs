use specs::{Component, Entity, HashMapStorage};

#[derive(Default)]
pub struct Inventoried {
    pub contents: Vec<Entity>,
}

impl Component for Inventoried {
    type Storage = HashMapStorage<Self>;
}
