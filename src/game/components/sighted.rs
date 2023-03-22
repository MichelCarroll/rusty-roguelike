use specs::{BitSet, Component, HashMapStorage};

#[derive(Default)]
pub struct Sighted {
    pub seen: BitSet,
    pub seen_recently: BitSet,
}

impl Component for Sighted {
    type Storage = HashMapStorage<Self>;
}
