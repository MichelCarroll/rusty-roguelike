use specs::{Component, HashMapStorage, Entity};

use super::damageable::Damageable;


#[derive(Default)]
pub struct Armed {
    pub damage: u32,
    pub targetting: Option<Entity>
}

impl Component for Armed {
    type Storage = HashMapStorage<Self>;
}
 