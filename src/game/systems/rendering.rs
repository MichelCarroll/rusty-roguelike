use std::collections::HashMap;

use specs::prelude::*;
use web_sys::CanvasRenderingContext2d;

use crate::game::{
    common::{CanvasSize, Color, CanvasPosition},
    components::{
        player_controlled::PlayerControlled, rendered::Render, sighted::Sighted, damageable::Damageable,
    }, world::{WorldPosition, UIState},
};

const CELL_SIZE: f64 = 50.0;
const BACKGROUND_COLOR: &str = "#000000";

pub struct Rendering {
    pub canvas_size: CanvasSize,
    pub rendering_context: CanvasRenderingContext2d
}

struct RenderTarget<'a> {
    glyph: Option<(char, Color)>,
    background_color: Option<Color>,
    damageable: Option<&'a Damageable>,
    semi_renderable: bool
}

// Safe because wasm is always strictly single-threaded
unsafe impl Send for Rendering {}

impl Rendering {

    fn add_to_render_targets<'a>(hash_map: &mut HashMap<WorldPosition, RenderTarget<'a>>, render: &Render, pos: &WorldPosition, damageable: Option<&'a Damageable>, semi_renderable: bool) {
        let new_glyph = render.glyph.map(|g| (g, render.foreground_color.clone()));
        if let Some(render_target) = hash_map.get_mut(pos) {
            if let Some(new_glyph) = new_glyph {
                render_target.glyph = new_glyph.into();
            }
            render_target.damageable = damageable;
            render_target.background_color = render
                .background_color
                .clone()
                .or(render_target.background_color.clone());
        } else {
            hash_map.insert(
                pos.clone(),
                RenderTarget {
                    glyph: new_glyph,
                    background_color: render.background_color.clone(),
                    damageable,
                    semi_renderable
                },
            );
        }
    }
}

impl<'a> System<'a> for Rendering {
    type SystemData = (
        ReadStorage<'a, WorldPosition>,
        ReadStorage<'a, Render>,
        ReadStorage<'a, PlayerControlled>,
        ReadStorage<'a, Sighted>,
        ReadStorage<'a, Damageable>,
        Read<'a, UIState>
    );
    

    fn run(&mut self, (pos, render, player_controlled, sighted, damageable, ui_state): Self::SystemData) {
        let x_text_offset = CELL_SIZE / 2.0;
        let y_text_offset = CELL_SIZE / 2.0;
        self.rendering_context.set_font("bold 44px Arial");
        self.rendering_context.set_text_baseline("middle");
        self.rendering_context.set_text_align("center");
        self.rendering_context
            .set_fill_style(&BACKGROUND_COLOR.into());
        self.rendering_context
            .fill_rect(0.0, 0.0, self.canvas_size.width, self.canvas_size.height);

        if let Some((_, sighted)) = (&player_controlled, &sighted)
            .join()
            .collect::<Vec<_>>()
            .first()
        {
            let mut renderable = (&pos, &render, (&damageable).maybe(), &sighted.seen).join().collect::<Vec<_>>();
            let semi_renderable = (&pos, &render, !&sighted.seen, &sighted.seen_recently).join().collect::<Vec<_>>();
            renderable.sort_by(|a, b| a.1.z_layer.cmp(&b.1.z_layer));

            let mut hash_map: HashMap<WorldPosition, RenderTarget> = HashMap::new();

            for (pos, render, health, _) in renderable {
                Rendering::add_to_render_targets(&mut hash_map, render, pos, health, false);
            }

            for (pos, render, _, _) in semi_renderable {
                Rendering::add_to_render_targets(&mut hash_map, render, pos, None, true);
            }

            for (pos, render_target) in hash_map {
                let x = CELL_SIZE * pos.x as f64;
                let y = CELL_SIZE * pos.y as f64;

                let hovered = match ui_state.mouse_over {
                    Some(CanvasPosition { x: mouse_x, y: mouse_y }) => {
                        let mouse_x = mouse_x as f64;
                        let mouse_y = mouse_y as f64;
                        mouse_x > x && mouse_x < x + CELL_SIZE && mouse_y > y && mouse_y < y + CELL_SIZE
                    },
                    None => false,
                };

                if let Some(background_color) = &render_target.background_color {
                    let mut color = *background_color;
                    if hovered {
                        color = color.tinted();
                    }
                    else if render_target.semi_renderable { 
                        color = color.darkened();
                    }
                    self.rendering_context
                        .set_fill_style(&(color.to_string().into()));
                    self.rendering_context.fill_rect(x, y, CELL_SIZE, CELL_SIZE);
                }

                if let Some((glyph, foreground_color)) = render_target.glyph {
                    let mut color = foreground_color;
                    if hovered {
                        color = color.tinted();
                    }
                    else if render_target.semi_renderable { 
                        color = color.darkened();
                    }
                    self.rendering_context
                        .set_fill_style(&(color.to_string().into()));
                    self.rendering_context
                        .fill_text(&glyph.to_string(), x + x_text_offset, y + y_text_offset)
                        .unwrap();
                }

                if let Some(damageable) = render_target.damageable {
                    if damageable.max_health != damageable.health {
                        self.rendering_context
                            .set_stroke_style(&(Color::black().to_string().into()));
                        self.rendering_context
                            .set_fill_style(&(Color::bright_red().to_string().into()));
                        let fill_width = CELL_SIZE * (damageable.health as f64 / damageable.max_health as f64);
                        let health_bar_height = CELL_SIZE / 6.0;
                        let bar_y = y + CELL_SIZE - health_bar_height;
                        self.rendering_context.stroke_rect(x, bar_y, CELL_SIZE, health_bar_height);
                        self.rendering_context.fill_rect(x, bar_y, fill_width, health_bar_height);
                    }
                }
            }
        }
    }
}
