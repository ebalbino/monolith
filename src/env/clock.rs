use core::cell::Cell;
use libc::{clock_getres, clock_gettime, timespec, CLOCK_MONOTONIC};

fn get_time() -> u64 {
    let mut ts = timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    unsafe {
        clock_gettime(CLOCK_MONOTONIC, &mut ts);
    }
    ts.tv_sec as u64 * 1_000_000_000 + ts.tv_nsec as u64
}

fn get_resolution() -> u64 {
    let mut ts = timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    unsafe {
        clock_getres(CLOCK_MONOTONIC, &mut ts);
    }
    ts.tv_sec as u64 + ts.tv_nsec as u64
}

pub struct Instant {
    ticks: u64,
    resolution: u64,
}

pub struct Clock {
    resolution: u64,
    start: u64,
    current: Cell<u64>,
    delta: Cell<u64>,
}

impl Instant {
    pub fn new(ticks: u64, resolution: u64) -> Self {
        Instant { ticks, resolution }
    }

    pub fn ticks(&self) -> u64 {
        self.ticks
    }

    pub fn nanoseconds(&self) -> u64 {
        self.ticks / self.resolution
    }

    pub fn microseconds(&self) -> u64 {
        self.nanoseconds() / 1_000
    }

    pub fn milliseconds(&self) -> u64 {
        self.microseconds() / 1_000
    }

    pub fn seconds(&self) -> f64 {
        self.milliseconds() as f64 / 1_000.0
    }
}

impl Clock {
    pub fn new() -> Self {
        let resolution = get_resolution();
        let start = get_time();

        Self {
            start,
            resolution,
            current: Cell::new(0),
            delta: Cell::new(0),
        }
    }

    pub fn update(&self) {
        let current = get_time();
        let start = self.start;

        self.delta.set((current - start) - self.current.get());
        self.current.set(current - start);
    }

    pub fn now(&self) -> Instant {
        Instant::new(self.current.get(), self.resolution)
    }

    pub fn resolution(&self) -> u64 {
        self.resolution
    }
}
