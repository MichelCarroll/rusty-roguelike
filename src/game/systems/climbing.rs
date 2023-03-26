
use specs::prelude::*;

use crate::game::{components::{player_controlled::PlayerControlled, climbable::Climbable, level::Level}, world::{WorldPositionLookupTable, WorldPosition}};

pub struct Climbing {}

impl<'a> System<'a> for Climbing {
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, PlayerControlled>,
        ReadStorage<'a, Climbable>,
        ReadStorage<'a, WorldPosition>,
        WriteStorage<'a, Level>,
        Write<'a, WorldPositionLookupTable>,
    );

    fn run(
        &mut self,
        (entities, player_controlled, climbable, world_position, mut level, mut world_position_lookup_table): Self::SystemData,
    ) {
        let mut next_level = false;
        for (_, world_position) in (&player_controlled, &world_position).join() {
            if let Some(entities) = world_position_lookup_table.world_position_entities.get(world_position) {
                for entity in entities {
                    if climbable.get(*entity).is_some() {
                        next_level = true;
                    }
                }
            }
        }

        if next_level {
            for entity in (&entities).join() {
                entities.delete(entity).unwrap();
                world_position_lookup_table.remove(entity);
            }

            entities
                .build_entity()
                .with(Level::default(), &mut level)
                .build();
        }
    }
}
