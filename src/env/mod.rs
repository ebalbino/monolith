use crate::math::Vec2;
use core::cell::{Cell, RefCell};
use tao::event::MouseButton;
use tao::event::{ElementState, Event, KeyEvent, MouseScrollDelta, WindowEvent};
use tao::event_loop::ControlFlow;
use tao::keyboard::KeyCode;
use tao::window::Window;

mod button;
mod clock;
mod delta;
mod keyboard;
mod mouse;

use clock::Clock;
use delta::Delta;
use keyboard::Keyboard;
use mouse::Mouse;

pub struct Environment {
    initialized: Cell<bool>,
    quit: Cell<bool>,

    window: Window,
    mouse: Mouse,
    keyboard: Keyboard,
    clock: Clock,
}

impl Environment {
    pub fn new(window: Window) -> Self {
        let mouse = Mouse::default();
        let keyboard = Keyboard::new();
        let clock = Clock::new();

        Self {
            initialized: Cell::new(false),
            quit: Cell::new(false),
            window,
            mouse,
            keyboard,
            clock,
        }
    }

    pub fn initialized(&self) -> bool {
        self.initialized.get()
    }

    pub fn quit(&self) -> bool {
        self.quit.get()
    }

    pub fn update_keyboard(&self, key: KeyCode, down: bool) {
        self.keyboard.update(key, down);
    }

    pub fn update_mouse_button(&self, button: MouseButton, down: bool) {
        self.mouse.update_button(button, down);
    }

    pub fn update_mouse_position(&self, x: f64, y: f64) {
        let position = Vec2::new(x as f32, y as f32);
        self.mouse.update_position(position);
    }

    pub fn update_mouse_scroll(&self, delta: f32) {
        self.mouse.update_scroll(delta);
    }

    pub fn keyboard(&self) -> &Keyboard {
        &self.keyboard
    }

    pub fn mouse(&self) -> &Mouse {
        &self.mouse
    }

    pub fn clock(&self) -> &Clock {
        &self.clock
    }

    pub fn window_title(&self) -> String {
        self.window.title()
    }

    pub fn window_size(&self) -> (u32, u32) {
        let size = self.window.inner_size();
        (size.width, size.height)
    }

    pub fn window_resizable(&self) -> bool {
        self.window.is_resizable()
    }

    pub fn update(&self, event: Event<()>, control_flow: &mut ControlFlow) {
        match event {
            Event::WindowEvent {
                event, window_id, ..
            } => match event {
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
                    ElementState::Pressed => self.update_keyboard(physical_key, true),
                    ElementState::Released => self.update_keyboard(physical_key, false),
                    _ => (),
                },
                WindowEvent::MouseInput { state, button, .. } => match state {
                    ElementState::Pressed => self.update_mouse_button(button, true),
                    ElementState::Released => self.update_mouse_button(button, false),
                    _ => (),
                },
                WindowEvent::MouseWheel { delta, .. } => match delta {
                    MouseScrollDelta::LineDelta(x, _) => {
                        self.update_mouse_scroll(x);
                    }
                    MouseScrollDelta::PixelDelta(pos) => {
                        self.update_mouse_scroll(pos.x as f32);
                    }
                    _ => (),
                },
                WindowEvent::CursorMoved { position, .. } => {
                    self.update_mouse_position(position.x, position.y);
                }
                _ => (),
            },
            Event::MainEventsCleared => {
                let keyboard = self.keyboard();
                let mouse = self.mouse();
                let clock = self.clock();

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
    }
}
