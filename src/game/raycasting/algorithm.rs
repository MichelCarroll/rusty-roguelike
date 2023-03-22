use crate::game::world::WorldPosition;
use std::f64::consts::PI;

struct RealPosition {
    x: f64,
    y: f64
}

struct DiscreteStep {
    x: i32,
    y: i32
}

pub struct Raycast {
    steps: DiscreteStep,
    current_world_position: WorldPosition,
    t_deltas: RealPosition,
    t_max: RealPosition,
    max_position: WorldPosition,
}

impl Raycast {

    pub fn new(world_position: WorldPosition, max_position: WorldPosition, radians: f64) -> Self {
        let step_x: i32 = if radians > -PI / 2.0 && radians < PI / 2.0 { 1 } else { -1 }; 
        let step_y: i32 = if radians > 0.0 { 1 } else { -1 }; 
        let t_delta_x = (1.0 / radians.cos()).abs();
        let t_delta_y = (1.0 / radians.sin()).abs();
        let t_max_x = t_delta_x / 2.0;
        let t_max_y = t_delta_y / 2.0;

        Raycast { 
            steps: DiscreteStep { x: step_x, y: step_y },
            current_world_position: world_position,
            t_deltas: RealPosition { x: t_delta_x, y: t_delta_y },
            t_max: RealPosition { x: t_max_x, y: t_max_y },
            max_position
        }
    }

}

impl Iterator for Raycast {
    type Item = WorldPosition;

    fn next(&mut self) -> Option<Self::Item> {
        if self.t_max.x < self.t_max.y {
            self.t_max.x += self.t_deltas.x;

            match (self.current_world_position.x as i32).checked_add(self.steps.x) {
                Some(new_position) => {
                    if new_position > (self.max_position.x as i32) {
                        None 
                    }
                    else {
                        self.current_world_position.x = new_position as u64;
                        Some(self.current_world_position)
                    }
                },
                None => {
                    None
                },
            }
        }
        else {
            self.t_max.y += self.t_deltas.y;

            match (self.current_world_position.y as i32).checked_add(self.steps.y) {
                Some(new_position) => {
                    if new_position > (self.max_position.y as i32) {
                        None 
                    }
                    else {
                        self.current_world_position.y = new_position as u64;
                        Some(self.current_world_position)
                    }
                },
                None => {
                    None
                },
            }
        }
    }
}