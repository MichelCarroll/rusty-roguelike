mod game;

use futures::{channel::mpsc, future, stream::select, StreamExt};
use game::{
    common::{CanvasPosition, CanvasSize, UIEvent},
    components::{
        ai_controlled::AIControlled, armed::Armed, collidable::Collidable, damageable::Damageable,
        factioned::Factioned, inventoried::Inventoried, level::Level, movable::Movable,
        opaque::Opaque, pickupable::Pickupable, player_controlled::PlayerControlled,
        rendered::Render, sighted::Sighted,
    },
    systems::{
        ai::AI, combat::Combat, level_generation::LevelGeneration, looting::Looting,
        movement::Movement, perspective::Perspective, player_command_handler::PlayerCommandHandler,
        rendering::Rendering, ui::UI,
    },
    ui::game_ui::GameUI,
    world::{LastUserEvent, UIState, WorldParameters, WorldPosition, WorldTime},
};
use gloo_timers::future::IntervalStream;
use log::info;
use specs::prelude::*;
use std::panic;
use wasm_bindgen::prelude::*;

use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

#[wasm_bindgen]
extern "C" {
    fn alert(string: &str);
}

struct CanvasHandle {
    context: CanvasRenderingContext2d,
    canvas: HtmlCanvasElement,
    actual_canvas_size: CanvasSize,
}

fn init_canvas_handle(size: CanvasSize) -> CanvasHandle {
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document
        .get_elements_by_class_name("game-canvas")
        .item(0)
        .unwrap();
    canvas
        .set_attribute("width", &(size.width).to_string())
        .unwrap();
    canvas
        .set_attribute("height", &(size.height).to_string())
        .unwrap();

    let canvas: web_sys::HtmlCanvasElement =
        canvas.dyn_into::<web_sys::HtmlCanvasElement>().unwrap();

    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();

    let actual_canvas_size = CanvasSize {
        width: canvas.offset_width() as f64,
        height: canvas.offset_height() as f64,
    };

    CanvasHandle {
        context,
        canvas,
        actual_canvas_size,
    }
}

#[wasm_bindgen]
pub async fn start() {
    console_log::init_with_level(log::Level::Debug).unwrap();
    panic::set_hook(Box::new(console_error_panic_hook::hook));

    let game_ui = GameUI::new();
    dominator::append_dom(&dominator::body(), GameUI::render(&game_ui));

    let canvas_size = CanvasSize {
        width: 1500.0,
        height: 1500.0,
    };
    let canvas_handle = init_canvas_handle(canvas_size);

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
    info!("{:?}", WorldParameters::from_canvas_size(canvas_size));
    world.insert(LastUserEvent::default());
    world.insert(WorldParameters::from_canvas_size(canvas_size));
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
        .with(
            Perspective {},
            "perspective",
            &["movement", "combat", "looting"],
        )
        .with(
            UI {
                ui_state: game_ui.clone(),
            },
            "ui",
            &["perspective", "looting", "combat", "movement"],
        )
        .with(
            Rendering {
                canvas_size,
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
        event_dispatcher
            .unbounded_send(UIEvent::MouseOver(CanvasPosition::from_real(
                CanvasPosition {
                    x: e.offset_x() as f64,
                    y: e.offset_y() as f64,
                },
                canvas_handle.actual_canvas_size,
                canvas_size,
            )))
            .unwrap();
        e.prevent_default();
    });

    canvas_handle
        .canvas
        .add_event_listener_with_callback("mousemove", mouse_move_handler.as_ref().unchecked_ref())
        .unwrap();
    mouse_move_handler.forget();

    let event_dispatcher = dx.clone();
    let mouse_down_handler = Closure::<dyn FnMut(_)>::new(move |e: web_sys::MouseEvent| {
        event_dispatcher
            .unbounded_send(UIEvent::MousePress(CanvasPosition::from_real(
                CanvasPosition {
                    x: e.offset_x() as f64,
                    y: e.offset_y() as f64,
                },
                canvas_handle.actual_canvas_size,
                canvas_size,
            )))
            .unwrap();
        e.prevent_default();
    });

    canvas_handle
        .canvas
        .add_event_listener_with_callback("mousedown", mouse_down_handler.as_ref().unchecked_ref())
        .unwrap();
    mouse_down_handler.forget();

    let render_request_stream = IntervalStream::new(16).map(|_| None);
    let events_stream = select(rx.map(|e: UIEvent| Some(e)), render_request_stream);
    let mut events_since_last_render: Vec<UIEvent> = vec![];

    events_stream
        .for_each(move |event| {
            match event {
                None => {
                    let mut last_user_event = world.write_resource::<LastUserEvent>();
                    last_user_event.events = events_since_last_render.drain(..).collect();
                    drop(last_user_event);
                    dispatcher.dispatch(&mut world);
                    world.maintain();
                }
                Some(event) => {
                    events_since_last_render.push(event);
                }
            }
            future::ready(())
        })
        .await;
}
