use dominator::{html, Dom, clone};
use futures_signals::signal::{Mutable, SignalExt};
use futures_signals::signal_vec::{MutableVec, SignalVecExt};

use std::sync::Arc;

pub struct GameUI {
    pub player_health: Mutable<u32>,
    pub inspected_entities: MutableVec<String>
}

impl GameUI {
    pub fn new() -> Arc<Self> {
        Arc::new(Self {
            player_health: Mutable::new(0),
            inspected_entities: MutableVec::new()
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

            .children(&mut [
                html!("ul", {
                    .class("inspection-result")
                    .children_signal_vec(state.inspected_entities.signal_vec_cloned()
                        .map(clone!(state => move |description| {
                            html!("li", {
                                .class("player-health")
                                .text(&description)
                            })
                        }))
                    )
                }),
            ])
        })
    }
}
