use specs::prelude::*;

use crate::game::{ components::{movable::{Movable}, world_position::WorldPosition}};

pub struct Movement {}

impl<'a> System<'a> for Movement {
    type SystemData = (WriteStorage<'a, Movable>, WriteStorage<'a, WorldPosition>);

    fn run(&mut self, (mut movable, mut world_position): Self::SystemData) {
        for (movable, world_position) in (&mut movable, &mut world_position).join() {
            if let Some(direction) = movable.unprocessed_move.take() {
                *world_position = world_position.moved(direction)
            }
        }
    }
}
