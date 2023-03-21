use std::collections::{HashMap, HashSet};

use log::info;
use specs::prelude::*;

use crate::game::{
    components::{
        armed::Armed, collidable::Collidable, damageable::Damageable, movable::Movable,
        world_position::WorldPosition,
    },
    world::WorldParameters,
};

pub struct Movement {}

impl<'a> System<'a> for Movement {
    type SystemData = (
        Entities<'a>,
        WriteStorage<'a, Movable>,
        WriteStorage<'a, WorldPosition>,
        WriteStorage<'a, Armed>,
        ReadStorage<'a, Collidable>,
        ReadStorage<'a, Damageable>,
        Read<'a, WorldParameters>,
    );

    fn run(
        &mut self,
        (
            entities,
            mut movable,
            mut world_position,
            mut armed,
            collidable,
            damageable,
            world_parameters,
        ): Self::SystemData,
    ) {
        let mut collidable_map: HashMap<WorldPosition, Entity> = HashMap::new();

        for (entity, _, world_position) in (&entities, &collidable, &world_position).join() {
            collidable_map.insert(*world_position, entity.clone());
        }

        for (entity, movable, world_position, armed) in (
            &entities,
            &mut movable,
            &mut world_position,
            (&mut armed).maybe(),
        )
            .join()
        {
            if let Some(direction) = movable.unprocessed_move.take() {
                let new_world_position = world_position.moved(
                    direction,
                    world_parameters.width,
                    world_parameters.height,
                );
                if let Some(collided_with) = collidable_map.get(&new_world_position) {
                    match (armed, damageable.get(collided_with.clone())) {
                        (Some(armed), Some(_)) => {
                            armed.targetting = collided_with.clone().into();
                            info!("Targeting something?");
                        }
                        _ => {}
                    }
                } else {
                    collidable_map.remove(world_position);
                    *world_position = new_world_position;
                    collidable_map.insert(new_world_position, entity);
                }
            }
        }
    }
}
