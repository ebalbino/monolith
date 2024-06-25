use core::ptr::NonNull;

struct Arena {
    data: Box<[u8]>,
    offset: usize,
}

impl Arena {
    fn new(size: usize) -> Arena {
        Arena {
            data: vec![0; size].into_boxed_slice(),
            offset: 0,
        }
    }

    fn allocate<T>(&mut self) -> Option<NonNull<T>> {
        let size = std::mem::size_of::<T>();
        let align = std::mem::align_of::<T>();
        let offset = (self.offset + align - 1) & !(align - 1);
        let new_offset = offset + size;

        if new_offset <= self.data.len() {
            let ptr = &mut self.data[offset] as *mut u8 as *mut T;
            self.offset = new_offset;
            Some(unsafe { NonNull::new_unchecked(ptr) })
        } else {
            None
        }
    }

    fn allocate_x<T>(&mut self, count: usize) -> Option<&mut [T]> {
        let size = std::mem::size_of::<T>();
        let align = std::mem::align_of::<T>();
        let offset = (self.offset + align - 1) & !(align - 1);
        let new_offset = offset + (size * count);

        if new_offset <= self.data.len() {
            let ptr = &mut self.data[offset] as *mut u8 as *mut T;
            self.offset = new_offset;
            Some(unsafe { std::slice::from_raw_parts_mut(ptr, count) })
        } else {
            None
        }
    }

    fn clear(&mut self) {
        self.offset = 0;
    }

    fn occupied(&self) -> usize {
        self.offset
    }

    fn is_full(&self) -> bool {
        self.offset == self.data.len()
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct StrIntern {
    str: *const u8,
    len: usize,
}

struct StrInterner {
    arena: Arena,
    strings: Vec<StrIntern>,
}

impl StrInterner {
    fn new(size: usize) -> StrInterner {
        StrInterner {
            arena: Arena::new(size),
            strings: Vec::new(),
        }
    }

    fn intern(&mut self, s: &str) -> StrIntern {
        for intern in self.strings.iter() {
            unsafe {
                let slice = std::slice::from_raw_parts(intern.str, intern.len);
                if slice == s.as_bytes() {
                    return *intern;
                }
            }
        }

        let len = s.len();
        let ptr = self.arena.allocate_x::<u8>(len).unwrap();

        ptr.copy_from_slice(s.as_bytes());

        let intern = StrIntern {
            str: ptr.as_ptr(),
            len,
        };

        self.strings.push(intern);
        intern
    }

    fn clear(&mut self) {
        self.arena.clear();
        unsafe { self.strings.set_len(0) };
    }
}

#[inline]
fn kilobytes(bytes: usize) -> usize {
    bytes * 1024
}

#[inline]
fn megabytes(bytes: usize) -> usize {
    kilobytes(bytes) * 1024
}

fn main() {
    let mut _arena = Arena::new(megabytes(2));
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

        let p1 : NonNull<Point> = arena.allocate().unwrap();
        let p2 : NonNull<Point> = arena.allocate::<Point>().unwrap();

        unsafe {
            p1.as_ptr().write(Point { x: 1.0, y: 2.0 });
            p2.as_ptr().write(Point { x: 3.0, y: 4.0 });

            assert_eq!(p1.as_ref().x, 1.0);
            assert_eq!(p1.as_ref().y, 2.0);

            assert_eq!(p2.as_ref().x, 3.0);
            assert_eq!(p2.as_ref().y, 4.0);
        }

        assert_eq!(arena.occupied(), std::mem::size_of::<Point>() * 2);
    }

    #[test]
    fn test_arena_x() {
        let mut arena = Arena::new(1024);

        let entities : &mut [Entity] = arena.allocate_x(10).unwrap();

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

        let _p1 : NonNull<Point> = arena.allocate().unwrap();
        let _p2 : NonNull<Point> = arena.allocate().unwrap();
        let p3 : Option<NonNull<Point>> = arena.allocate::<Point>();

        assert!(p3.is_none());
        assert!(arena.is_full());

        arena.clear();
        assert_eq!(arena.occupied(), 0);

        let _p4 : NonNull<Point> = arena.allocate().unwrap();
        let _p5 : NonNull<Point> = arena.allocate().unwrap();
        let p6 : Option<NonNull<Point>> = arena.allocate();

        assert!(p6.is_none());
        assert!(arena.is_full());
    }

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
