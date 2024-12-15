use std::cell::{Cell,OnceCell};
use glam::Vec2;
use tao::event::{ElementState,KeyEvent,WindowEvent};
use tao::keyboard::{KeyCode}

struct Position = Vec2;

struct Button {
    down: bool,
    pressed: bool,
    released: bool,
}

struct Signal {
    value: f32,
    threshold: f32,
}

struct Axis {
    x: f32,
    y: f32,
}

pub struct Instant {
    ticks: u64,
    nanoseconds: u64,
    microseconds: u64,
    milliseconds: u64,
    seconds: f32,
}

pub struct Modifier {
    shift: bool,
    ctrl: bool,
    alt: bool,
    super: bool,
}

struct Keyboard {
    keys: [Cell<Button>; 512],
    modifier: Cell<Modifier>,
}

struct Delta<T> {
    value: Cell<T>,
    delta: Cell<T>,
}

struct Switch<T> {
    value: Cell<T>,
    previous: Cell<T>,
}

struct AnalogButton {
    signal: Cell<Signal>,
    button: Cell<Button>,
}

struct Stick {
    axis: Cell<Axis>,
    button: Cell<Button>,
    threshold: Cell<f32>,
}

struct Mouse {
    left_button: Cell<Button>,
    right_button: Cell<Button>,
    middle_button: Cell<Button>,

    position: Delta<Position>,
    scroll: Delta<i32>,
}

struct Gamepad {
    connected: Switch<bool>,

    a: Cell<Button>,
    b: Cell<Button>,
    x: Cell<Button>,
    y: Cell<Button>,

    left_trigger: AnalogButton,
    right_trigger: AnalogButton,

    left_shoulder: Cell<Button>,
    right_shoulder: Cell<Button>,

    up: Cell<Button>,
    down: Cell<Button>,
    left: Cell<Button>,
    right: Cell<Button>,

    left_thumb_stick: Stick,
    right_thumb_stick: Stick,
    
    back: Cell<Button>,
    start: Cell<Button>,
}

struct Clock {
    instant: Delta<Instant>,
    initial_ticks: OnceCell<u64>,
    ticks_per_second: Cell<u64>,
}

pub struct Environment {
    initialized: Cell<bool>,
    visible: Cell<bool>,
    quit: Cell<bool>,

    window_size: Cell<Vec2>,
    window_title: Cell<[char; 256]>,

    mouse: Mouse,
    gamepad: Gamepad,
    keys: [Cell<DigitalButton>; 512],
    modifier: Cell<Modifier>,

    text_buffer: Cell<[char; 256]>,
    text_length: Cell<usize>,
}


impl Keyboard {

    pub fn new() -> Self {
        let mut keys = [Cell::new(Button::new()); 512];
        let modifier = Cell::new(Modifier::new());
        Self { keys, modifier }
    }

    pub fn key(&self, key: Key) -> &Cell<Button> {
        &self.keys[key as usize]
    }
}
