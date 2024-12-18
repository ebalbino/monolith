use core::cell::Cell;
use tao::event::MouseButton;
use crate::math::Vec2;
use super::button::Button;
use super::delta::Delta;

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
            position: Delta::new(Position::default()),
            scroll: Delta::new(0.0),
        }
    }
}

impl Mouse {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn position(&self) -> Position {
        self.position.value()
    }

    pub fn position_delta(&self) -> Position {
        self.position.delta()
    }

    pub fn scroll(&self) -> f32 {
        self.scroll.value()
    }

    pub fn scroll_delta(&self) -> f32 {
        self.scroll.delta()
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
        self.position.update(position);
    }

    pub fn update_scroll(&self, delta: f32) {
        let value = self.scroll.value();
        self.scroll.update(value + delta);
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
