use crate::arena::{Arena, ArenaView};

struct Texture {
    width: u32,
    height: u32,
    channels: u32,
    data: ArenaView<u8>,
}
