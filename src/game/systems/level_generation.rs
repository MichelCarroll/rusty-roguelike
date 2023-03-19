use specs::prelude::*;

use crate::{game::{ components::{movable::{Movable}, world_position::WorldPosition, level::Level, rendered::{Render, ZLayer}, player_controlled::PlayerControlled}}, common::Color};

pub struct LevelGeneration {}

impl<'a> System<'a> for LevelGeneration {
    type SystemData = (Entities<'a>, WriteStorage<'a, Level>, WriteStorage<'a, WorldPosition>, WriteStorage<'a, Render>, WriteStorage<'a, PlayerControlled>, WriteStorage<'a, Movable>);

    fn run(&mut self, (entities, mut level, mut world_position, mut render, mut player_controlled, mut movable): Self::SystemData) {
        for level in (&mut level).join() {
            if level.contents.is_empty() {

                let character_render = Render {
                    glyph: '@'.into(),
                    foreground_color: Color::new(210, 200, 180),
                    background_color: None,
                    z_layer: ZLayer::Creature
                };

                let floor_render = Render {
                    glyph: '.'.into(),
                    foreground_color: Color::new(210, 200, 180),
                    background_color: Color::new(40, 20, 120).into(),
                    z_layer: ZLayer::Ground
                };
                
                level.contents.push(
                    entities
                        .build_entity()
                        .with(WorldPosition { x: 2, y: 1 }, &mut world_position)
                        .with(character_render.clone(), &mut render)
                        .with(PlayerControlled::default(), &mut player_controlled)
                        .with(Movable::default(), &mut movable)
                        .build()
                );

                
                level.contents.push(
                    entities
                        .build_entity()
                        .with(WorldPosition { x: 1, y: 2 }, &mut world_position)
                        .with(floor_render.clone(), &mut render)
                        .build()
                );
            }
        }
    }
}
