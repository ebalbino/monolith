use core::ops::Sub;
use core::cell::Cell;

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
        self.delta.set(value - current);
        self.value.set(value);
    }
}
