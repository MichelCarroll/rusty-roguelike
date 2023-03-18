use std::{sync::mpsc::{self, Receiver}, panic, borrow::BorrowMut};

use specs::prelude::*;
use wasm_bindgen::prelude::*;

use web_sys::CanvasRenderingContext2d;

#[wasm_bindgen]
extern "C" {
    fn alert(string: &str);
}

#[derive(Clone, Copy)]
struct CanvasSize {
    width: f64, height: f64
}

#[derive(Debug)]
enum UserEvent {
    Right, Left, Up, Down
}

struct CanvasHandle {
    context: CanvasRenderingContext2d
}

fn create_canvas(size: CanvasSize) -> CanvasHandle {
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.create_element("canvas").unwrap();
    canvas.set_attribute("width", &size.width.to_string()).unwrap();
    canvas.set_attribute("height", &size.height.to_string()).unwrap();

    document.body().unwrap().append_child(&canvas).unwrap();
    
    let canvas: web_sys::HtmlCanvasElement = canvas
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .unwrap();

    CanvasHandle {
        context: canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<web_sys::CanvasRenderingContext2d>()
            .unwrap()
    }
}

#[derive(Default)]
struct LastUserEvent {
    event: Option<UserEvent>
}

#[wasm_bindgen]
pub fn start() {
    panic::set_hook(Box::new(console_error_panic_hook::hook));

    let size = CanvasSize { width: 500.0, height: 500.0 };
    let canvas_handle = create_canvas(size);

    let mut world = World::new();
    world.register::<Position>();
    world.insert(LastUserEvent::default());

    world.create_entity().with(Position { x: 200.0, y: 100.0 }).build();
    world.create_entity().with(Position { x: 100.0, y: 100.0 }).build();
    world.create_entity().with(Position { x: 400.0, y: 100.0 }).build();
    world.create_entity().with(Position { x: 300.0, y: 100.0 }).build();

    let rendering = Rendering {
        canvas_size: size,
        rendering_context: canvas_handle.context
    };
    let mut dispatcher = DispatcherBuilder::new().with(rendering, "rendering", &[]).build();

    dispatcher.dispatch(&mut world);
    

    let a = Closure::<dyn FnMut(_)>::new(move |e: web_sys::KeyboardEvent| {

        let mut last_user_event = world.write_resource::<LastUserEvent>();
        match e.key_code() {
            37 => last_user_event.event = UserEvent::Left.into(),
            38 => last_user_event.event = UserEvent::Up.into(),
            39 => last_user_event.event = UserEvent::Right.into(),
            40 => last_user_event.event = UserEvent::Down.into(),
            _ => ()
        };
        drop(last_user_event);
        
        dispatcher.dispatch(&mut world);
    });
    web_sys::window().unwrap().add_event_listener_with_callback("keydown", a.as_ref().unchecked_ref()).unwrap();
    a.forget();


}
 


struct Position { 
    x: f64,
    y: f64,
}

impl Component for Position {
    type Storage = VecStorage<Self>;
}

struct Rendering {
    canvas_size: CanvasSize,
    rendering_context: CanvasRenderingContext2d
}

// Safe because wasm is always strictly single-threaded
unsafe impl Send for Rendering {}

impl<'a> System<'a> for Rendering {
    
    type SystemData = (ReadStorage<'a, Position>, Write<'a, LastUserEvent>);
    
    fn run(&mut self, (pos, mut last_user_event): Self::SystemData) {
        self.rendering_context.set_fill_style(&"#556688".into());
        self.rendering_context.fill_rect(0.0, 0.0,self.canvas_size.width, self.canvas_size.height);
        alert(&format!("{:?}", last_user_event.event));
        last_user_event.event = None;
        for pos in (&pos).join() {
            self.rendering_context.set_fill_style(&"#885511".into());
            self.rendering_context.fill_rect(pos.x, pos.y,50.0, 50.0);
        }
    }
}
