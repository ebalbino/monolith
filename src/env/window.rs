use core::cell::Cell;
use glam::Vec2;

pub struct Window {
    title: String,
    size: Cell<Vec2>,
    focused: Cell<bool>,
}

impl Default for Window {
    fn default() -> Self {
        Self {
            title: "monolith".to_string(),
            size: Cell::new(Vec2::default()),
            focused: Cell::new(false),
        }
    }
}
