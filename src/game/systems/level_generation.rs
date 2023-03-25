use std::collections::HashSet;

use specs::prelude::*;

use crate::game::{
    common::Color,
    components::{
        ai_controlled::AIControlled,
        armed::Armed,
        collidable::Collidable,
        damageable::Damageable,
        describable::Describable,
        factioned::{Faction, Factioned},
        inventoried::Inventoried,
        level::Level,
        movable::{Direction, Movable},
        opaque::Opaque,
        pickupable::Pickupable,
        player_controlled::PlayerControlled,
        rendered::{Render, ZLayer},
        sighted::Sighted,
    },
    random::{random_in_range, random_in_vec_and_remove},
    world::{WorldParameters, WorldPosition, WorldPositionLookupTable},
};

pub struct LevelGeneration {}

impl<'a> System<'a> for LevelGeneration {
    type SystemData = (
        Entities<'a>,
        Read<'a, WorldParameters>,
        Write<'a, WorldPositionLookupTable>,
        WriteStorage<'a, Level>,
        WriteStorage<'a, WorldPosition>,
        WriteStorage<'a, Render>,
        WriteStorage<'a, PlayerControlled>,
        WriteStorage<'a, Movable>,
        WriteStorage<'a, Collidable>,
        WriteStorage<'a, Pickupable>,
        WriteStorage<'a, Inventoried>,
        WriteStorage<'a, Factioned>,
        WriteStorage<'a, AIControlled>,
        WriteStorage<'a, Damageable>,
        WriteStorage<'a, Armed>,
        WriteStorage<'a, Sighted>,
        WriteStorage<'a, Opaque>,
        WriteStorage<'a, Describable>,
    );

    fn run(
        &mut self,
        (
            entities,
            world_parameters,
            mut world_position_lookup_table,
            mut level,
            mut world_position,
            mut render,
            mut player_controlled,
            mut movable,
            mut collidable,
            mut pickupable,
            mut inventoried,
            mut factioned,
            mut ai_controlled,
            mut damageable,
            mut armed,
            mut sighted,
            mut opaque,
            mut describable,
        ): Self::SystemData,
    ) {
        for level in (&mut level).join() {
            if level.contents.is_empty() {
                let floor_render = Render {
                    glyph: '.'.into(),
                    foreground_color: Color::mildew(),
                    background_color: Color::brown().into(),
                    z_layer: ZLayer::Ground,
                };

                let stone_render = Render {
                    glyph: '#'.into(),
                    foreground_color: Color::brown(),
                    background_color: Color::black().into(),
                    z_layer: ZLayer::Saturating,
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
                        *pos = pos.moved(
                            Direction::random(),
                            world_parameters.width,
                            world_parameters.height,
                        );
                        carved.insert(*pos);
                    }
                }

                for x in 0..world_parameters.width {
                    for y in 0..world_parameters.height {
                        let position = WorldPosition { x, y };
                        if carved.contains(&position) {
                            let entity = entities
                                .build_entity()
                                .with(WorldPosition { x, y }, &mut world_position)
                                .with(
                                    Describable {
                                        description: "Floor".to_owned(),
                                    },
                                    &mut describable,
                                )
                                .with(floor_render.clone(), &mut render)
                                .build();
                            level.contents.push(entity);
                            world_position_lookup_table.update(entity, WorldPosition { x, y });
                        } else {
                            let entity = entities
                                .build_entity()
                                .with(WorldPosition { x, y }, &mut world_position)
                                .with(
                                    Describable {
                                        description: "Stone Wall".to_owned(),
                                    },
                                    &mut describable,
                                )
                                .with(stone_render.clone(), &mut render)
                                .with(Collidable {}, &mut collidable)
                                .with(Opaque::default(), &mut opaque)
                                .build();
                            level.contents.push(entity);
                            world_position_lookup_table.update(entity, WorldPosition { x, y });
                        }
                    }
                }

                let mut all_carved: Vec<_> = carved.iter().collect();

                if let Some(&player_position) = random_in_vec_and_remove(&mut all_carved) {
                    let character_render = Render {
                        glyph: '@'.into(),
                        foreground_color: Color::black(),
                        background_color: None,
                        z_layer: ZLayer::Creature,
                    };

                    let entity = entities
                        .build_entity()
                        .with(player_position.clone(), &mut world_position)
                        .with(character_render.clone(), &mut render)
                        .with(
                            Describable {
                                description: "Player".to_owned(),
                            },
                            &mut describable,
                        )
                        .with(PlayerControlled::default(), &mut player_controlled)
                        .with(Movable::default(), &mut movable)
                        .with(Inventoried::default(), &mut inventoried)
                        .with(
                            Factioned {
                                faction: Faction::Player,
                            },
                            &mut factioned,
                        )
                        .with(Collidable {}, &mut collidable)
                        .with(
                            Damageable {
                                health: 100,
                                max_health: 100,
                            },
                            &mut damageable,
                        )
                        .with(
                            Armed {
                                damage: 5,
                                targetting: None,
                            },
                            &mut armed,
                        )
                        .with(Sighted::default(), &mut sighted)
                        .build();
                    level.contents.push(entity);
                    world_position_lookup_table.update(entity, player_position);
                }

                let item_render = Render {
                    glyph: '$'.into(),
                    foreground_color: Color::yellow(),
                    background_color: None,
                    z_layer: ZLayer::Item,
                };

                for _ in 0..10 {
                    if let Some(&item_position) = random_in_vec_and_remove(&mut all_carved) {
                        let entity = entities
                            .build_entity()
                            .with(item_position.clone(), &mut world_position)
                            .with(item_render.clone(), &mut render)
                            .with(
                                Describable {
                                    description: "Gold".to_owned(),
                                },
                                &mut describable,
                            )
                            .with(Pickupable::default(), &mut pickupable)
                            .build();
                        level.contents.push(entity);
                        world_position_lookup_table.update(entity, item_position);
                    }
                }

                let monster_render = Render {
                    glyph: 'm'.into(),
                    foreground_color: Color::deep_red(),
                    background_color: None,
                    z_layer: ZLayer::Creature,
                };

                for _ in 0..5 {
                    if let Some(&monster_position) = random_in_vec_and_remove(&mut all_carved) {
                        let entity = entities
                            .build_entity()
                            .with(monster_position.clone(), &mut world_position)
                            .with(monster_render.clone(), &mut render)
                            .with(
                                Describable {
                                    description: "Monster".to_owned(),
                                },
                                &mut describable,
                            )
                            .with(AIControlled::default(), &mut ai_controlled)
                            .with(Movable::default(), &mut movable)
                            .with(Inventoried::default(), &mut inventoried)
                            .with(
                                Factioned {
                                    faction: Faction::Enemy,
                                },
                                &mut factioned,
                            )
                            .with(Collidable {}, &mut collidable)
                            .with(
                                Damageable {
                                    health: 10,
                                    max_health: 10,
                                },
                                &mut damageable,
                            )
                            .with(
                                Armed {
                                    damage: 1,
                                    targetting: None,
                                },
                                &mut armed,
                            )
                            .build();
                        level.contents.push(entity);
                        world_position_lookup_table.update(entity, monster_position);
                    }
                }
            }
        }
    }
}
