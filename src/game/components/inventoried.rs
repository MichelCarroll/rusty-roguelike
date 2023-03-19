use specs::{Component, HashMapStorage, Entity};


#[derive(Default)]
pub struct Inventoried {
    pub contents: Vec<Entity>
}

impl Component for Inventoried {
    type Storage = HashMapStorage<Self>;
}
 