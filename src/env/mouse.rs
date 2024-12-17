use crate::env::{Button, Delta};
use core::cell::Cell;
use glam::Vec2;
use tao::event::MouseButton;

type Position = Vec2;

pub struct Mouse {
    left_button: Cell<Button>,
    right_button: Cell<Button>,
    middle_button: Cell<Button>,

    position: Delta<Position>,
    scroll: Delta<f32>,
}

impl Default for Mouse {
    fn default() -> Self {
        Self {
            left_button: Cell::new(Button::default()),
            right_button: Cell::new(Button::default()),
            middle_button: Cell::new(Button::default()),
            position: Delta {
                value: Cell::new(Position::default()),
                delta: Cell::new(Position::default()),
            },
            scroll: Delta {
                value: Cell::new(0.0),
                delta: Cell::new(0.0),
            },
        }
    }
}

impl Mouse {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn position(&self) -> Position {
        self.position.value.get()
    }

    pub fn scroll(&self) -> f32 {
        self.scroll.value.get()
    }

    pub fn left_button(&self) -> Button {
        self.left_button.get()
    }

    pub fn right_button(&self) -> Button {
        self.right_button.get()
    }

    pub fn middle_button(&self) -> Button {
        self.middle_button.get()
    }

    pub fn update_position(&self, position: Position) {
        let value = self.position.value.get();

        self.position.value.set(position);
        self.position.delta.set(value - position);
    }

    pub fn update_scroll(&self, delta: f32) {
        let value = self.scroll.value.get();
        self.scroll.value.set(value + delta);
        self.scroll.delta.set(delta);
    }

    pub fn update_button(&mut self, button: MouseButton, down: bool) {
        let button = match button {
            MouseButton::Left => self.left_button.get_mut(),
            MouseButton::Right => self.right_button.get_mut(),
            MouseButton::Middle => self.middle_button.get_mut(),
            _ => return,
        };

        button.update(down);
    }
}
