use core::default::Default;
use glam::{BVec4, IVec2, IVec3, IVec4, UVec2, UVec3, UVec4};
use core::ops::{Mul, Add, Sub};

pub type Vec2 = glam::Vec2;
pub type Vec3 = glam::Vec3;
pub type Vec4 = glam::Vec4;
pub type Vec2i = IVec2;
pub type Vec3i = IVec3;
pub type Vec4i = IVec4;
pub type Vec2u = UVec2;
pub type Vec3u = UVec3;
pub type Vec4u = UVec4;
pub type Vec4b = BVec4;
pub type Point = Vec2;

pub struct BoundingBox2D {
    min: Vec2,
    max: Vec2,
}

pub struct BoundingBox3D {
    min: Vec3,
    max: Vec3,
}

pub struct Ray2D {
    origin: Vec2,
    direction: Vec2,
}

pub struct Ray3D {
    origin: Vec3,
    direction: Vec3,
}

pub fn vec2(x: f32, y: f32) -> Vec2 {
    Vec2::new(x, y)
}

pub fn vec3(x: f32, y: f32, z: f32) -> Vec3 {
    Vec3::new(x, y, z)
}

pub fn vec4(x: f32, y: f32, z: f32, w: f32) -> Vec4 {
    Vec4::new(x, y, z, w)
}

impl Default for BoundingBox2D {
    fn default() -> Self {
        BoundingBox2D {
            min: Vec2::ZERO,
            max: Vec2::ZERO,
        }
    }
}

impl Default for BoundingBox3D {
    fn default() -> Self {
        BoundingBox3D {
            min: Vec3::ZERO,
            max: Vec3::ZERO,
        }
    }
}

impl BoundingBox2D {
    pub fn new(min: Vec2, max: Vec2) -> Self {
        BoundingBox2D { min, max }
    }

    pub fn center(&self) -> Vec2 {
        (self.min + self.max) / 2.0
    }

    pub fn size(&self) -> Vec2 {
        self.max - self.min
    }

    pub fn contains(&self, point: Vec2) -> bool {
        point.x >= self.min.x && point.x <= self.max.x && point.y >= self.min.y && point.y <= self.max.y
    }

    pub fn intersects(&self, other: &BoundingBox2D) -> bool {
        self.min.x <= other.max.x && self.max.x >= other.min.x && self.min.y <= other.max.y && self.max.y >= other.min.y
    }

    pub fn expand(&mut self, point: Vec2) {
        if point.x < self.min.x {
            self.min.x = point.x;
        } else if point.x > self.max.x {
            self.max.x = point.x;
        }

        if point.y < self.min.y {
            self.min.y = point.y;
        } else if point.y > self.max.y {
            self.max.y = point.y;
        }
    }

    pub fn expand_to_fit(&mut self, other: &BoundingBox2D) {
        if other.min.x < self.min.x {
            self.min.x = other.min.x;
        }

        if other.max.x > self.max.x {
            self.max.x = other.max.x;
        }

        if other.min.y < self.min.y {
            self.min.y = other.min.y;
        }

        if other.max.y > self.max.y {
            self.max.y = other.max.y;
        }
    }
}

impl BoundingBox3D {
    pub fn new(min: Vec3, max: Vec3) -> Self {
        BoundingBox3D { min, max }
    }

    pub fn center(&self) -> Vec3 {
        (self.min + self.max) / 2.0
    }

    pub fn size(&self) -> Vec3 {
        self.max - self.min
    }

    pub fn contains(&self, point: Vec3) -> bool {
        point.x >= self.min.x && point.x <= self.max.x && point.y >= self.min.y && point.y <= self.max.y && point.z >= self.min.z && point.z <= self.max.z
    }

    pub fn intersects(&self, other: &BoundingBox3D) -> bool {
        self.min.x <= other.max.x && self.max.x >= other.min.x && self.min.y <= other.max.y && self.max.y >= other.min.y && self.min.z <= other.max.z && self.max.z >= other.min.z
    }

    pub fn expand(&mut self, point: Vec3) {
        if point.x < self.min.x {
            self.min.x = point.x;
        } else if point.x > self.max.x {
            self.max.x = point.x;
        }

        if point.y < self.min.y {
            self.min.y = point.y;
        } else if point.y > self.max.y {
            self.max.y = point.y;
        }

        if point.z < self.min.z {
            self.min.z = point.z;
        } else if point.z > self.max.z {
            self.max.z = point.z;
        }
    }

