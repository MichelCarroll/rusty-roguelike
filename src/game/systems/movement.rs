use specs::prelude::*;

use crate::game::{ components::{movable::{Movable}, world_position::WorldPosition}, world::WorldParameters};

pub struct Movement {}

impl<'a> System<'a> for Movement {
    type SystemData = (WriteStorage<'a, Movable>, WriteStorage<'a, WorldPosition>, Read<'a, WorldParameters>);

    fn run(&mut self, (mut movable, mut world_position, world_parameters): Self::SystemData) {
        for (movable, world_position) in (&mut movable, &mut world_position).join() {
            if let Some(direction) = movable.unprocessed_move.take() {
                *world_position = world_position.moved(direction, world_parameters.width, world_parameters.height)
            }
        }
    }
}
