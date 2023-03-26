use specs::{Component, VecStorage};

#[derive(Default)]
pub struct Describable {
    pub description: String,
}

impl Component for Describable {
    type Storage = VecStorage<Self>;
}
