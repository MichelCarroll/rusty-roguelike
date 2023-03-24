use specs::prelude::*;

use crate::game::{
    common::{UIEvent, CanvasPosition},
    components::{ 
        movable::{Direction, Movable},
        player_controlled::PlayerControlled,
    },
    world::{LastUserEvent, WorldTime, UIState},
};

pub struct PlayerCommandHandler {}

impl<'a> System<'a> for PlayerCommandHandler {
    type SystemData = (
        ReadStorage<'a, PlayerControlled>,
        WriteStorage<'a, Movable>,
        Write<'a, LastUserEvent>,
        Write<'a, WorldTime>,
        Write<'a, UIState>,
    );

    fn run(&mut self, (player_controlled, mut movable, mut last_user_event, mut world_time, mut ui_state): Self::SystemData) {
        if let Some(user_event) = last_user_event.event.take() {
            for (_, movable) in (&player_controlled, &mut movable).join() {
                match user_event {
                    UIEvent::Down => {
                        movable.unprocessed_move = Direction::Down.into();
                        world_time.tick += 1;
                    },
                    UIEvent::Left => {
                        movable.unprocessed_move = Direction::Left.into();
                        world_time.tick += 1;
                    },
                    UIEvent::Right => {
                        movable.unprocessed_move = Direction::Right.into();
                        world_time.tick += 1;
                    },
                    UIEvent::Up => {
                        movable.unprocessed_move = Direction::Up.into();
                        world_time.tick += 1;
                    },
                    UIEvent::MouseOver(x, y) => {
                        ui_state.mouse_over = CanvasPosition { x, y }.into()
                    },
                    UIEvent::MousePress(x, y) => {
                        
                    },
                }
            }
        }
    }
}
