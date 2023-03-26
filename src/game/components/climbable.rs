use specs::{Component, NullStorage};

#[derive(Default)]
pub struct Climbable;

impl Component for Climbable {
    type Storage = NullStorage<Self>;
}
