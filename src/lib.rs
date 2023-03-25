mod game;

use futures::{channel::mpsc, StreamExt, future, stream::select};
use game::{
    common::{CanvasSize, UIEvent},
    components::{
        ai_controlled::AIControlled, armed::Armed, collidable::Collidable, damageable::Damageable,
        factioned::Factioned, inventoried::Inventoried, level::Level, movable::Movable,
        pickupable::Pickupable, player_controlled::PlayerControlled, rendered::Render,
        sighted::Sighted, opaque::Opaque, 
    }, 
    systems::{
        ai::AI, combat::Combat, level_generation::LevelGeneration, looting::Looting,
        movement::Movement, player_command_handler::PlayerCommandHandler, rendering::Rendering, perspective::Perspective,
    },
    world::{LastUserEvent, WorldParameters, WorldPosition, WorldTime, UIState},
};
use gloo_timers::future::{TimeoutFuture, IntervalStream};
use specs::prelude::*;
use std::{panic, sync::Arc, time::Duration, ops::Range};
use wasm_bindgen::prelude::*;

use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};
 
#[wasm_bindgen]
extern "C" {
    fn alert(string: &str);
}

struct CanvasHandle {
    context: CanvasRenderingContext2d,
    canvas: HtmlCanvasElement
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
    let style_content = format!(
        "canvas {{ width: {:?}px; height: {:?}px; }}",
        size.width as u32, size.height as u32
    );
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

    CanvasHandle { context, canvas }
}

#[wasm_bindgen]
pub async fn start() {
    console_log::init_with_level(log::Level::Debug).unwrap();
    panic::set_hook(Box::new(console_error_panic_hook::hook));

    let size = CanvasSize {
        width: 1500.0,
        height: 1500.0,
    };
    let resolution_factor = 2.0;
    let canvas_handle = create_canvas(size, resolution_factor);

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
    world.register::<Sighted>();
    world.register::<Opaque>();

    world.insert(LastUserEvent::default());
    world.insert(WorldParameters {
        width: 30,
        height: 30,
    });
    world.insert(WorldTime::default());
    world.insert(UIState::default());

    world.create_entity().with(Level::default()).build();

    let mut dispatcher = DispatcherBuilder::new()
        .with(LevelGeneration {}, "level-generation", &[])
        .with(
            PlayerCommandHandler {},
            "player-command-handling",
            &["level-generation"],
        )
        .with(AI::default(), "ai", &["level-generation"])
        .with(Movement {}, "movement", &["player-command-handling"])
        .with(Combat {}, "combat", &["movement"])
        .with(Looting {}, "looting", &["movement"])
        .with(Perspective {}, "perspective", &["movement", "combat", "looting"])
        .with(
            Rendering {
                canvas_size: size,
                rendering_context: canvas_handle.context
            },
            "rendering",
            &["perspective"],
        )
        .build();

    let (dx, rx) = mpsc::unbounded::<UIEvent>();

    dispatcher.dispatch(&mut world);

    let event_dispatcher = dx.clone();
    let keyboard_handler = Closure::<dyn FnMut(_)>::new(move |e: web_sys::KeyboardEvent| {
        let event: Option<UIEvent> = match e.key_code() {
            37 => UIEvent::Left.into(),
            38 => UIEvent::Up.into(),
            39 => UIEvent::Right.into(),
            40 => UIEvent::Down.into(),
            _ => None,
        };
        if let Some(event) = event {
            event_dispatcher.unbounded_send(event).unwrap();
        }
        e.prevent_default();
    });

    web_sys::window()
        .unwrap()
        .add_event_listener_with_callback("keydown", keyboard_handler.as_ref().unchecked_ref())
        .unwrap();
    keyboard_handler.forget();

    let event_dispatcher = dx.clone();
    let mouse_move_handler = Closure::<dyn FnMut(_)>::new(move |e: web_sys::MouseEvent| {
        event_dispatcher.unbounded_send(UIEvent::MouseOver(e.offset_x() as f64 * resolution_factor, e.offset_y() as f64  * resolution_factor)).unwrap();
        e.prevent_default();
    });

    canvas_handle.canvas
        .add_event_listener_with_callback("mousemove", mouse_move_handler.as_ref().unchecked_ref())
        .unwrap();
    mouse_move_handler.forget();


    let event_dispatcher = dx.clone();
    let mouse_down_handler = Closure::<dyn FnMut(_)>::new(move |e: web_sys::MouseEvent| {
        event_dispatcher.unbounded_send(UIEvent::MousePress(e.offset_x() as f64 * resolution_factor, e.offset_y() as f64  * resolution_factor)).unwrap();
        e.prevent_default();
    });

    canvas_handle.canvas
        .add_event_listener_with_callback("mousedown", mouse_down_handler.as_ref().unchecked_ref())
        .unwrap();
    mouse_down_handler.forget();


    let render_request_stream = IntervalStream::new(16).map(|e| None);
    let events_stream = select(rx.map(|e: UIEvent| Some(e)), render_request_stream);
    let mut events_since_last_render: Vec<UIEvent> = vec![];
    
    events_stream.for_each(move |event| {
        match event {
            None => {
                let mut last_user_event = world.write_resource::<LastUserEvent>();
                last_user_event.events = events_since_last_render.drain(..).collect();
                drop(last_user_event);
                dispatcher.dispatch(&mut world);
                world.maintain();
            },
            Some(event) => {
                events_since_last_render.push(event);
            }
        }
        future::ready(())
    }).await;
    
}
 