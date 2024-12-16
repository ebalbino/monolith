use crate::arena::{Arena, ArenaSlice};

pub struct Texture {
    width: u32,
    height: u32,
    channels: u32,
    data: ArenaSlice<u8>,
}

impl Texture {
    pub fn new(arena: &Arena, width: u32, height: u32, channels: u32) -> Self {
        let data = arena
            .allocate::<u8>(width as usize * height as usize * channels as usize)
            .unwrap();
        Self {
            width,
            height,
            channels,
            data,
        }
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn channels(&self) -> u32 {
        self.channels
    }

    pub fn data(&self) -> &[u8] {
        unsafe { core::slice::from_raw_parts(self.data.as_ptr(), self.data.len()) }
    }

    pub fn data_mut(&mut self) -> &mut [u8] {
        unsafe { core::slice::from_raw_parts_mut(self.data.as_mut_ptr(), self.data.len()) }
    }
}
