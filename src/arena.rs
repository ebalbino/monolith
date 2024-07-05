use core::ops::{Deref, DerefMut};
use core::cell::Cell;
use alloc::boxed::Box;
use alloc::vec;

pub struct Arena {
    data: Box<[u8]>,
    offset: Cell<usize>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ArenaHandle<'a, T> {
    ptr: *mut  T,
    len: usize,
    marker: core::marker::PhantomData<&'a T>,
}

impl<'a, T> Deref for ArenaHandle<'a, T> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        unsafe { core::slice::from_raw_parts(self.ptr, self.len) }
    }
}

impl<'a, T> DerefMut for ArenaHandle<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { core::slice::from_raw_parts_mut(self.ptr, self.len) }
    }
}

impl<'a> Arena {
    pub fn new(size: usize) -> Arena {
        Arena {
            data: vec![0; size].into_boxed_slice(),
            offset: Cell::new(0),
        }
    }

    pub fn allocate<T>(&'a self, len: usize) -> Option<ArenaHandle<'a, T>> {
        let size = core::mem::size_of::<T>();
        let align = core::mem::align_of::<T>();
        let offset = (self.offset.get() + align - 1) & !(align - 1);
        let new_offset = offset + (size * len);

        if new_offset <= self.data.len() {
            let ptr = &self.data[offset] as *const u8 as *mut T;
            self.offset.set(new_offset);

            Some(ArenaHandle {
                ptr,
                len,
                marker: core::marker::PhantomData,
            })
        } else {
            None
        }
    }

    pub fn push<T>(&'a self, value: T) -> Option<ArenaHandle<'a, T>> {
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

            Some(ArenaHandle {
                ptr,
                len: 1,
                marker: core::marker::PhantomData,
            })
        } else {
            None
        }
    }

    pub fn push_slice<T>(&'a self, values: &[T]) -> Option<ArenaHandle<'a, T>> {
        let size = core::mem::size_of::<T>();
        let align = core::mem::align_of::<T>();
        let offset = (self.offset.get() + align - 1) & !(align - 1);
        let new_offset = offset + (size * values.len());

        if new_offset <= self.data.len() {
            let ptr = &self.data[offset] as *const u8 as *mut T;
            self.offset.set(new_offset);

            unsafe {
                ptr.copy_from_nonoverlapping(values.as_ptr(), values.len());
            }

            Some(ArenaHandle {
                ptr,
                len: values.len(),
                marker: core::marker::PhantomData,
            })
        } else {
            None
        }
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

        let mut p1: ArenaHandle<Point> = arena.allocate::<Point>(1).unwrap();
        let mut p2: ArenaHandle<Point> = arena.allocate::<Point>(1).unwrap();

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

        let mut entities: ArenaHandle<Entity> = arena.allocate::<Entity>(10).unwrap();

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
        let mut arena = Arena::new(core::mem::size_of::<Point>() * 2);

        let _p1: ArenaHandle<Point> = arena.allocate::<Point>(1).unwrap();
        let _p2: ArenaHandle<Point> = arena.allocate::<Point>(1).unwrap();

        assert!(arena.is_full());
        arena.clear();
        assert_eq!(arena.occupied(), 0);

        let _p4: ArenaHandle<Point> = arena.allocate::<Point>(1).unwrap();
        let _p5: ArenaHandle<Point> = arena.allocate::<Point>(1).unwrap();
        assert!(arena.is_full());
    }

    #[test]
    fn test_arena_clear() {
        let mut arena = Arena::new(1024);

        let mut p1: ArenaHandle<Point> = arena.allocate::<Point>(1).unwrap();
        let mut p2: ArenaHandle<Point> = arena.allocate::<Point>(1).unwrap();

        p1[0] = Point { x: 1.0, y: 2.0 };
        p2[0] = Point { x: 3.0, y: 4.0 };

        assert_eq!(p1[0].x, 1.0);
        assert_eq!(p1[0].y, 2.0);

        assert_eq!(p2[0].x, 3.0);
        assert_eq!(p2[0].y, 4.0);

        assert_eq!(arena.occupied(), core::mem::size_of::<Point>() * 2);

        arena.clear();
        assert_eq!(arena.occupied(), 0);

        let mut p3: ArenaHandle<Point> = arena.allocate::<Point>(1).unwrap();
        let mut p4: ArenaHandle<Point> = arena.allocate::<Point>(1).unwrap();

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
