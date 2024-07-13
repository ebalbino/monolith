use monolith::{Arena};
use monolith::math::*;
use monolith::draw::mesh::{Element};
use monolith::draw::mesh::{make_box, quads_to_triangles};
use tao::event_loop::{EventLoop, ControlFlow};
use tao::window::WindowBuilder;
use tao::event::{Event, WindowEvent};

fn main() {
    let render_arena = Arena::new(1024 * 1024 * 16);

    let mesh = make_box(&render_arena, Vec3u::new(2, 2, 2), Vec3::new(1.0, 1.0, 1.0), Vec3::new(64.0, 64.0, 64.0)).unwrap();

    let positions = mesh.positions();
    let texcoords = mesh.texcoords();
    let vertex_count = positions.len();
    let element_count = match mesh.elements() {
        Element::Quad(elements) => elements.len(),
        _ => 0,
    };

    let Element::Quad(elements) = mesh.elements() else { todo!() };

    println!("Vertex count: {}", vertex_count);
    println!("Element count: {}", element_count);
    println!("Memory usage: {} bytes", render_arena.occupied());

    for i in 0..vertex_count {
        let position = positions[i];
        let texcoord = texcoords[i];
        println!("Vertex {}: {:?} {:?}", i, position, texcoord);
    }

    let triangles = quads_to_triangles(&render_arena, &elements).unwrap();

    println!("Triangle count: {}", triangles.len());
    println!("Memory usage: {} bytes", render_arena.occupied());

    let event_loop = EventLoop::new();
    let _window_one = WindowBuilder::new()
        .with_title("Monolith")
        .with_inner_size(tao::dpi::PhysicalSize::new(320, 240))
        .build(&event_loop);

    event_loop.run(move |event, _, control_flow| {
        // ControlFlow::Poll continuously runs the event loop, even if the OS hasn't
        // dispatched any events. This is ideal for games and similar applications.
        *control_flow = ControlFlow::Poll;

        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                println!("The close button was pressed; stopping");
                *control_flow = ControlFlow::Exit
            },
            Event::MainEventsCleared => {
                // Application update code.

                // Queue a RedrawRequested event.
                //
                // You only need to call this if you've determined that you need to redraw, in
                // applications which do not always need to. Applications that redraw continuously
                // can just render here instead.
                //window.request_redraw();
            },
            Event::RedrawRequested(_) => {
                // Redraw the application.
                //
                // It's preferable for applications that do not render continuously to render in
                // this event rather than in MainEventsCleared, since rendering in here allows
                // the program to gracefully handle redraws requested by the OS.
            },
            _ => ()
        }
    });
}
