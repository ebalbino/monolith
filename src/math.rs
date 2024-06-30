extern crate nalgebra as na;

use std::default::Default;
use na::{Vector2, Vector3, Vector4};

pub type Vec2 = Vector2<f64>;
pub type Vec3 = Vector3<f64>;
pub type Vec4 = Vector4<f64>;
pub type Vec2i = Vector2<i32>;
pub type Vec3i = Vector3<i32>;
pub type Vec4i = Vector4<i32>;
pub type Vec4b = Vector4<u8>;

pub type Point = Vec2;

const ZERO_VEC2: Vec2 = Vec2::new(0.0, 0.0);
const ZERO_VEC3: Vec3 = Vec3::new(0.0, 0.0, 0.0);
const ZERO_VEC4: Vec4 = Vec4::new(0.0, 0.0, 0.0, 0.0);

const ONE_VEC2: Vec2 = Vec2::new(1.0, 1.0);
const ONE_VEC3: Vec3 = Vec3::new(1.0, 1.0, 1.0);
const ONE_VEC4: Vec4 = Vec4::new(1.0, 1.0, 1.0, 1.0);

pub struct BoundingBox2D {
    pub min: Vec2,
    pub max: Vec2,
}

pub struct BoundingBox3D {
    pub min: Vec3,
    pub max: Vec3,
}

pub struct Ray2D {
    pub origin: Vec2,
    pub direction: Vec2,
}

pub struct Ray3D {
    pub origin: Vec3,
    pub direction: Vec3,
}

pub fn vec2(x: f64, y: f64) -> Vec2 {
    Vector2::new(x, y)
}

pub fn vec3(x: f64, y: f64, z: f64) -> Vec3 {
    Vector3::new(x, y, z)
}

pub fn vec4(x: f64, y: f64, z: f64, w: f64) -> Vec4 {
    Vector4::new(x, y, z, w)
}

impl Default for BoundingBox2D {
    fn default() -> Self {
        BoundingBox2D {
            min: ZERO_VEC2,
            max: ZERO_VEC2,
        }
    }
}

impl Default for BoundingBox3D {
    fn default() -> Self {
        BoundingBox3D {
            min: ZERO_VEC3,
            max: ZERO_VEC3,
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

