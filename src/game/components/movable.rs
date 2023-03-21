use specs::{Component, VecStorage};

use crate::game::random::random_in_vec;

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    Right,
    Left,
    Up,
    Down,
}

impl Direction {
    pub fn random() -> Direction {
        let directions = vec![
            Direction::Right,
            Direction::Left,
            Direction::Up,
            Direction::Down,
        ];
        random_in_vec(&directions).unwrap().clone()
    }
}

#[derive(Default)]
pub struct Movable {
    pub unprocessed_move: Option<Direction>,
}

impl Component for Movable {
    type Storage = VecStorage<Self>;
}
