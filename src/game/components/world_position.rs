use std::cmp;

use specs::{Component, VecStorage};

use crate::game::world::WorldPosition;

use super::movable::Direction;

impl Component for WorldPosition {
    type Storage = VecStorage<Self>;
}

impl WorldPosition {
    pub fn moved(
        &self,
        direction: Direction,
        clamp_width: u64,
        clamp_height: u64,
    ) -> WorldPosition {
        let new_direction = match direction {
            Direction::Right => WorldPosition {
                x: self.x.checked_add(1).unwrap_or(0),
                y: self.y,
            },
            Direction::Left => WorldPosition {
                x: self.x.checked_sub(1).unwrap_or(0),
                y: self.y,
            },
            Direction::Up => WorldPosition {
                x: self.x,
                y: self.y.checked_sub(1).unwrap_or(0),
            },
            Direction::Down => WorldPosition {
                x: self.x,
                y: self.y.checked_add(1).unwrap_or(0),
            },
        };
        WorldPosition {
            x: cmp::min(new_direction.x, clamp_width - 1),
            y: cmp::min(new_direction.y, clamp_height - 1),
        }
    }

    pub fn distance_from(&self, other: WorldPosition) -> f64 {
        let delta_x = self.x as i64 - other.x as i64;
        let delta_y = self.y as i64 - other.y as i64;
        ((delta_x.pow(2) + delta_y.pow(2)) as f64).sqrt()
    }
}
