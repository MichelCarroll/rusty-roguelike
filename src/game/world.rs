use std::collections::{HashMap, HashSet};
use specs::{Entity, World};

use super::common::{CanvasPosition, CanvasSize, UIEvent};

pub const CELL_SIZE: f64 = 50.0;

#[derive(Default)]
pub struct LastUserEvent {
    pub events: Vec<UIEvent>,
}

#[derive(Debug, Default)]
pub struct WorldParameters {
    pub width: u64,
    pub height: u64,
}

impl WorldParameters {
    pub fn max_position(&self) -> WorldPosition {
        WorldPosition {
            x: self.width - 1,
            y: self.height - 1,
        }
    }

    pub fn from_canvas_size(canvas_size: CanvasSize) -> WorldParameters {
        WorldParameters {
            width: (canvas_size.width as f64 / CELL_SIZE) as u64,
            height: (canvas_size.height as f64 / CELL_SIZE) as u64,
        }
    }
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct WorldPosition {
    pub x: u64,
    pub y: u64,
}

#[derive(Default)]
pub struct WorldTime {
    pub tick: u64,
}

#[derive(Default)]
pub struct UIState {
    pub mouse_over: Option<CanvasPosition>,
}

#[derive(Default)]
pub struct WorldPositionLookupTable {
    pub world_position_entities: HashMap<WorldPosition, HashSet<Entity>>,
    pub entity_world_position: HashMap<Entity, WorldPosition>
}  

impl WorldPositionLookupTable {
    pub fn update(&mut self, entity: Entity, new_world_position: WorldPosition) {
        let old_world_position = self.entity_world_position.get(&entity).map(|p| p.clone());
        if let Some(old_world_position) = old_world_position {
            if new_world_position == old_world_position {
                return
            }
            self.entity_world_position.insert(entity, new_world_position);
            if let Some(hash_map) = self.world_position_entities.get_mut(&new_world_position) {
                hash_map.insert(entity);
            }
            else {
                let mut new_hash_set = HashSet::new();
                new_hash_set.insert(entity);
                self.world_position_entities.insert(new_world_position, new_hash_set);
            }

            if let Some(hash_map) = self.world_position_entities.get_mut(&old_world_position) {
                hash_map.remove(&entity);
            }
        }
        else {
            self.entity_world_position.insert(entity, new_world_position);
            let mut new_hash_set = HashSet::new();
            new_hash_set.insert(entity);
            self.world_position_entities.insert(new_world_position, new_hash_set);
        }
    }

    pub fn remove(&mut self, entity: Entity) {
        let old_world_position = self.entity_world_position.get(&entity).map(|p| p.clone());
        self.entity_world_position.remove(&entity);
        if let Some(old_world_position) = old_world_position {
            if let Some(entities_in_position) = self.world_position_entities.get_mut(&old_world_position) {
                entities_in_position.remove(&entity);
            }
        }
    }
}