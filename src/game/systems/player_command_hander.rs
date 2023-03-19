use specs::prelude::*;

use crate::game::{world::LastUserEvent, components::{movable::{Movable, Direction}, player_controlled::{PlayerControlled, PlayerCommand}}};

pub struct PlayerCommandHandler {}

impl<'a> System<'a> for PlayerCommandHandler {
    type SystemData = (ReadStorage<'a, PlayerControlled>, WriteStorage<'a, Movable>, Write<'a, LastUserEvent>);

    fn run(&mut self, (player_controlled, mut movable, mut last_user_event): Self::SystemData) {
        if let Some(user_event) = last_user_event.event.take() {
            for (_, movable) in (&player_controlled, &mut movable).join() {
                match user_event {
                    PlayerCommand::GoDown => {
                        movable.unprocessed_move = Direction::Down.into()
                    },
                    PlayerCommand::GoLeft => {
                        movable.unprocessed_move = Direction::Left.into()
                    },
                    PlayerCommand::GoRight => {
                        movable.unprocessed_move = Direction::Right.into()
                    },
                    PlayerCommand::GoUp => {
                        movable.unprocessed_move = Direction::Up.into()
                    }
                }
            }
        }
    }
}
