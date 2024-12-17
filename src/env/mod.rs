use core::cell::{Cell, OnceCell};
use core::ops::Sub;
use glam::Vec2;
use tao::event::MouseButton;
use tao::keyboard::KeyCode;

mod clock;
mod gamepad;
mod keyboard;
mod mouse;

use clock::{Clock, Instant};
use keyboard::Keyboard;
use mouse::Mouse;

#[derive(Clone, Copy)]
pub struct Axis {
    x: f32,
    y: f32,
}

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
        self.value.set(value);
        self.delta.set(value - current);
    }
}

pub struct Switch<T: Copy> {
    value: Cell<T>,
    previous: Cell<T>,
}

impl<T: Copy> Switch<T> {
    pub fn new(value: T) -> Self {
        Self {
            value: Cell::new(value),
            previous: Cell::new(value),
        }
    }

    pub fn current(&self) -> T {
        self.value.get()
    }

    pub fn previous(&self) -> T {
        self.previous.get()
    }

    pub fn update(&self, value: T) {
        let current = self.value.get();
        self.previous.set(current);
        self.value.set(value);
    }
}

pub struct AnalogButton {
    threshold: f32,
    value: Cell<f32>,
    inner: Cell<Button>,
}

impl AnalogButton {
    pub fn new(threshold: f32) -> Self {
        Self {
            threshold,
            value: Cell::new(0.0),
            inner: Cell::new(Button::default()),
        }
    }

    pub fn update(&mut self, value: f32) {
        let button = self.inner.get_mut();
        self.value.set(value);

        button.update(value >= self.threshold);
    }

    pub fn value(&self) -> f32 {
        self.value.get()
    }

    pub fn down(&self) -> bool {
        self.inner.get().down()
    }

    pub fn repeat(&self) -> bool {
        self.inner.get().repeat()
    }

    pub fn pressed(&self) -> bool {
        self.inner.get().pressed()
    }

    pub fn released(&self) -> bool {
        self.inner.get().released()
    }
}

pub struct Stick {
    axis: Cell<Axis>,
    button: Cell<Button>,
    threshold: Cell<f32>,
}

impl Stick {
    pub fn new(threshold: f32) -> Self {
        Self {
            axis: Cell::new(Axis { x: 0.0, y: 0.0 }),
            button: Cell::new(Button::default()),
            threshold: Cell::new(threshold),
        }
    }

    pub fn update(&self, x: f32, y: f32) {
        let x = if x.abs() <= self.threshold.get() {
            0.0
        } else {
            x
        };
        let y = if y.abs() <= self.threshold.get() {
            0.0
        } else {
            y
        };
        self.axis.set(Axis { x, y });
    }
}

pub struct Environment {
    initialized: Cell<bool>,
    quit: Cell<bool>,
    focused: Cell<bool>,

    window_size: Cell<Vec2>,
    window_title: String,

    mouse: Mouse,
    keyboard: Keyboard,
    clock: Clock,
}

impl Environment {
    pub fn new() -> Self {
        let keyboard = Keyboard::new();
        let mouse = Mouse::default();
        let clock = Clock::new();

        Self {
            window_size: Cell::new(Vec2::new(0.0, 0.0)),
            window_title: "monolith".to_string(),
            initialized: Cell::new(false),
            quit: Cell::new(false),
            focused: Cell::new(true),
            keyboard,
            clock,
            mouse,
        }
    }

    pub fn initialized(&self) -> bool {
        self.initialized.get()
    }

    pub fn quit(&self) -> bool {
        self.quit.get()
    }

    pub fn window_size(&self) -> Vec2 {
        self.window_size.get()
    }

    pub fn window_title(&self) -> &str {
        &self.window_title
    }

    pub fn update_keyboard(&mut self, key: KeyCode, down: bool) {
        self.keyboard.update(key, down);
    }

    pub fn update_focused(&mut self, focused: bool) {
        self.focused.set(focused);
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
}
