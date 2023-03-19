use specs::{Component, NullStorage};


#[derive(Default)]
pub struct Pickupable;

impl Component for Pickupable {
    type Storage = NullStorage<Self>;
}
 