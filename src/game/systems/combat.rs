use specs::prelude::*;

use crate::game::{ components::{armed::{Armed}, damageable::Damageable}};

pub struct Combat {}

impl<'a> System<'a> for Combat {
    type SystemData = (
        Entities<'a>, 
        WriteStorage<'a, Armed>, 
        WriteStorage<'a, Damageable>
    );

    fn run(&mut self, (mut entities, mut armed, mut damageable): Self::SystemData) {
        for armed in (&mut armed).join() {
            if let Some(target) = armed.targetting.take() {
                if let Some(damage) = damageable.get_mut(target) {
                    damage.health = damage.health.checked_sub(armed.damage).unwrap_or(0);
                    if damage.health == 0 {
                        entities.delete(target).unwrap();
                    }
                }
            }
        }   
    }
}
  