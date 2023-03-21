use specs::prelude::*;

use crate::game::{
    common::Command,
    components::{
        movable::{Direction, Movable},
        player_controlled::PlayerControlled,
    },
    world::LastUserEvent,
};

pub struct PlayerCommandHandler {}

impl<'a> System<'a> for PlayerCommandHandler {
    type SystemData = (
        ReadStorage<'a, PlayerControlled>,
        WriteStorage<'a, Movable>,
        Write<'a, LastUserEvent>,
    );

    fn run(&mut self, (player_controlled, mut movable, mut last_user_event): Self::SystemData) {
        if let Some(user_event) = last_user_event.event.take() {
            for (_, movable) in (&player_controlled, &mut movable).join() {
                match user_event {
                    Command::GoDown => movable.unprocessed_move = Direction::Down.into(),
                    Command::GoLeft => movable.unprocessed_move = Direction::Left.into(),
                    Command::GoRight => movable.unprocessed_move = Direction::Right.into(),
                    Command::GoUp => movable.unprocessed_move = Direction::Up.into(),
                }
            }
        }
    }
}
