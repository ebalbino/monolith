use alloc::boxed::Box;
use alloc::vec;
use core::cell::Cell;
use core::cmp::Ordering;
use core::fmt::Write;
use core::ops::{Deref, DerefMut};

pub struct Arena {
    data: Box<[u8]>,
    offset: Cell<usize>,
}

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord)]
pub struct ArenaSlice<T> {
    arena: *const Arena,
    ptr: *mut T,
    len: usize,
}

#[derive(Debug, Clone, Eq)]
pub struct ArenaString {
    inner: ArenaSlice<u8>,
    len: usize,
}

impl<T> Deref for ArenaSlice<T> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        unsafe { core::slice::from_raw_parts(self.ptr, self.len) }
    }
}

impl<T> DerefMut for ArenaSlice<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { core::slice::from_raw_parts_mut(self.ptr, self.len) }
    }
}

impl<T> AsRef<[T]> for ArenaSlice<T> {
    fn as_ref(&self) -> &[T] {
        self.deref()
    }
}

impl<T> Clone for ArenaSlice<T> {
    fn clone(&self) -> Self {
        let new_ptr = unsafe { (*self.arena).push_slice(&self[..]).unwrap().as_ptr() as *mut T };

        ArenaSlice {
            arena: self.arena,
            ptr: new_ptr,
            len: self.len,
        }
    }
}

impl Deref for ArenaString {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        unsafe { core::str::from_utf8_unchecked(&self.inner) }
    }
}

impl DerefMut for ArenaString {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { core::str::from_utf8_unchecked_mut(&mut self.inner) }
    }
}

impl AsRef<str> for ArenaString {
    fn as_ref(&self) -> &str {
        self.deref()
    }
}

impl PartialEq for ArenaString {
    fn eq(&self, other: &Self) -> bool {
        self.deref() == other.deref()
    }
}

impl PartialOrd for ArenaString {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.deref().partial_cmp(other.deref())
    }
}

impl PartialEq<str> for ArenaString {
    fn eq(&self, other: &str) -> bool {
        self.deref() == other
    }
}

impl Write for ArenaString {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        if self.len() > s.len() {
            let slice = &mut self.inner[self.len..self.len + s.len()];
            slice.copy_from_slice(s.as_bytes());
            self.len += s.len();
            return Ok(());
        }

        Err(core::fmt::Error)
    }
}

impl ArenaString {
    pub fn from_slice(view: ArenaSlice<u8>) -> ArenaString {
        ArenaString {
            inner: view,
            len: 0,
        }
    }
}

impl Arena {
    pub fn new(size: usize) -> Arena {
        Arena {
            data: vec![0; size].into_boxed_slice(),
            offset: Cell::new(0),
        }
    }

    pub fn allocate<T>(&self, len: usize) -> Option<ArenaSlice<T>> {
        let size = core::mem::size_of::<T>();
        let align = core::mem::align_of::<T>();
        let offset = (self.offset.get() + align - 1) & !(align - 1);
        let new_offset = offset + (size * len);

        if new_offset <= self.data.len() {
            let ptr = &self.data[offset] as *const u8 as *mut T;
            self.offset.set(new_offset);

            Some(ArenaSlice {
                arena: self,
                ptr,
                len,
            })
        } else {
            None
        }
    }

    pub fn allocate_string(&self, len: usize) -> Option<ArenaString> {
        let inner = self.allocate(len)?;

        Some(ArenaString { inner, len: 0 })
    }

    pub fn push<T>(&self, value: T) -> Option<ArenaSlice<T>> {
        let size = core::mem::size_of::<T>();
        let align = core::mem::align_of::<T>();
        let offset = (self.offset.get() + align - 1) & !(align - 1);
        let new_offset = offset + size;

        if new_offset <= self.data.len() {
            let ptr = &self.data[offset] as *const u8 as *mut T;
            self.offset.set(new_offset);

            unsafe {
                ptr.write(value);
            }

            Some(ArenaSlice {
                arena: self,
                ptr,
                len: 1,
            })
        } else {
            None
        }
    }

    pub fn push_slice<T>(&self, values: &[T]) -> Option<ArenaSlice<T>> {
        let align = core::mem::align_of::<T>();
        let offset = (self.offset.get() + align - 1) & !(align - 1);
        let new_offset = offset + core::mem::size_of_val(values);

        if new_offset <= self.data.len() {
            let ptr = &self.data[offset] as *const u8 as *mut T;
            self.offset.set(new_offset);

            unsafe {
                ptr.copy_from_nonoverlapping(values.as_ptr(), values.len());
            }

            Some(ArenaSlice {
                arena: self,
                ptr,
                len: values.len(),
            })
        } else {
            None
        }
    }