    pub fn expand_to_fit(&mut self, other: &BoundingBox3D) {
        if other.min.x < self.min.x {
            self.min.x = other.min.x;
        }

        if other.max.x > self.max.x {
            self.max.x = other.max.x;
        }

        if other.min.y < self.min.y {
            self.min.y = other.min.y;
        }

        if other.max.y > self.max.y {
            self.max.y = other.max.y;
        }

        if other.min.z < self.min.z {
            self.min.z = other.min.z;
        }

        if other.max.z > self.max.z {
            self.max.z = other.max.z;
        }
    }
}

impl Ray2D {
    pub fn new(origin: Vec2, direction: Vec2) -> Self {
        Ray2D { origin, direction }
    }

    pub fn point_at(&self, t: f32) -> Vec2 {
        self.origin + self.direction * t
    }
}

impl Ray3D {
    pub fn new(origin: Vec3, direction: Vec3) -> Self {
        Ray3D { origin, direction }
    }

    pub fn point_at(&self, t: f32) -> Vec3 {
        self.origin + self.direction * t
    }
}

pub fn lerp<T>(a: T, b: T, t: f32) -> T
where T: Mul<f32, Output = T> + Add<Output = T> {
    a * (1.0 - t) + b * t
}

pub fn lerp_2d<T>(a: T, b: T, c: T, d: T, u: f32, v: f32) -> T
where T: Mul<f32, Output = T> + Add<Output = T> {
    lerp(lerp(a, b, u), lerp(c, d, u), v)
}

pub fn lerp_3d<T>(a: T, b: T, c: T, d: T, e: T, f: T, g: T, h: T, u: f32, v: f32, w: f32) -> T
where T: Mul<f32, Output = T> + Add<Output = T> {
    lerp(lerp_2d(a, b, c, d, u, v), lerp_2d(e, f, g, h, u, v), w)
}

pub fn linear_lerp<T>(a: T, b: T, t: f32) -> T
where T: Mul<f32, Output = T> + Add<Output = T> + Sub<Output = T> + Copy {
    a + (b - a) * t
}

