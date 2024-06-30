use core::cell::Cell;
use core::ops::{Deref, DerefMut};
use core::ptr::NonNull;

pub struct Arena {
    data: Box<[u8]>,
    offset: Cell<usize>,
    generation: Cell<usize>,
}

pub struct ArenaHandle<T> {
    ptr: NonNull<T>,
    len: usize,
    generation: usize,
}

impl<'a, T> Deref for ArenaHandle<T> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        unsafe { std::slice::from_raw_parts(self.ptr.as_ptr(), self.len) }
    }
}

impl<'a, T> DerefMut for ArenaHandle<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { std::slice::from_raw_parts_mut(self.ptr.as_ptr(), self.len) }
    }
}

impl Arena {
    pub fn new(size: usize) -> Arena {
        Arena {
            data: vec![0; size].into_boxed_slice(),
            offset: Cell::new(0),
            generation: Cell::new(0),
        }
    }

    pub fn allocate<T>(&self, len: usize) -> Option<ArenaHandle<T>> {
        let size = std::mem::size_of::<T>();
        let align = std::mem::align_of::<T>();
        let offset = (self.offset.get() + align - 1) & !(align - 1);
        let new_offset = offset + (size * len);

        if new_offset <= self.data.len() {
            let ptr = unsafe { NonNull::new_unchecked(&self.data[offset] as *const u8 as *mut T) };
            self.offset.set(new_offset);

            Some(ArenaHandle {
                ptr,
                len,
                generation: self.generation.get(),
            })
        } else {
            None
        }
    }

    pub fn clear(&self) {
        self.offset.set(0);
        self.generation.set(self.generation.get() + 1);
    }

    pub fn size(&self) -> usize {
        self.data.len()
    }

    pub fn occupied(&self) -> usize {
        self.offset.get()
    }

    pub fn is_full(&self) -> bool {
        self.offset.get() == self.data.len()
    }

    pub fn is_valid_ptr<T>(&self, handle: &ArenaHandle<T>) -> bool {
        if self.generation.get() == handle.generation {
            handle.ptr.as_ptr() >= &self.data[0] as *const u8 as *mut T
                && handle.ptr.as_ptr() < &self.data[self.data.len() - 1] as *const u8 as *mut T
        } else {
            false
        }
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

        let mut p1: ArenaHandle<Point> = arena.allocate(1).unwrap();
        let mut p2: ArenaHandle<Point> = arena.allocate(1).unwrap();

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
        let arena = Arena::new(1024);

        let mut entities: ArenaHandle<Entity> = arena.allocate(10).unwrap();

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
        let arena = Arena::new(std::mem::size_of::<Point>() * 2);

        let p1: ArenaHandle<Point> = arena.allocate(1).unwrap();
        let p2: ArenaHandle<Point> = arena.allocate(1).unwrap();
        let p3: Option<ArenaHandle<Point>> = arena.allocate::<Point>(1);

        assert!(p3.is_none());
        assert!(arena.is_full());

        assert!(arena.is_valid_ptr(&p1));
        assert!(arena.is_valid_ptr(&p2));

        arena.clear();
        assert_eq!(arena.occupied(), 0);

        assert!(!arena.is_valid_ptr(&p1));
        assert!(!arena.is_valid_ptr(&p2));

        let p4: ArenaHandle<Point> = arena.allocate(1).unwrap();
        let p5: ArenaHandle<Point> = arena.allocate(1).unwrap();
        let p6: Option<ArenaHandle<Point>> = arena.allocate(1);

        assert!(p6.is_none());
        assert!(arena.is_full());

        assert!(arena.is_valid_ptr(&p4));
        assert!(arena.is_valid_ptr(&p5));
    }

    #[test]
    fn test_arena_clear() {
        let arena = Arena::new(1024);

        let mut p1: ArenaHandle<Point> = arena.allocate(1).unwrap();
        let mut p2: ArenaHandle<Point> = arena.allocate(1).unwrap();

        p1[0] = Point { x: 1.0, y: 2.0 };
        p2[0] = Point { x: 3.0, y: 4.0 };

        assert_eq!(p1[0].x, 1.0);
        assert_eq!(p1[0].y, 2.0);

        assert_eq!(p2[0].x, 3.0);
        assert_eq!(p2[0].y, 4.0);

        assert_eq!(arena.occupied(), std::mem::size_of::<Point>() * 2);

        arena.clear();
        assert_eq!(arena.occupied(), 0);

        let mut p3: ArenaHandle<Point> = arena.allocate(1).unwrap();
        let mut p4: ArenaHandle<Point> = arena.allocate(1).unwrap();

        p3[0] = Point { x: 5.0, y: 6.0 };
        p4[0] = Point { x: 7.0, y: 8.0 };

        assert_eq!(p3[0].x, 5.0);
        assert_eq!(p3[0].y, 6.0);

        assert_eq!(p4[0].x, 7.0);
        assert_eq!(p4[0].y, 8.0);

        assert_eq!(arena.occupied(), std::mem::size_of::<Point>() * 2);
    }

    #[test]
    fn test_arena_handle_mut() {
        let arena = Arena::new(1024);

        let mut p: ArenaHandle<Point> = arena.allocate(1).unwrap();

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
