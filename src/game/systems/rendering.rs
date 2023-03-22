use std::collections::HashMap;

use specs::prelude::*;
use web_sys::CanvasRenderingContext2d;

use crate::game::{
    common::{CanvasSize, Color},
    components::{
        player_controlled::PlayerControlled, rendered::Render, sighted::Sighted,
    }, world::WorldPosition,
};

const CELL_SIZE: f64 = 50.0;
const BACKGROUND_COLOR: &str = "#000000";

pub struct Rendering {
    pub canvas_size: CanvasSize,
    pub rendering_context: CanvasRenderingContext2d,
}

struct RenderTarget {
    glyph: Option<(char, Color)>,
    background_color: Option<Color>,
}

// Safe because wasm is always strictly single-threaded
unsafe impl Send for Rendering {}

impl<'a> System<'a> for Rendering {
    type SystemData = (
        ReadStorage<'a, WorldPosition>,
        ReadStorage<'a, Render>,
        ReadStorage<'a, PlayerControlled>,
        ReadStorage<'a, Sighted>,
    );

    fn run(&mut self, (pos, render, player_controlled, sighted): Self::SystemData) {
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
            let mut renderable = (&pos, &render, &sighted.seen).join().collect::<Vec<_>>();
            renderable.sort_by(|a, b| a.1.z_layer.cmp(&b.1.z_layer));

            let mut hash_map: HashMap<WorldPosition, RenderTarget> = HashMap::new();

            for (pos, render, _) in renderable {
                let new_glyph = render.glyph.map(|g| (g, render.foreground_color.clone()));
                if let Some(render_target) = hash_map.get_mut(pos) {
                    if let Some(new_glyph) = new_glyph {
                        render_target.glyph = new_glyph.into();
                    }
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
                        },
                    );
                }
            }

            for (pos, render_target) in hash_map {
                let x = CELL_SIZE * pos.x as f64;
                let y = CELL_SIZE * pos.y as f64;

                if let Some(background_color) = &render_target.background_color {
                    self.rendering_context
                        .set_fill_style(&(background_color.rbg_code.clone().into()));
                    self.rendering_context.fill_rect(x, y, CELL_SIZE, CELL_SIZE);
                }

                if let Some((glyph, foreground_color)) = render_target.glyph {
                    self.rendering_context
                        .set_fill_style(&(foreground_color.rbg_code.into()));
                    self.rendering_context
                        .fill_text(&glyph.to_string(), x + x_text_offset, y + y_text_offset)
                        .unwrap();
                }
            }
        }
    }
}
