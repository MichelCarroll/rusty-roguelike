mod game;

use game::{
    common::{CanvasSize, Command},
    components::{
        player_controlled::PlayerControlled, rendered::Render, world_position::WorldPosition, movable::Movable, level::Level, collidable::Collidable, pickupable::Pickupable, inventoried::Inventoried, factioned::Factioned, ai_controlled::AIControlled, damageable::Damageable, armed::Armed,
    },
    systems::{rendering::Rendering, player_command_handler::PlayerCommandHandler, movement::Movement, level_generation::LevelGeneration, looting::Looting, ai::AI, combat::Combat},
    world::{LastUserEvent, WorldParameters},
};
use specs::prelude::*;
use std::panic;
use wasm_bindgen::prelude::*;

use web_sys::CanvasRenderingContext2d;

#[wasm_bindgen]
extern "C" {
    fn alert(string: &str);
}

struct CanvasHandle {
    context: CanvasRenderingContext2d,
}

fn create_canvas(size: CanvasSize, resolution_factor: f64) -> CanvasHandle {
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.create_element("canvas").unwrap();
    canvas
        .set_attribute("width", &(size.width * resolution_factor).to_string())
        .unwrap();
    canvas
        .set_attribute("height", &(size.height * resolution_factor).to_string())
        .unwrap();

    let style = document.create_element("style").unwrap();
    let style_content = format!("canvas {{ width: {:?}px; height: {:?}px; }}", size.width as u32, size.height as u32);
    style.set_text_content(Some(&style_content));
    document.body().unwrap().append_child(&style).unwrap();
    document.body().unwrap().append_child(&canvas).unwrap();

    let canvas: web_sys::HtmlCanvasElement =
        canvas.dyn_into::<web_sys::HtmlCanvasElement>().unwrap();

    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();


    CanvasHandle { context }
}

#[wasm_bindgen]
pub fn start() {
    console_log::init_with_level(log::Level::Debug).unwrap();
    panic::set_hook(Box::new(console_error_panic_hook::hook));

    let size = CanvasSize {
        width: 1500.0,
        height: 1500.0,
    };
    let canvas_handle = create_canvas(size, 2.0);

    let mut world = World::new();
    world.register::<WorldPosition>();
    world.register::<PlayerControlled>();
    world.register::<Movable>();
    world.register::<Render>();
    world.register::<Level>(); 
    world.register::<Collidable>(); 
    world.register::<Pickupable>(); 
    world.register::<Inventoried>(); 
    world.register::<Factioned>(); 
    world.register::<AIControlled>(); 
    world.register::<Damageable>(); 
    world.register::<Armed>(); 

    world.insert(LastUserEvent::default());
    world.insert(WorldParameters { width: 30, height: 30 });

    world.create_entity().with(Level::default()).build();
 
    let mut dispatcher = DispatcherBuilder::new()
        .with(LevelGeneration {}, "level-generation", &[])
        .with(PlayerCommandHandler {}, "player-command-handling", &["level-generation"])
        .with(AI {}, "ai", &["level-generation"])
        .with(Movement {}, "movement", &["player-command-handling"])
        .with(Combat {}, "combat", &["movement"])
        .with(Looting {}, "looting", &["movement"])
        .with(Rendering {
            canvas_size: size,
            rendering_context: canvas_handle.context,
        }, "rendering", &["movement", "combat", "looting"])
        .build();

    dispatcher.dispatch(&mut world);

    let a = Closure::<dyn FnMut(_)>::new(move |e: web_sys::KeyboardEvent| {
        let mut last_user_event = world.write_resource::<LastUserEvent>();
        match e.key_code() {
            37 => last_user_event.event = Command::GoLeft.into(),
            38 => last_user_event.event = Command::GoUp.into(),
            39 => last_user_event.event = Command::GoRight.into(),
            40 => last_user_event.event = Command::GoDown.into(),
            _ => (),
        };
        drop(last_user_event);

        dispatcher.dispatch(&mut world);
        world.maintain();
        e.prevent_default();
    });
    web_sys::window()
        .unwrap()
        .add_event_listener_with_callback("keydown", a.as_ref().unchecked_ref())
        .unwrap();
    a.forget();
}
