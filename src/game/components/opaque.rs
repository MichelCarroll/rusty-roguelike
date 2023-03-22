use specs::{Component, NullStorage};

#[derive(Default)]
pub struct Opaque;

impl Component for Opaque {
    type Storage = NullStorage<Self>;
}
