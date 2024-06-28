use core::ptr::NonNull;
use core::ops::{Deref, DerefMut};

pub struct Arena {
    data: Box<[u8]>,
    offset: usize,
}

pub struct ArenaHandle<T> {
    ptr: NonNull<T>,
    len: usize,
}

impl<T> Deref for ArenaHandle<T> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        unsafe { std::slice::from_raw_parts(self.ptr.as_ptr(), self.len) }
    }
}

impl<T> DerefMut for ArenaHandle<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { std::slice::from_raw_parts_mut(self.ptr.as_ptr(), self.len) }
    }
}

impl Arena {
    pub fn new(size: usize) -> Arena {
        Arena {
            data: vec![0; size].into_boxed_slice(),
            offset: 0,
        }
    }

    pub fn allocate<T>(&mut self, count: usize) -> Option<ArenaHandle<T>> {
        let size = std::mem::size_of::<T>();
        let align = std::mem::align_of::<T>();
        let offset = (self.offset + align - 1) & !(align - 1);
        let new_offset = offset + (size * count);

        if new_offset <= self.data.len() {
            let ptr = &mut self.data[offset] as *mut u8 as *mut T;
            self.offset = new_offset;

            Some(ArenaHandle {
                ptr: unsafe { NonNull::new_unchecked(ptr) },
                len: count,
            })
        } else {
            None
        }
    }

    pub fn clear(&mut self) {
        self.offset = 0;
    }

    pub fn occupied(&self) -> usize {
        self.offset
    }

    pub fn is_full(&self) -> bool {
        self.offset == self.data.len()
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
        let mut arena = Arena::new(1024);

        let mut p1 : ArenaHandle<Point> = arena.allocate(1).unwrap();
        let mut p2 : ArenaHandle<Point> = arena.allocate(1).unwrap();

        p1[0] = Point { x: 1.0, y: 2.0 };
        p2[0] = Point { x: 3.0, y: 4.0 };

        assert_eq!(p1[0].x, 1.0);
        assert_eq!(p1[0].y, 2.0);

        assert_eq!(p2[0].x, 3.0);
        assert_eq!(p2[0].y, 4.0);

        assert_eq!(arena.occupied(), std::mem::size_of::<Point>() * 2);
    }

    #[test]
    fn test_arena_x() {
        let mut arena = Arena::new(1024);

        let mut entities : ArenaHandle<Entity> = arena.allocate(10).unwrap();

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

        assert_eq!(arena.occupied(), std::mem::size_of::<Entity>() * 10);
    }

    #[test]
    fn test_full_arena() {
        let mut arena = Arena::new(std::mem::size_of::<Point>() * 2);

        let _p1 : ArenaHandle<Point> = arena.allocate(1).unwrap();
        let _p2 : ArenaHandle<Point> = arena.allocate(1).unwrap();
        let p3 : Option<ArenaHandle<Point>> = arena.allocate::<Point>(1);

        assert!(p3.is_none());
        assert!(arena.is_full());

        arena.clear();
        assert_eq!(arena.occupied(), 0);

        let _p4 : ArenaHandle<Point> = arena.allocate(1).unwrap();
        let _p5 : ArenaHandle<Point> = arena.allocate(1).unwrap();
        let p6 : Option<ArenaHandle<Point>> = arena.allocate(1);

        assert!(p6.is_none());
        assert!(arena.is_full());
    }
}
