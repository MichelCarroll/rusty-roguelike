use std::collections::HashMap;

use specs::prelude::*;

use crate::game::{
    components::{inventoried::Inventoried, pickupable::Pickupable, parent::Parent},
    world::{WorldPosition, WorldPositionLookupTable},
};

pub struct Looting {}

impl<'a> System<'a> for Looting {
    type SystemData = (
        Entities<'a>,
        Write<'a, WorldPositionLookupTable>,
        ReadStorage<'a, Pickupable>,
        WriteStorage<'a, WorldPosition>,
        ReadStorage<'a, Inventoried>,
        WriteStorage<'a, Parent>
    );

    fn run(
        &mut self,
        (
            entities,
            mut world_position_lookup_table,
            pickupable,
            mut world_position,
            inventoried,
            mut parent
        ): Self::SystemData,
    ) {
        let mut items_to_process: Vec<Entity> = vec![];

        for (inventoried_entity, _, inventoried_world_position) in (&entities, &inventoried, &world_position).join()
        {
            if let Some(entities) = world_position_lookup_table.world_position_entities.get(&inventoried_world_position) {
                for entity in entities {
                    match (pickupable.get(*entity), parent.get_mut(*entity)) {
                        (Some(_), Some(parent)) => {
                            items_to_process.push(*entity);
                            parent.entity = inventoried_entity;
                        },
                        _ => {}
                    }
                }
            }
        }

        for item in items_to_process {
            world_position.remove(item);
            world_position_lookup_table.remove(item);
        }
    }
}
