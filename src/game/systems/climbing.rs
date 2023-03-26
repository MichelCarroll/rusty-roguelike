use std::ops::Not;

use log::info;
use specs::prelude::*;

use crate::game::{
    components::{climbable::Climbable, level::Level, player_controlled::PlayerControlled, parent::Parent, sighted::{Sighted, self}},
    world::{WorldPosition, WorldPositionLookupTable},
};

pub struct Climbing {}

impl<'a> System<'a> for Climbing {
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, PlayerControlled>,
        ReadStorage<'a, Climbable>,
        WriteStorage<'a, WorldPosition>,
        WriteStorage<'a, Level>,
        WriteStorage<'a, Parent>,
        WriteStorage<'a, Sighted>,
        Write<'a, WorldPositionLookupTable>,
    );

    fn run(
        &mut self,
        (
            entities,
            player_controlled,
            climbable,
            mut world_position,
            mut level,
            mut parent,
            mut sighted,
            mut world_position_lookup_table,
        ): Self::SystemData,
    ) {
        let mut next_level = false;
        for (_, world_position) in (&player_controlled, &world_position).join() {
            if let Some(entities) = world_position_lookup_table
                .world_position_entities
                .get(world_position)
            {
                for entity in entities {
                    if climbable.get(*entity).is_some() {
                        next_level = true;
                    }
                }
            }
        }

        if next_level {

            let mut old_level: Option<(Entity, &mut Parent)> = None;
            for (entity, _, parent) in (&entities, &level, &mut parent).join() {
                old_level = (entity, parent).into();
            }
            
            if let Some((old_level_entity, old_entity_parent)) = old_level {

                entities.build_entity()
                    .with(Level::default(), &mut level)
                    .with(Parent { entity: old_entity_parent.entity }, &mut parent)
                    .build();
                
                entities.delete(old_level_entity).unwrap();

                for (entity, sighted, _) in (&entities, &mut sighted, &player_controlled).join() {
                    sighted.seen.clear();
                    sighted.seen_recently.clear();
                    world_position.remove(entity);
                }
                
                for (entity, entity_parent) in (&entities, &parent).join() {
                    if entity_parent.entity == old_level_entity {
                        entities.delete(entity).unwrap();
                        world_position_lookup_table.remove(entity);
                    }
                }
            }

            world_position_lookup_table.clear();
        }
    }
}
 