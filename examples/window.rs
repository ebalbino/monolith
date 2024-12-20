use glam::Vec2;
use monolith::draw::mesh::Element;
use monolith::draw::mesh::{make_box, quads_to_triangles};
use monolith::env::Environment;
use monolith::math::*;
use monolith::Arena;
use tao::event::{DeviceEvent, ElementState, Event, KeyEvent, MouseScrollDelta, WindowEvent};
use tao::event_loop::{ControlFlow, EventLoop};
use tao::window::WindowBuilder;

fn main() {
    let title = "Monolith";
    let width = 800;
    let height = 600;
    let resizable = true;
    
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title(title)
        .with_inner_size(tao::dpi::PhysicalSize::new(width, height))
        .with_resizable(resizable)
        .build(&event_loop)
        .unwrap();

    let mut env = Environment::new(window);

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;

        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => {
                    *control_flow = ControlFlow::Exit;
                }
                WindowEvent::Destroyed => {
                    *control_flow = ControlFlow::Exit;
                }
                WindowEvent::KeyboardInput {
                    event:
                        KeyEvent {
                            state,
                            physical_key,
                            ..
                        },
                    ..
                } => match state {
                    ElementState::Pressed => env.update_keyboard(physical_key, true),
                    ElementState::Released => env.update_keyboard(physical_key, false),
                    _ => (),
                },
                WindowEvent::Resized(size) => {
                    //window.request_redraw();
                }
                WindowEvent::MouseInput {
                    state,
                    button,
                    device_id,
                    ..
                } => match state {
                    ElementState::Pressed => env.update_mouse_button(button, true),
                    ElementState::Released => env.update_mouse_button(button, false),
                    _ => (),
                },
                WindowEvent::MouseWheel { delta, .. } => match delta {
                    MouseScrollDelta::LineDelta(x, _) => {
                        env.update_mouse_scroll(x);
                    }
                    MouseScrollDelta::PixelDelta(pos) => {
                        env.update_mouse_scroll(pos.x as f32);
                    }
                    _ => (),
                },
                WindowEvent::CursorMoved { position, .. } => {
                    env.update_mouse_position(position.x, position.y);
                }
                _ => (),
            },
            Event::MainEventsCleared => {
                // Application update code.

                // Queue a RedrawRequested event.
                //
                // You only need to call this if you've determined that you need to redraw, in
                // applications which do not always need to. Applications that redraw continuously
                // can just render here instead.
                //window.request_redraw();
                let keyboard = env.keyboard();
                let mouse = env.mouse();
                let clock = env.clock();

                if keyboard.q_key_pressed() {
                    *control_flow = ControlFlow::Exit;
                }

                if mouse.left_button().down() {
                    println!("Mouse position: {:?}", mouse.position());
                }

                if mouse.right_button().down() {
                    let now = clock.now();
                    let resolution = clock.resolution();
                    println!("Current time: {:?}", now.seconds());
                    println!("Current resolution: {:?}", resolution);
                }

                clock.update();
            }
            _ => (),
        }
    })
}
