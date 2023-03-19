use std::collections::HashSet;

use js_sys::Math::random;
use specs::prelude::*;

use crate::{game::{ components::{movable::{Movable, Direction}, world_position::WorldPosition, level::Level, rendered::{Render, ZLayer}, player_controlled::PlayerControlled}, common::Color, world::WorldParameters, random::{random_in_range, random_in_vec}}};

pub struct LevelGeneration {}

impl<'a> System<'a> for LevelGeneration {
    type SystemData = (
        Entities<'a>, 
        Read<'a, WorldParameters>,
        WriteStorage<'a, Level>, 
        WriteStorage<'a, WorldPosition>,
        WriteStorage<'a, Render>, 
        WriteStorage<'a, PlayerControlled>, 
        WriteStorage<'a, Movable>
    );

    fn run(&mut self, (entities, world_parameters, mut level, mut world_position, mut render, mut player_controlled, mut movable): Self::SystemData) {
        for level in (&mut level).join() {
            if level.contents.is_empty() {

                let floor_render = Render {
                    glyph: '.'.into(),
                    foreground_color: Color::MILDEW(),
                    background_color: Color::BROWN().into(),
                    z_layer: ZLayer::Ground
                };

                let stone_render = Render {
                    glyph: '#'.into(),
                    foreground_color: Color::BROWN(),
                    background_color: Color::BLACK().into(),
                    z_layer: ZLayer::Saturating
                };


                let mut automata: Vec<WorldPosition> = vec![];
                let mut carved = HashSet::<WorldPosition>::new();

                for _ in 0..3 {
                    let x = random_in_range(0, world_parameters.width);
                    let y = random_in_range(0, world_parameters.height);
                    let position = WorldPosition { x, y };
                    automata.push(position);
                    carved.insert(position);
                }

                for _ in 0..1000 {
                    for pos in automata.iter_mut() {
                        *pos = pos.moved(Direction::random(), world_parameters.width, world_parameters.height);
                        carved.insert(*pos); 
                    }
                }

                for x in 0..world_parameters.width {
                    for y in 0..world_parameters.height {
                        let position = WorldPosition { x, y };
                        if carved.contains(&position) {
                            level.contents.push(
                                entities
                                    .build_entity()
                                    .with(WorldPosition { x, y }, &mut world_position)
                                    .with(floor_render.clone(), &mut render)
                                    .build()
                            )
                        }
                        else {
                            level.contents.push(
                                entities
                                    .build_entity()
                                    .with(WorldPosition { x, y }, &mut world_position)
                                    .with(stone_render.clone(), &mut render)
                                    .build()
                            )
                        }
                    }
                }

                let all_carved: Vec<_> = carved.iter().collect();
                if let Some(&player_position) = random_in_vec(&all_carved) {

                    let character_render = Render {
                        glyph: '@'.into(),
                        foreground_color: Color::BLACK(),
                        background_color: None,
                        z_layer: ZLayer::Creature
                    }; 
    
                    level.contents.push(
                        entities
                            .build_entity()
                            .with(player_position.clone(), &mut world_position)
                            .with(character_render.clone(), &mut render)
                            .with(PlayerControlled::default(), &mut player_controlled)
                            .with(Movable::default(), &mut movable)
                            .build()
                    );
                }
                
            }
        }
    }
}
