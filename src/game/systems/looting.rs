use std::collections::HashMap;

use specs::prelude::*;

use crate::game::{ components::{world_position::{WorldPosition, self}, pickupable::Pickupable, inventoried::Inventoried}};

pub struct Looting {}

impl<'a> System<'a> for Looting {
    type SystemData = (
        Entities<'a>, 
        ReadStorage<'a, Pickupable>, 
        WriteStorage<'a, WorldPosition>, 
        WriteStorage<'a, Inventoried>
    );

    fn run(&mut self, (entities, pickupable, mut world_position, mut inventoried): Self::SystemData) {
        let mut item_map: HashMap<WorldPosition, Vec<Entity>> = HashMap::new();

        for (entity, _, item_world_position) in (&entities, &pickupable, &world_position).join() {
            if let Some(items) = item_map.get_mut(item_world_position) {
                items.push(entity);
            }
            else {
                item_map.insert(*item_world_position, vec![entity]);
            }
        }

        let mut items_to_process: Vec<Entity> = vec![];

        for (inventoried, inventoried_world_position) in (&mut inventoried, &mut world_position).join() {
            if let Some(items) = item_map.remove(&inventoried_world_position) {
                for item in items {
                    items_to_process.push(item);
                    inventoried.contents.push(item);
                }
            }
        }

        for item in items_to_process {
            world_position.remove(item);
        }
        
    }
}