pub fn cosine_lerp<T>(a: T, b: T, t: f32) -> T
where T: Mul<f32, Output = T> + Add<Output = T> + Sub<Output = T> + Copy {
    let t = 0.5 - (t.fract() as f32 * core::f32::consts::PI).cos() * 0.5;
    a + (b - a) * t
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bounding_box2d() {
        let mut bbox = BoundingBox2D::new(vec2(0.0, 0.0), vec2(1.0, 1.0));
        assert_eq!(bbox.center(), vec2(0.5, 0.5));
        assert_eq!(bbox.size(), vec2(1.0, 1.0));
        assert_eq!(bbox.contains(vec2(0.5, 0.5)), true);
        assert_eq!(bbox.contains(vec2(1.5, 1.5)), false);

        bbox.expand(vec2(2.0, 2.0));
        assert_eq!(bbox.size(), vec2(2.0, 2.0));
        assert_eq!(bbox.contains(vec2(2.0, 2.0)), true);

        bbox.expand(vec2(-1.0, -1.0));
        assert_eq!(bbox.size(), vec2(3.0, 3.0));
        assert_eq!(bbox.contains(vec2(-1.0, -1.0)), true);

        let other = BoundingBox2D::new(vec2(-1.0, -1.0), vec2(1.0, 1.0));
        bbox.expand_to_fit(&other);
        assert_eq!(bbox.size(), vec2(3.0, 3.0));
        assert_eq!(bbox.contains(vec2(-1.0, -1.0)), true);
        assert_eq!(bbox.contains(vec2(1.0, 1.0)), true);
    }

    #[test]
    fn test_bounding_box3d() {
        let mut bbox = BoundingBox3D::new(vec3(0.0, 0.0, 0.0), vec3(1.0, 1.0, 1.0));
        assert_eq!(bbox.center(), vec3(0.5, 0.5, 0.5));
        assert_eq!(bbox.size(), vec3(1.0, 1.0, 1.0));
        assert_eq!(bbox.contains(vec3(0.5, 0.5, 0.5)), true);
        assert_eq!(bbox.contains(vec3(1.5, 1.5, 1.5)), false);

        bbox.expand(vec3(2.0, 2.0, 2.0));
        assert_eq!(bbox.size(), vec3(2.0, 2.0, 2.0));
        assert_eq!(bbox.contains(vec3(2.0, 2.0, 2.0)), true);

        bbox.expand(vec3(-1.0, -1.0, -1.0));
        assert_eq!(bbox.size(), vec3(3.0, 3.0, 3.0));
        assert_eq!(bbox.contains(vec3(-1.0, -1.0, -1.0)), true);

        let other = BoundingBox3D::new(vec3(-1.0, -1.0, -1.0), vec3(1.0, 1.0, 1.0));
        bbox.expand_to_fit(&other);
        assert_eq!(bbox.size(), vec3(3.0, 3.0, 3.0));
        assert_eq!(bbox.contains(vec3(-1.0, -1.0, -1.0)), true);
        assert_eq!(bbox.contains(vec3(1.0, 1.0, 1.0)), true);
    }

    #[test]
    fn test_lerp() {
        assert_eq!(lerp(0.0, 1.0, 0.5), 0.5);
        assert_eq!(lerp(vec2(0.0, 0.0), vec2(1.0, 1.0), 0.5), vec2(0.5, 0.5));
        assert_eq!(lerp(vec3(0.0, 0.0, 0.0), vec3(1.0, 1.0, 1.0), 0.5), vec3(0.5, 0.5, 0.5));
        assert_eq!(lerp(vec4(0.0, 0.0, 0.0, 0.0), vec4(1.0, 1.0, 1.0, 1.0), 0.5), vec4(0.5, 0.5, 0.5, 0.5));
    }

    #[test]
    fn test_lerp_2d() {
        assert_eq!(lerp_2d(0.0, 1.0, 2.0, 3.0, 0.5, 0.5), 1.5);
        assert_eq!(lerp_2d(vec2(0.0, 0.0), vec2(1.0, 1.0), vec2(2.0, 2.0), vec2(3.0, 3.0), 0.5, 0.5), vec2(1.5, 1.5));
    }

    #[test]
    fn test_lerp_3d() {
        assert_eq!(lerp_3d(0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 0.5, 0.5, 0.5), 3.5);
        assert_eq!(lerp_3d(vec3(0.0, 0.0, 0.0), vec3(1.0, 1.0, 1.0), vec3(2.0, 2.0, 2.0), vec3(3.0, 3.0, 3.0), vec3(4.0, 4.0, 4.0), vec3(5.0, 5.0, 5.0), vec3(6.0, 6.0, 6.0), vec3(7.0, 7.0, 7.0), 0.5, 0.5, 0.5), vec3(3.5, 3.5, 3.5));
    }

    #[test]
    fn test_linear_lerp() {
        assert_eq!(linear_lerp(0.0, 1.0, 0.5), 0.5);
        assert_eq!(linear_lerp(vec2(0.0, 0.0), vec2(1.0, 1.0), 0.5), vec2(0.5, 0.5));
        assert_eq!(linear_lerp(vec3(0.0, 0.0, 0.0), vec3(1.0, 1.0, 1.0), 0.5), vec3(0.5, 0.5, 0.5));
        assert_eq!(linear_lerp(vec4(0.0, 0.0, 0.0, 0.0), vec4(1.0, 1.0, 1.0, 1.0), 0.5), vec4(0.5, 0.5, 0.5, 0.5));
    }

    #[test]
    fn test_cosine_lerp() {
        assert_eq!(cosine_lerp(0.0, 1.0, 0.5), 0.5);
        assert_eq!(cosine_lerp(vec2(0.0, 0.0), vec2(1.0, 1.0), 0.5), vec2(0.5, 0.5));
        assert_eq!(cosine_lerp(vec3(0.0, 0.0, 0.0), vec3(1.0, 1.0, 1.0), 0.5), vec3(0.5, 0.5, 0.5));
        assert_eq!(cosine_lerp(vec4(0.0, 0.0, 0.0, 0.0), vec4(1.0, 1.0, 1.0, 1.0), 0.5), vec4(0.5, 0.5, 0.5, 0.5));

        assert_eq!(cosine_lerp(0.0, 1.0, 0.0), 0.0);
        assert_eq!(cosine_lerp(0.0, 1.0, 1.0), 0.0);

        assert_eq!(cosine_lerp(0.0, 1.0, 0.25), 0.14644662);
        assert_eq!(cosine_lerp(0.0, 1.0, 0.75), 0.8535534);
    }
}

