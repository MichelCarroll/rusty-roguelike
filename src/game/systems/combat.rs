use log::info;
use specs::prelude::*;

use crate::game::{
    components::{armed::Armed, damageable::Damageable, factioned::Factioned},
    world::WorldPositionLookupTable,
};

pub struct Combat {}

impl<'a> System<'a> for Combat {
    type SystemData = (
        Entities<'a>,
        Write<'a, WorldPositionLookupTable>,
        WriteStorage<'a, Armed>,
        WriteStorage<'a, Damageable>,
    );

    fn run(
        &mut self,
        (entities, mut world_position_lookup_table, mut armed, mut damageable): Self::SystemData,
    ) {
        for (armed_entity, armed) in (&entities, &mut armed).join() {
            if let Some(target) = armed.targetting.take() {
                if target == armed_entity {
                    continue;
                }
                if let Some(damage) = damageable.get_mut(target) {
                    damage.health = damage.health.checked_sub(armed.damage).unwrap_or(0);
                    if damage.health == 0 {
                        entities.delete(target).unwrap();
                        world_position_lookup_table.remove(target);
                    }
                }
            }
        }
    }
}
