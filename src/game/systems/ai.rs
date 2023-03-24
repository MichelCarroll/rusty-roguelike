use specs::prelude::*;

use crate::game::{
    components::{
        ai_controlled::AIControlled,
        factioned::{Faction, Factioned},
        movable::{Direction, Movable},
    }, world::{WorldPosition, WorldTime},
};

#[derive(Default)]
pub struct AI {
    pub last_tick: u64
}

impl<'a> System<'a> for AI {
    type SystemData = (
        ReadStorage<'a, AIControlled>,
        ReadStorage<'a, Factioned>,
        ReadStorage<'a, WorldPosition>,
        WriteStorage<'a, Movable>,
        Read<'a, WorldTime>
    );

    fn run(&mut self, (ai_controlled, factioned, world_position, mut movable, world_time): Self::SystemData) {
        if self.last_tick >= world_time.tick {
            return 
        }

        let first_player_position = (&factioned, &world_position)
            .join()
            .filter_map(|(factioned, world_position)| match factioned.faction {
                Faction::Player => Some(world_position),
                _ => None,
            })
            .next();

        if let Some(player_position) = first_player_position {
            for (factioned, world_position, _, movable) in
                (&factioned, &world_position, &ai_controlled, &mut movable).join()
            {
                if factioned.faction == Faction::Enemy {
                    let delta_x = (player_position.x as f64) - (world_position.x as f64);
                    let delta_y = (player_position.y as f64) - (world_position.y as f64);
                    let rads = (delta_y).atan2(delta_x);
                    movable.unprocessed_move = Direction::from_radians(rads).into();
                }
            }
        }

        self.last_tick = world_time.tick;
    }
}
