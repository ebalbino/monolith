use core::cell::Cell;
use libc::{CLOCK_MONOTONIC, timespec, clock_gettime, clock_getres};

#[derive(Debug, Clone)]
pub struct Clock {
    ticks: Cell<u64>,
    nanoseconds: Cell<u64>,
    microseconds: Cell<u64>,
    milliseconds: Cell<u64>,
    seconds: Cell<f64>,

    delta_ticks: Cell<u64>,
    delta_nanoseconds: Cell<u64>,
    delta_microseconds: Cell<u64>,
    delta_milliseconds: Cell<u64>,
    delta_seconds: Cell<f64>,

    initial_ticks: u64,
    ticks_per_second: u64,
}

fn get_time() -> u64 {
    let mut ts = timespec { tv_sec: 0, tv_nsec: 0 };
    unsafe {
        clock_gettime(CLOCK_MONOTONIC, &mut ts);
    }
    ts.tv_sec as u64 * 1_000_000_000 + ts.tv_nsec as u64
}

fn get_resolution() -> u64 {
    let mut ts = timespec { tv_sec: 0, tv_nsec: 0 };
    unsafe {
        clock_getres(CLOCK_MONOTONIC, &mut ts);
    }
    ts.tv_sec as u64 + ts.tv_nsec as u64
}

impl Clock {
    pub fn monotonic() -> Self {
        let ticks_per_second = get_resolution();
        let ticks = get_time();

        Clock {
            ticks: Cell::new(0),
            nanoseconds: Cell::new(0),
            microseconds: Cell::new(0),
            milliseconds: Cell::new(0),
            seconds: Cell::new(0.0),

            delta_ticks: Cell::new(0),
            delta_nanoseconds: Cell::new(0),
            delta_microseconds: Cell::new(0),
            delta_milliseconds: Cell::new(0),
            delta_seconds: Cell::new(0.0),

            initial_ticks: ticks,
            ticks_per_second: ticks_per_second,
        }
    }

    pub fn update(&self) {
        let current_ticks = get_time();
        let delta_ticks = (current_ticks - self.initial_ticks) - self.ticks.get();
        let ticks = current_ticks - self.initial_ticks;

        self.delta_ticks.set(delta_ticks);
        self.ticks.set(ticks);

        self.delta_nanoseconds.set(delta_ticks / self.ticks_per_second);
        self.delta_microseconds.set(self.delta_nanoseconds() / 1_000);
        self.delta_milliseconds.set(self.delta_microseconds() / 1_000);
        self.delta_seconds.set(self.delta_milliseconds() as f64 / 1_000.0);

        self.nanoseconds.set(ticks / self.ticks_per_second);
        self.microseconds.set(self.nanoseconds() / 1_000);
        self.milliseconds.set(self.microseconds() / 1_000);
        self.seconds.set(self.milliseconds() as f64 / 1_000.0);
    }

    pub fn ticks(&self) -> u64 {
        self.ticks.get()
    }

    pub fn nanoseconds(&self) -> u64 {
        self.nanoseconds.get()
    }

    pub fn microseconds(&self) -> u64 {
        self.microseconds.get()
    }

    pub fn milliseconds(&self) -> u64 {
        self.milliseconds.get()
    }

    pub fn seconds(&self) -> f64 {
        self.seconds.get()
    }

    pub fn delta_ticks(&self) -> u64 {
        self.delta_ticks.get()
    }

    pub fn delta_nanoseconds(&self) -> u64 {
        self.delta_nanoseconds.get()
    }

    pub fn delta_microseconds(&self) -> u64 {
        self.delta_microseconds.get()
    }

    pub fn delta_milliseconds(&self) -> u64 {
        self.delta_milliseconds.get()
    }

    pub fn delta_seconds(&self) -> f64 {
        self.delta_seconds.get()
    }

    pub fn resolution(&self) -> u64 {
        self.ticks_per_second
    }
}
