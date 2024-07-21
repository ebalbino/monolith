#![no_std]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

extern crate alloc;

pub mod arena;
pub mod draw;
pub mod intern;
pub mod math;
pub mod platform;

pub use arena::{Arena, ArenaView};
pub use draw::*;
pub use intern::StrPool;
pub use math::*;
