use std::f64::consts::PI;

use log::info;
use specs::prelude::*;

use crate::game::{ components::{movable::{Movable, Direction}, ai_controlled::AIControlled, factioned::{Factioned, Faction}, world_position::WorldPosition}, common::Command};

pub struct AI {}

impl<'a> System<'a> for AI {
    type SystemData = (
        ReadStorage<'a, AIControlled>, 
        ReadStorage<'a, Factioned>,
        ReadStorage<'a, WorldPosition>,
        WriteStorage<'a, Movable>
    );

    fn run(&mut self, (ai_controlled, factioned, world_position, mut movable): Self::SystemData) {
        let first_player_position = (&factioned, &world_position)
            .join()
            .filter_map(|(factioned, world_position)| {
                match factioned.faction {
                    Faction::Player => Some(world_position),
                    _ => None
                }
            })
            .next();

        if let Some(player_position) = first_player_position {

            for (factioned, world_position, _, movable) in (&factioned, &world_position, &ai_controlled, &mut movable).join() {
                if factioned.faction == Faction::Enemy {
                    let delta_x = (player_position.x as f64) - (world_position.x as f64); 
                    let delta_y = (player_position.y as f64) - (world_position.y as f64);
                    let rads = (delta_y).atan2(delta_x);

                    let new_direction = 
                        if rads >= -PI / 4.0 && rads < PI / 4.0 {
                            Direction::Right
                        }
                        else if rads >= PI / 4.0 && rads < PI * 3.0 / 4.0 {
                            Direction::Down
                        }
                        else if rads >= -PI * 3.0 / 4.0 && rads < -PI / 4.0 {
                            Direction::Up
                        }
                        else {
                            Direction::Left
                        };

                    movable.unprocessed_move = new_direction.into();
                }
            }

        }
    }
}
