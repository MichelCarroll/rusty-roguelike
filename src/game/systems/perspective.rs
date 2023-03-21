use log::info;
use specs::prelude::*;

use crate::game::components::{sighted::Sighted, world_position::WorldPosition};

pub struct Perspective {}

impl<'a> System<'a> for Perspective {
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, WorldPosition>,
        WriteStorage<'a, Sighted>,
    );

    fn run(&mut self, (entities, world_position, mut sighted): Self::SystemData) {
        for (sighted_world_position, sighted) in (&world_position, &mut sighted).join() {
            sighted.seen.clear();
            for (entity, seen_world_position) in (&entities, &world_position).join() {
                if sighted_world_position.distance_from(*seen_world_position) < 5.0 {
                    sighted.seen.add(entity.id());
                }
            }
        }
    }
}
