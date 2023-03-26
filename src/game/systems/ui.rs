use std::sync::Arc;

use specs::prelude::*;

use crate::game::{
    components::{
        damageable::Damageable, describable::Describable, player_controlled::PlayerControlled,
        sighted::Sighted, parent::Parent,
    },
    ui::game_ui::GameUI,
    world::{UIState, WorldPosition, WorldPositionLookupTable}, hierarchy::Hierarchy,
};

pub struct UI {
    pub ui_state: Arc<GameUI>,
    pub last_mouse_over_position: Option<WorldPosition>,
    pub past_children_bitset: Option<BitSet>
}

impl<'a> System<'a> for UI {
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, PlayerControlled>,
        ReadStorage<'a, Damageable>,
        ReadStorage<'a, Describable>,
        ReadStorage<'a, Sighted>,
        Read<'a, UIState>,
        Read<'a, WorldPositionLookupTable>,
        ReadExpect<'a, Hierarchy<Parent>>,
    );

    fn run(
        &mut self,
        (
            entities,
            player_controlled,
            damageable,
            describable,
            sighted,
            ui_state,
            world_position_lookup_table,
            hierarchy
        ): Self::SystemData,
    ) {
        for (entity, _, damageable) in (&entities, &player_controlled, &damageable).join() {
            self.ui_state.player_health.set(damageable.health);
            let new_child_bitset = hierarchy.all_children(entity);
            let swap = self.past_children_bitset.clone().map(|b| b != new_child_bitset).unwrap_or(true);
            if swap {
                let mut lock = self.ui_state.inventory_entities.lock_mut();
                lock.clear();
                for (_, description) in (new_child_bitset.clone(), &describable).join() {
                    lock.push_cloned(description.description.clone());
                }
                self.past_children_bitset = new_child_bitset.clone().into();
            }
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
