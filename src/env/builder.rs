use core::cell::Cell;
use super::{Environment,WindowDef};
use tao::event_loop::{EventLoop};
use tao::window::{WindowBuilder};

#[derive(Clone, Copy)]
pub struct WindowConfig {
    title: &'static str,
    width: u32,
    height: u32,
    resizable: bool,
}

pub struct EnvironmentBuilder {
    windows: Vec<WindowConfig>,
}

impl EnvironmentBuilder {
    pub fn new() -> Self {
        let windows = Vec::new();

        Self {
            windows,
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
                .build(event_loop)
                .unwrap();

            Cell::new(WindowDef {
                id: window.id(),
                title,
                width,
                height,
                resizable,
            })
        }).collect();

        Environment::new(windows)
    }
}

impl Default for EnvironmentBuilder {
    fn default() -> Self {
        Self::new()
    }
}
