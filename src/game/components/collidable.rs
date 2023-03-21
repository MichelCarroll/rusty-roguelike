use specs::{Component, NullStorage};

#[derive(Default)]
pub struct Collidable;

impl Component for Collidable {
    type Storage = NullStorage<Self>;
}
