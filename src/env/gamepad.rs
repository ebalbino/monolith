use crate::env::{AnalogButton, Button, Stick, Switch};
use core::cell::Cell;

pub struct Gamepad {
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