    pub fn push_string(&self, string: &str) -> Option<ArenaString> {
        let inner = self.push_slice(string.as_bytes())?;
        Some(ArenaString { inner, len: 0 })
    }

    pub fn clear(&self) {
        self.offset.set(0);
    }

    pub fn size(&self) -> usize {
        self.data.len()
    }

    pub fn occupied(&self) -> usize {
        self.offset.get()
    }

    pub fn is_full(&self) -> bool {
        self.occupied() == self.data.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Copy, Clone)]
    struct Point {
        x: f64,
        y: f64,
    }

    #[derive(Debug)]
    struct Entity {
        position: Point,
        velocity: Point,
        acceleration: f64,
        dummy: u8,
    }

    #[test]
    fn test_arena() {
        let arena = Arena::new(1024);

        let mut p1: ArenaSlice<Point> = arena.allocate::<Point>(1).unwrap();
        let mut p2: ArenaSlice<Point> = arena.allocate::<Point>(1).unwrap();

        p1[0] = Point { x: 1.0, y: 2.0 };
        p2[0] = Point { x: 3.0, y: 4.0 };

        assert_eq!(p1[0].x, 1.0);
        assert_eq!(p1[0].y, 2.0);

        assert_eq!(p2[0].x, 3.0);
        assert_eq!(p2[0].y, 4.0);

        assert_eq!(arena.occupied(), core::mem::size_of::<Point>() * 2);
    }

    #[test]
    fn test_arena_x() {
        let arena = Arena::new(1024);

        let mut entities: ArenaSlice<Entity> = arena.allocate::<Entity>(10).unwrap();

        for entity in entities.iter_mut() {
            entity.position = Point { x: 1.0, y: 2.0 };
            entity.velocity = Point { x: 3.0, y: 4.0 };
            entity.acceleration = 5.0;
            entity.dummy = 0;
        }

        for entity in entities.iter() {
            assert_eq!(entity.position.x, 1.0);
            assert_eq!(entity.position.y, 2.0);

            assert_eq!(entity.velocity.x, 3.0);
            assert_eq!(entity.velocity.y, 4.0);

            assert_eq!(entity.acceleration, 5.0);
            assert_eq!(entity.dummy, 0);
        }

        assert_eq!(arena.occupied(), core::mem::size_of::<Entity>() * 10);
    }

    #[test]
    fn test_full_arena() {
        let arena = Arena::new(core::mem::size_of::<Point>() * 2);

        let _p1: ArenaSlice<Point> = arena.allocate::<Point>(1).unwrap();
        let _p2: ArenaSlice<Point> = arena.allocate::<Point>(1).unwrap();

        assert!(arena.is_full());
        arena.clear();
        assert_eq!(arena.occupied(), 0);

        let _p4: ArenaSlice<Point> = arena.allocate::<Point>(1).unwrap();
        let _p5: ArenaSlice<Point> = arena.allocate::<Point>(1).unwrap();
        assert!(arena.is_full());
    }

    #[test]
    fn test_arena_clear() {
        let arena = Arena::new(1024);

        let mut p1 = arena.allocate::<Point>(1).unwrap();
        let mut p2 = arena.allocate::<Point>(1).unwrap();

        p1[0] = Point { x: 1.0, y: 2.0 };
        p2[0] = Point { x: 3.0, y: 4.0 };

        assert_eq!(p1[0].x, 1.0);
        assert_eq!(p1[0].y, 2.0);

        assert_eq!(p2[0].x, 3.0);
        assert_eq!(p2[0].y, 4.0);

        assert_eq!(arena.occupied(), core::mem::size_of::<Point>() * 2);

        arena.clear();
        assert_eq!(arena.occupied(), 0);

        let mut p3 = arena.allocate::<Point>(1).unwrap();
        let mut p4 = arena.allocate::<Point>(1).unwrap();

        p3[0] = Point { x: 5.0, y: 6.0 };
        p4[0] = Point { x: 7.0, y: 8.0 };

        assert_eq!(p3[0].x, 5.0);
        assert_eq!(p3[0].y, 6.0);

        assert_eq!(p4[0].x, 7.0);
        assert_eq!(p4[0].y, 8.0);

        assert_eq!(arena.occupied(), core::mem::size_of::<Point>() * 2);
    }

    #[test]
    fn test_arena_handle_mut() {
        let arena = Arena::new(1024);

        let mut p = arena.allocate::<Point>(1).unwrap();

        for point in p.iter_mut() {
            point.x = 1.0;
            point.y = 2.0;
        }

        for point in p.iter() {
            assert_eq!(point.x, 1.0);
            assert_eq!(point.y, 2.0);
        }
    }
}
