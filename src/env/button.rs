#[derive(Default, Clone, Copy, Debug)]
pub struct Button {
    down: bool,
    repeat: bool,
    pressed: bool,
    released: bool,
}

impl Button {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn update(&self, down: bool) -> Button {
        let was_down = self.down;
        let down = down;
        let repeat = was_down && down;
        let pressed = !was_down && down;
        let released = was_down && !down;

        Self {
            down,
            repeat,
            pressed,
            released,
        }
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
