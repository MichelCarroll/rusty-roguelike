use std::sync::Arc;

use specs::prelude::*;

use crate::game::{
    components::{damageable::Damageable, player_controlled::PlayerControlled},
    ui::game_ui::GameUI,
};

pub struct UI {
    pub ui_state: Arc<GameUI>,
}

impl<'a> System<'a> for UI {
    type SystemData = (
        ReadStorage<'a, PlayerControlled>,
        ReadStorage<'a, Damageable>,
    );

    fn run(&mut self, (player_controlled, damageable): Self::SystemData) {
        for (_, damageable) in (&player_controlled, &damageable).join() {
            self.ui_state.player_health.set(damageable.health)
        }
    }
}
