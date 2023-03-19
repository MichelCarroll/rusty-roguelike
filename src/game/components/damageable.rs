use specs::{Component, HashMapStorage};


#[derive(Default)]
pub struct Damageable {
    pub health: u32
}

impl Component for Damageable {
    type Storage = HashMapStorage<Self>;
}
 