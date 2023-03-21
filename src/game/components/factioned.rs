use specs::{Component, VecStorage};

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum Faction {
    Player,
    Enemy,
}

#[derive(Clone, Debug)]
pub struct Factioned {
    pub faction: Faction,
}

impl Component for Factioned {
    type Storage = VecStorage<Self>;
}
