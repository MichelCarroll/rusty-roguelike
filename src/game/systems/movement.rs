use std::collections::HashMap;

use log::info;
use specs::prelude::*;

use crate::game::{
    components::{armed::Armed, collidable::Collidable, damageable::Damageable, movable::Movable},
    world::{WorldParameters, WorldPosition, WorldPositionLookupTable},
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
        Write<'a, WorldPositionLookupTable>,
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
            mut world_position_lookup_table
        ): Self::SystemData,
    ) {
        for (entity, movable, mut world_position, armed) in (
            &entities,
            &mut movable,
            &mut world_position,
            (&mut armed).maybe(),
        )
            .join()
        {
            self.apply_movement(entity, movable, world_position, &world_parameters, &mut world_position_lookup_table, &collidable, &damageable, armed);
        }
    }
}

impl Movement {

    fn apply_movement(&mut self, entity: Entity, movable: &mut Movable, world_position: &mut WorldPosition, world_parameters: &Read<WorldParameters>, world_position_lookup_table: &mut WorldPositionLookupTable, collidable: &ReadStorage<Collidable>, damageable: &ReadStorage<Damageable>, armed: Option<&mut Armed>) {
        if let Some(direction) = movable.unprocessed_move.take() {
            let new_world_position = world_position.moved(
                direction,
                world_parameters.width,
                world_parameters.height,
            );
            if let Some(entities) = world_position_lookup_table.world_position_entities.get(&new_world_position) {
                for entity in entities {
                    match (collidable.get(*entity), damageable.get(*entity)) {
                        (Some(_), Some(_)) => {
                            if let Some(armed) = armed {
                                armed.targetting = (*entity).into();
                            }
                            return;
                        }
                        (Some(_), None) => {
                            return;
                        }
                        _ => {}
                    }
                }
            }
            *world_position = new_world_position;
            world_position_lookup_table.update(entity, new_world_position);
        }
    }

}