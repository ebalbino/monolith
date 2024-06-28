use crate::arena::Arena;
use core::ops::{Deref};
use std::cmp::PartialEq;

pub struct Pool<T> {
    arena: Arena,
    lookup: Vec<Intern<T>>,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Intern<T> {
    data: *const u8,
    phantom: core::marker::PhantomData<T>,
}

pub struct StrPool {
    arena: Arena,
    lookup: Vec<StrIntern>,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct StrIntern {
    data: *const u8,
    len: usize,
}

impl<T> Deref for Intern<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*(self.data as *const T) }
    }
}

impl Deref for StrIntern {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        unsafe {
            let slice = core::slice::from_raw_parts(self.data, self.len);
            std::str::from_utf8_unchecked(slice)
        }
    }
}

impl<T: Copy + PartialEq> Pool<T> {
    pub fn new(size: usize) -> Pool<T> {
        Pool {
            arena: Arena::new(size),
            lookup: Vec::new(),
        }
    }

    pub fn intern(&mut self, value: &T) -> Option<Intern<T>> {
        for intern in &self.lookup {
            if unsafe { *(intern.data as *const T) } == *value {
                return Some(*intern);
            }
        }

        let len = core::mem::size_of::<T>();
        let ptr = self.arena.allocate(len)?;

        unsafe {
            let data = ptr.as_ptr() as *mut T;
            data.write(*value);
        }

        let intern = Intern {
            data: ptr.as_ptr() as *const u8,
            phantom: core::marker::PhantomData,
        };

        self.lookup.push(intern);
        Some(intern)
    }

    pub fn clear(&mut self) {
        self.arena.clear();
        self.lookup.clear();
    }
}

impl StrPool {
    pub fn new(size: usize) -> StrPool {
        StrPool {
            arena: Arena::new(size),
            lookup: Vec::new(),
        }
    }

    pub fn intern(&mut self, value: &str) -> Option<StrIntern> {
        for intern in &self.lookup {
            if intern.len == value.len() {
                let slice = unsafe { core::slice::from_raw_parts(intern.data, intern.len) };
                if slice == value.as_bytes() {
                    return Some(*intern);
                }
            }
        }

        let len = value.len();
        let ptr = self.arena.allocate(len)?;

        unsafe {
            let data = ptr.as_ptr() as *mut u8;
            data.copy_from(value.as_ptr(), len);
        }

        let intern = StrIntern {
            data: ptr.as_ptr(),
            len,
        };

        self.lookup.push(intern);
        Some(intern)
    }

    pub fn clear(&mut self) {
        self.arena.clear();
        self.lookup.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, PartialEq, Clone, Copy)]
    struct Test {
        x: i32,
        y: i32,
    }

    #[test]
    fn test_pool() {
        let mut pool = Pool::new(1024);
        let a = Test { x: 1, y: 2 };
        let b = Test { x: 3, y: 4 };

        let a_intern = pool.intern(&a).unwrap();
        let b_intern = pool.intern(&b).unwrap();

        assert_ne!(a_intern, b_intern);

        let c_intern = pool.intern(&a).unwrap();
        let d_intern = pool.intern(&b).unwrap();

        assert_eq!(a_intern.data, c_intern.data);
        assert_eq!(b_intern.data, d_intern.data);
        assert_ne!(c_intern.data, d_intern.data);

        let e = Test { x: 1, y: 2 };
        let e_intern = pool.intern(&e).unwrap();
        assert_eq!(a_intern.data, e_intern.data);
        assert_eq!(c_intern.data, e_intern.data);
        assert_eq!(*e_intern, e);
    }

    #[test]
    fn test_str_pool() {
        let mut pool = StrPool::new(1024);
        let a = "hello";
        let b = "world";

        let a_intern = pool.intern(a).unwrap();
        let b_intern = pool.intern(b).unwrap();

        assert_ne!(a_intern.data, b_intern.data);

        let c_intern = pool.intern(a).unwrap();
        let d_intern = pool.intern(b).unwrap();

        assert_eq!(a_intern.data, c_intern.data);
        assert_eq!(b_intern.data, d_intern.data);
        assert_ne!(c_intern.data, d_intern.data);
    }
}
