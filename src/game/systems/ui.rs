use std::sync::Arc;

use specs::prelude::*;

use crate::game::{
    components::{
        damageable::Damageable, describable::Describable, player_controlled::PlayerControlled,
        sighted::Sighted,
    },
    ui::game_ui::GameUI,
    world::{UIState, WorldPosition, WorldPositionLookupTable},
};

pub struct UI {
    pub ui_state: Arc<GameUI>,
    pub last_mouse_over_position: Option<WorldPosition>,
}

impl<'a> System<'a> for UI {
    type SystemData = (
        ReadStorage<'a, PlayerControlled>,
        ReadStorage<'a, Damageable>,
        ReadStorage<'a, Describable>,
        ReadStorage<'a, Sighted>,
        Read<'a, UIState>,
        Read<'a, WorldPositionLookupTable>,
    );

    fn run(
        &mut self,
        (
            player_controlled,
            damageable,
            describable,
            sighted,
            ui_state,
            world_position_lookup_table,
        ): Self::SystemData,
    ) {
        for (_, damageable) in (&player_controlled, &damageable).join() {
            self.ui_state.player_health.set(damageable.health)
        }
        let mouse_position = ui_state.mouse_over_position();
        if mouse_position == self.last_mouse_over_position {
            return;
        }

        let mut lock = self.ui_state.inspected_entities.lock_mut();
        lock.clear();

        for (_, sighted) in (&player_controlled, &sighted).join() {
            if let Some(mouse_position) = mouse_position {
                if let Some(entities) = world_position_lookup_table
                    .world_position_entities
                    .get(&mouse_position)
                {
                    for entity in entities {
                        if !sighted.seen.contains(entity.id())
                            && !sighted.seen_recently.contains(entity.id())
                        {
                            continue;
                        }
                        if let Some(description) = describable.get(*entity) {
                            lock.push_cloned(description.description.clone());
                        }
                    }
                }
                self.last_mouse_over_position = mouse_position.into();
            }
        }
    }
}
