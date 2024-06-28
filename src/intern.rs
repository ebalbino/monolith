use crate::arena::Arena;
use core::ops::{Deref, DerefMut};

pub struct StrInterner {
    arena: Arena,
    strings: Vec<StrIntern>,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct StrIntern {
    str: *const u8,
    len: usize,
}

impl Deref for StrIntern {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        unsafe {
            let slice = std::slice::from_raw_parts(self.str, self.len);
            std::str::from_utf8_unchecked(slice)
        }
    }
}

impl DerefMut for StrIntern {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe {
            let slice = std::slice::from_raw_parts_mut(self.str as *mut u8, self.len);
            std::str::from_utf8_unchecked_mut(slice)
        }
    }
}

impl StrInterner {
    pub fn new(size: usize) -> StrInterner {
        StrInterner {
            arena: Arena::new(size),
            strings: Vec::new(),
        }
    }

    pub fn intern(&mut self, s: &str) -> StrIntern {
        for intern in self.strings.iter() {
            unsafe {
                let slice = std::slice::from_raw_parts(intern.str, intern.len);
                if slice == s.as_bytes() {
                    return *intern;
                }
            }
        }

        let len = s.len();
        let mut ptr = self.arena.allocate::<u8>(len).unwrap();

        ptr.copy_from_slice(s.as_bytes());

        let intern = StrIntern {
            str: ptr.as_ptr(),
            len,
        };

        self.strings.push(intern);
        intern
    }

    pub fn clear(&mut self) {
        self.arena.clear();
        unsafe { self.strings.set_len(0) };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_str_interner() {
        let mut interner = StrInterner::new(1024);

        let s1 = interner.intern("hello");
        let s2 = interner.intern("world");

        assert_ne!(s1, s2);

        let s3 = interner.intern("hello");
        let s4 = interner.intern("world");

        assert_eq!(s1, s3);
        assert_eq!(s2, s4);

        interner.clear();

        let s5 = interner.intern("new");
        let s6 = interner.intern("string");

        assert_ne!(s1, s5);
        assert_ne!(s2, s6);

        let s7 = interner.intern("new");
        let s8 = interner.intern("string");

        assert_eq!(s5, s7);
        assert_eq!(s6, s8);
    }
}
