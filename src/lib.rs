#![no_std]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

extern crate alloc;

pub mod bindings;
pub mod arena;
pub mod intern;
pub mod math;
pub mod draw;
pub mod input;

pub use arena::{Arena, ArenaView};
pub use intern::{StrPool};
pub use math::*;
pub use draw::*;
