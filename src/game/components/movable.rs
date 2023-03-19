use specs::{Component, VecStorage};

#[derive(Debug)]
pub enum Direction {
    Right,
    Left,
    Up,
    Down,
}

#[derive(Default)]
pub struct Movable {
    pub unprocessed_move: Option<Direction>,
}

impl Component for Movable {
    type Storage = VecStorage<Self>;
}
