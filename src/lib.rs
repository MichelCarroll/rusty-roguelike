
use std::f64;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(string: &str);
}

#[wasm_bindgen]
pub fn start() {
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.create_element("canvas").unwrap();
    document.body().unwrap().append_child(&canvas).unwrap();
    
    let canvas: web_sys::HtmlCanvasElement = canvas
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();

    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();

    context.begin_path();

    // Draw the outer circle.
    context
        .arc(75.0, 75.0, 50.0, 0.0, f64::consts::PI * 2.0)
        .unwrap();

    // Draw the mouth.
    context.move_to(110.0, 75.0);
    context.arc(75.0, 75.0, 35.0, 0.0, f64::consts::PI).unwrap();

    // Draw the left eye.
    context.move_to(65.0, 65.0);
    context
        .arc(60.0, 65.0, 5.0, 0.0, f64::consts::PI * 2.0)
        .unwrap();

    // Draw the right eye.
    context.move_to(95.0, 65.0);
    context
        .arc(90.0, 65.0, 5.0, 0.0, f64::consts::PI * 2.0)
        .unwrap();

    context.stroke();


    // The `World` is our
    // container for components
    // and other resources.

    let mut world = World::new();
    world.register::<Pos>();
    world.register::<Vel>();

    // An entity may or may not contain some component.

    world.create_entity().with(Vel(2.0)).with(Pos(0.0)).build();
    world.create_entity().with(Vel(4.0)).with(Pos(1.6)).build();
    world.create_entity().with(Vel(1.5)).with(Pos(5.4)).build();

    // This entity does not have `Vel`, so it won't be dispatched.
    world.create_entity().with(Pos(2.0)).build();

    // This builds a dispatcher.
    // The third parameter of `add` specifies
    // logical dependencies on other systems.
    // Since we only have one, we don't depend on anything.
    // See the `full` example for dependencies.
    let mut dispatcher = DispatcherBuilder::new().with(SysA, "sys_a", &[]).build();

    // This dispatches all the systems in parallel (but blocking).
    dispatcher.dispatch(&mut world);
}



extern crate specs;

use specs::prelude::*;

// A component contains data which is
// associated with an entity.

struct Vel(f32);

impl Component for Vel {
    type Storage = VecStorage<Self>;
}

struct Pos(f32);

impl Component for Pos {
    type Storage = VecStorage<Self>;
}

struct SysA;

impl<'a> System<'a> for SysA {
    // These are the resources required for execution.
    // You can also define a struct and `#[derive(SystemData)]`,
    // see the `full` example.
    type SystemData = (WriteStorage<'a, Pos>, ReadStorage<'a, Vel>);

    fn run(&mut self, (mut pos, vel): Self::SystemData) {
        // The `.join()` combines multiple components,
        // so we only access those entities which have
        // both of them.

        // This joins the component storages for Position
        // and Velocity together; it's also possible to do this
        // in parallel using rayon's `ParallelIterator`s.
        // See `ParJoin` for more.
        for (pos, vel) in (&mut pos, &vel).join() {
            alert("Hey");
            pos.0 += vel.0;
        }
    }
}
