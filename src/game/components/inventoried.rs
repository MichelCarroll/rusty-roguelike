use specs::{Component, Entity, HashMapStorage};

#[derive(Default)]
pub struct Inventoried { }

impl Component for Inventoried {
    type Storage = HashMapStorage<Self>;
}
