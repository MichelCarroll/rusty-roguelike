use specs::prelude::*;

use crate::game::{
    common::UIEvent,
    components::{
        movable::{Direction, Movable},
        player_controlled::PlayerControlled,
    },
    world::{LastUserEvent, UIState, WorldTime},
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

    fn run(
        &mut self,
        (player_controlled, mut movable, mut last_user_event, mut world_time, mut ui_state): Self::SystemData,
    ) {
        for user_event in last_user_event.events.iter() {
            for (_, movable) in (&player_controlled, &mut movable).join() {
                match user_event {
                    UIEvent::Down => {
                        movable.unprocessed_move = Direction::Down.into();
                        world_time.tick += 1;
                    }
                    UIEvent::Left => {
                        movable.unprocessed_move = Direction::Left.into();
                        world_time.tick += 1;
                    }
                    UIEvent::Right => {
                        movable.unprocessed_move = Direction::Right.into();
                        world_time.tick += 1;
                    }
                    UIEvent::Up => {
                        movable.unprocessed_move = Direction::Up.into();
                        world_time.tick += 1;
                    }
                    UIEvent::MouseOver(canvas_position) => {
                        ui_state.mouse_over = (*canvas_position).into();
                    }
                    UIEvent::MousePress(_) => {},
                    UIEvent::MouseLeave => {
                        ui_state.mouse_over = None
                    },
                }
            }
        }
        last_user_event.events.clear();
    }
}
