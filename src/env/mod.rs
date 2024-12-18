use core::cell::Cell;
use crate::math::Vec2;
use tao::event::MouseButton;
use tao::event::{ElementState, Event, KeyEvent, MouseScrollDelta, WindowEvent};
use tao::event_loop::{ControlFlow, EventLoop};
use tao::keyboard::KeyCode;
use tao::window::{WindowId, WindowBuilder};

mod button;
mod delta;
mod clock;
mod keyboard;
mod mouse;

use delta::Delta;
use clock::Clock;
use keyboard::Keyboard;
use mouse::Mouse;

#[derive(Clone, Copy)]
pub struct WindowConfig {
    title: &'static str,
    width: u32,
    height: u32,
    resizable: bool,
}

#[derive(Clone, Copy)]
pub struct WindowDef {
    id: WindowId,
    title: &'static str,
    width: u32,
    height: u32,
    resizable: bool,
}

pub struct Environment {
    initialized: Cell<bool>,
    quit: Cell<bool>,

    windows: Vec<Cell<WindowDef>>,
    mouse: Mouse,
    keyboard: Keyboard,
    clock: Clock,
}

pub struct EnvironmentBuilder {
    windows: Vec<WindowConfig>,
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

    pub fn windows(&self) -> &[Cell<WindowDef>] {
        &self.windows
    }

    pub fn window(&self, id: WindowId) -> Option<&Cell<WindowDef>> {
        self.windows.iter().find(|w| w.get().id == id)
    }

    pub fn window_mut(&mut self, id: WindowId) -> Option<&mut Cell<WindowDef>> {
        self.windows.iter_mut().find(|w| w.get().id == id)
    }

    pub fn window_title(&self, id: WindowId) -> Option<&'static str> {
        self.window(id).map(|w| w.get().title)
    }

    pub fn window_size(&self, id: WindowId) -> Option<(u32, u32)> {
        self.window(id).map(|w| (w.get().width, w.get().height))
    }

    pub fn window_resizable(&self, id: WindowId) -> Option<bool> {
        self.window(id).map(|w| w.get().resizable)
    }

    pub fn update(&mut self, event: Event<()>, control_flow: &mut ControlFlow) {
        match event {
            Event::WindowEvent { event, window_id, .. } => match event {
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
                    let width = size.width;
                    let height = size.height;

                        if let Some(cell) = self.window_mut(window_id) {
                            let window = cell.get_mut();
                            window.width = width;
                            window.height = height;
                        }
                }
                WindowEvent::MouseInput {
                    state,
                    button,
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

impl EnvironmentBuilder {
    pub fn new() -> Self {
        let windows = Vec::new();
        let mouse = Mouse::default();
        let keyboard = Keyboard::new();
        let clock = Clock::new();

        Self {
            windows,
            mouse,
            keyboard,
            clock,
        }
    }

    pub fn window(&mut self, title: &'static str, width: u32, height: u32, resizable: bool) -> &mut Self {
        let window = WindowConfig {
            title,
            width,
            height,
            resizable,
        };

        self.windows.push(window);
        self
    }

    pub fn build(self, event_loop: &EventLoop<()>) -> Environment {
        let windows = self.windows.into_iter().map(|w| {
            let title = w.title;
            let width = w.width;
            let height = w.height;
            let resizable = w.resizable;
            
            let window = WindowBuilder::new()
                .with_title(title)
                .with_inner_size(tao::dpi::PhysicalSize::new(width, height))
                .with_resizable(resizable)
                .build(&event_loop)
                .unwrap();

            Cell::new(WindowDef {
                id: window.id(),
                title,
                width,
                height,
                resizable,
            })
        }).collect();

        Environment {
            initialized: Cell::new(false),
            quit: Cell::new(false),
            keyboard: self.keyboard,
            mouse: self.mouse,
            clock: self.clock,
            windows,
        }
    }
}

