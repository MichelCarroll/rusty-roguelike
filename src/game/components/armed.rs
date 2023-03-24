use specs::{Component, Entity, HashMapStorage};

#[derive(Default)]
pub struct Armed {
    pub damage: u32,
    pub targetting: Option<Entity>,
}

impl Component for Armed {
    type Storage = HashMapStorage<Self>;
}
