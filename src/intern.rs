use core::cmp::PartialEq;
use core::ops::Deref;
use core::cell::RefCell;
use crate::arena::Arena;

pub struct StrPool<'a> {
    arena: RefCell<Arena>,
    lookup: RefCell<Vec<StrIntern<'a>>>,
}

#[derive(Debug, PartialEq, Clone, Copy)]
struct StrIntern<'a> {
    data: &'a str,
}

impl<'a> Deref for StrIntern<'a> {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.data
    }
}

impl<'a> StrPool<'a> {
    pub fn new(size: usize) -> StrPool<'a> {
        StrPool {
            arena: RefCell::new(Arena::new(size)),
            lookup: RefCell::new(Vec::new()),
        }
    }

    pub fn intern(&'a self, value: &'a str) -> Option<&'a str> {
        for intern in self.lookup.borrow().iter() {
            if intern.data.len() == value.len() {
                if intern.data.as_bytes() == value.as_bytes() {
                    return Some(intern.data);
                }
            }
        }

        let arena = self.arena.borrow();
        let slice = arena.push_slice(value.as_bytes())?;
        let data = unsafe {
            core::str::from_utf8_unchecked(core::slice::from_raw_parts(slice.as_ptr() as *const u8, slice.len()))
        };

        self.lookup.borrow_mut().push(StrIntern { data });
        Some(data)
    }

    pub fn occupied(&self) -> usize {
        self.arena.borrow().occupied()
    }

    pub fn len(&self) -> usize {
        self.lookup.borrow().len()
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
        assert_eq!(pool.len(), 3);
        assert_eq!(pool.occupied(), 17);
    }
}
