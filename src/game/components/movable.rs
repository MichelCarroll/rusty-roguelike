use std::f64::consts::PI;

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

    pub fn from_radians(rads: f64) -> Direction {
        if rads >= -PI / 4.0 && rads < PI / 4.0 {
            Direction::Right
        } else if rads >= PI / 4.0 && rads < PI * 3.0 / 4.0 {
            Direction::Down
        } else if rads >= -PI * 3.0 / 4.0 && rads < -PI / 4.0 {
            Direction::Up
        } else {
            Direction::Left
        }
    }
}

#[derive(Default)]
pub struct Movable {
    pub unprocessed_move: Option<Direction>,
}

impl Component for Movable {
    type Storage = VecStorage<Self>;
}
