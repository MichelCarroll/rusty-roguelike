use std::{f64::consts::PI, collections::HashSet};

use log::info;
use specs::prelude::*;

use crate::game::{components::{sighted::Sighted, opaque::Opaque}, world::{WorldPosition, WorldParameters}, algorithms::raycasting::Raycast};

pub struct Perspective {}

const MAX_CELL_DISTANCE: f64 = 20.0;

impl<'a> System<'a> for Perspective {
    type SystemData = (
        Entities<'a>,
        Read<'a, WorldParameters>,
        ReadStorage<'a, WorldPosition>,
        ReadStorage<'a, Opaque>,
        WriteStorage<'a, Sighted>,
    );

    fn run(&mut self, (entities, world_parameters, world_position, opaque, mut sighted): Self::SystemData) {
        for (sighted_world_position, sighted) in (&world_position, &mut sighted).join() {
            sighted.seen.clear();

            let mut has_opaque = HashSet::<WorldPosition>::new();
            for (_, opaque_world_position) in (&opaque, &world_position).join() {
                has_opaque.insert(*opaque_world_position);
            }

            let num_rays = (2.0 * PI * MAX_CELL_DISTANCE) as u32 * 2;
            let mut radians = 0.0;
            let radian_delta = (PI * 2.0) / num_rays as f64;
            
            let mut seen_positions = HashSet::<WorldPosition>::new();
            seen_positions.insert(*sighted_world_position);

            for _ in 0..num_rays {
                for ray_world_position in Raycast::new(*sighted_world_position, world_parameters.max_position(), radians - PI) {
                    seen_positions.insert(ray_world_position);
                    if !has_opaque.contains(&ray_world_position) && sighted_world_position.distance_from(ray_world_position) < MAX_CELL_DISTANCE {
                        seen_positions.insert(ray_world_position);
                    }
                    else {
                        break;
                    }
                }
                radians += radian_delta;
            }

            for (entity, seen_world_position) in (&entities, &world_position).join() {
                if seen_positions.contains(seen_world_position) {
                    sighted.seen.add(entity.id());
                }
            }
        }
    }
}