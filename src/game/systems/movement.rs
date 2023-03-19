use std::collections::HashSet;

use specs::prelude::*;

use crate::game::{ components::{movable::{Movable}, world_position::WorldPosition, collidable::Collidable}, world::WorldParameters};

pub struct Movement {}

impl<'a> System<'a> for Movement {
    type SystemData = (
        WriteStorage<'a, Movable>, 
        WriteStorage<'a, WorldPosition>, 
        ReadStorage<'a, Collidable>, 
        Read<'a, WorldParameters>
    );

    fn run(&mut self, (mut movable, mut world_position, collidable, world_parameters): Self::SystemData) {
        let mut collidable_map: HashSet<WorldPosition> = HashSet::new();

        for (_, world_position) in (&collidable, &world_position).join() {
            collidable_map.insert(*world_position);
        }

        for (movable, world_position) in (&mut movable, &mut world_position).join() {
            if let Some(direction) = movable.unprocessed_move.take() {
                let new_world_position = world_position.moved(direction, world_parameters.width, world_parameters.height);
                if !collidable_map.contains(&new_world_position) {
                    *world_position = new_world_position;
                }
            }
        }
    }
}
