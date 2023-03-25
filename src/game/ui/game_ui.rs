use dominator::{html, Dom};
use futures_signals::signal::{Mutable, SignalExt};
use std::sync::Arc;

pub struct GameUI {
    pub player_health: Mutable<u32>,
}

impl GameUI {
    pub fn new() -> Arc<Self> {
        Arc::new(Self {
            player_health: Mutable::new(0),
        })
    }

    pub fn render(state: &Arc<Self>) -> Dom {
        html!("div", {
            .class("game-ui-root")

            .children(&mut [
                html!("canvas", {
                    .class("game-canvas")
                }),
            ])

            .children(&mut [
                html!("div", {
                    .class("player-health")
                    .text_signal(state.player_health.signal().map(|x| format!("Player Health: {}", x)))
                }),
            ])
        })
    }
}
