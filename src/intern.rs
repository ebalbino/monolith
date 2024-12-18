use crate::arena::Arena;
use alloc::vec::Vec;
use core::cell::RefCell;
use core::cmp::PartialEq;
use core::ops::Deref;

pub struct StrPool {
    arena: RefCell<Arena>,
    lookup: RefCell<Vec<StrIntern>>,
}

#[derive(Debug, PartialEq, Clone, Copy)]
struct StrIntern {
    data: *const u8,
    len: usize,
}

impl Deref for StrIntern {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        unsafe {
            core::str::from_utf8_unchecked(core::slice::from_raw_parts(
                self.data,
                self.len,
            ))
        }
    }
}

impl StrPool {
    pub fn new(size: usize) -> StrPool {
        StrPool {
            arena: RefCell::new(Arena::new(size)),
            lookup: RefCell::new(Vec::new()),
        }
    }

    pub fn intern<'a>(&self, value: &'a str) -> Option<&'a str> {
        for intern in self.lookup.borrow().iter() {
            if intern.len == value.len() &&  intern.as_bytes() == value.as_bytes() {
                    let data = unsafe {
                        core::str::from_utf8_unchecked(core::slice::from_raw_parts(
                            intern.as_ptr(),
                            intern.len,
                        ))
                    };
                    return Some(data);
            }
        }

        let arena = self.arena.borrow();
        let string = arena.push_string(value)?;
        let data = unsafe {
            core::str::from_utf8_unchecked(core::slice::from_raw_parts(
                string.as_ptr(),
                string.len(),
            ))
        };

        self.lookup.borrow_mut().push(StrIntern {
            data: string.as_ptr(),
            len: string.len(),
        });

        Some(data)
    }

    pub fn occupied(&self) -> usize {
        self.arena.borrow().occupied()
    }

    pub fn len(&self) -> usize {
        self.lookup.borrow().len()
    }

    pub fn is_empty(&self) -> bool {
        self.lookup.borrow().is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_str_pool() {
        let pool = StrPool::new(1024);
        let a = "hello";
        let b = "world";
        let e = "goodbye";
        let f = "world";

        let a_intern: &str = &pool.intern(a).unwrap();
        let b_intern: &str = &pool.intern(b).unwrap();
        let c_intern: &str = &pool.intern(a).unwrap();
        let d_intern: &str = &pool.intern(b).unwrap();
        let e_intern: &str = &pool.intern(e).unwrap();
        let f_intern: &str = &pool.intern(f).unwrap();

        assert_ne!(a_intern, b_intern);
        assert_eq!(a_intern, c_intern);
        assert_eq!(b_intern, d_intern);
        assert_ne!(c_intern, d_intern);
        assert_ne!(a_intern, e_intern);
        assert_eq!(b_intern, f_intern);
        assert_ne!(a_intern.as_ptr(), b_intern.as_ptr());
        assert_eq!(a_intern.as_ptr(), c_intern.as_ptr());
        assert_eq!(b_intern.as_ptr(), d_intern.as_ptr());
        assert_ne!(c_intern.as_ptr(), d_intern.as_ptr());
        assert_ne!(a_intern.as_ptr(), e_intern.as_ptr());
        assert_eq!(b_intern.as_ptr(), f_intern.as_ptr());
        assert_eq!(pool.len(), 3);
        assert_eq!(pool.occupied(), 17);
    }
}
