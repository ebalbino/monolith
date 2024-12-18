
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
