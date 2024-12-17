use core::cell::Cell;
use core::ops::Sub;
use glam::Vec2;
use tao::event::MouseButton;
use tao::event::{DeviceEvent, ElementState, Event, KeyEvent, MouseScrollDelta, WindowEvent};
use tao::event_loop::{ControlFlow, EventLoop};
use tao::keyboard::KeyCode;
use tao::window::{Window, WindowBuilder};

mod clock;
mod keyboard;
mod mouse;

use clock::Clock;
use keyboard::Keyboard;
use mouse::Mouse;

#[derive(Clone, Copy, Debug)]
pub struct Button {
    down: bool,
    repeat: bool,
    pressed: bool,
    released: bool,
}

impl Default for Button {
    fn default() -> Self {
        Self {
            down: false,
            repeat: false,
            pressed: false,
            released: false,
        }
    }
}

impl Button {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn update(&mut self, down: bool) {
        let was_down = self.down;
        self.down = down;
        self.repeat = was_down && down;
        self.pressed = !was_down && down;
        self.released = was_down && !down;
    }

    pub fn down(&self) -> bool {
        self.down
    }

    pub fn repeat(&self) -> bool {
        self.repeat
    }

    pub fn pressed(&self) -> bool {
        self.pressed
    }

    pub fn released(&self) -> bool {
        self.released
    }
}

pub struct Delta<T: Copy + Sub<Output = T>> {
    value: Cell<T>,
    delta: Cell<T>,
}

impl<T: Copy + Sub<Output = T>> Delta<T> {
    pub fn new(value: T) -> Self {
        Self {
            value: Cell::new(value),
            delta: Cell::new(value),
        }
    }

    pub fn value(&self) -> T {
        self.value.get()
    }

    pub fn delta(&self) -> T {
        self.delta.get()
    }

    pub fn update(&self, value: T) {
        let current = self.value.get();
        self.delta.set(value - current);
        self.value.set(value);
    }
}

pub struct Environment {
    initialized: Cell<bool>,
    quit: Cell<bool>,

    window: Window,
    mouse: Mouse,
    keyboard: Keyboard,
    clock: Clock,
}

impl Environment {
    pub fn initialized(&self) -> bool {
        self.initialized.get()
    }

    pub fn quit(&self) -> bool {
        self.quit.get()
    }

    pub fn update_keyboard(&mut self, key: KeyCode, down: bool) {
        self.keyboard.update(key, down);
    }

    pub fn update_mouse_button(&mut self, button: MouseButton, down: bool) {
        self.mouse.update_button(button, down);
    }

    pub fn update_mouse_position(&mut self, x: f64, y: f64) {
        let position = Vec2::new(x as f32, y as f32);
        self.mouse.update_position(position);
    }

    pub fn update_mouse_scroll(&mut self, delta: f32) {
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

    pub fn update(&mut self, event: Event<()>, control_flow: &mut ControlFlow) {
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
                    ElementState::Pressed => self.update_keyboard(physical_key, true),
                    ElementState::Released => self.update_keyboard(physical_key, false),
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
                // Application update code.

                // Queue a RedrawRequested event.
                //
                // You only need to call this if you've determined that you need to redraw, in
                // applications which do not always need to. Applications that redraw continuously
                // can just render here instead.
                //window.request_redraw();
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

pub struct EnvironmentBuilder {
    window: Window,
    mouse: Mouse,
    keyboard: Keyboard,
    clock: Clock,
}

impl EnvironmentBuilder {
    pub fn new(event_loop: &EventLoop<()>, title: &str, width: u32, height: u32) -> Self {
        let window = WindowBuilder::new()
            .with_title(title)
            .with_inner_size(tao::dpi::PhysicalSize::new(width, height))
            .with_resizable(true)
            .build(&event_loop)
            .unwrap();
        let mouse = Mouse::default();
        let keyboard = Keyboard::new();
        let clock = Clock::new();

        Self {
            window,
            mouse,
            keyboard,
            clock,
        }
    }

    pub fn build(self) -> Environment {
        Environment {
            initialized: Cell::new(false),
            quit: Cell::new(false),
            keyboard: self.keyboard,
            mouse: self.mouse,
            clock: self.clock,
            window: self.window,
        }
    }
}
