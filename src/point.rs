use std::ops::{Add, Sub};

use crate::{line::Line, plane::Plane, Float};

#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "bytemuck", derive(bytemuck::Pod, bytemuck::Zeroable))]
#[repr(C)]
pub struct Point {
    pub x: Float,
    pub y: Float,
    pub z: Float,
    pub w: Float,
}
impl Default for Point {
    fn default() -> Self {
        Self::new(0.0, 0.0, 0.0, 1.0)
    }
}
impl Point {
    pub const ZERO: Point = Point {
        x: 0.0,
        y: 0.0,
        z: 0.0,
        w: 1.0,
    };
    pub const UP: Point = Point {
        x: 0.0,
        y: 1.0,
        z: 0.0,
        w: 1.0,
    };
    pub const DOWN: Point = Point {
        x: 0.0,
        y: -1.0,
        z: 0.0,
        w: 1.0,
    };
    pub const LEFT: Point = Point {
        x: -1.0,
        y: 0.0,
        z: 0.0,
        w: 1.0,
    };
    pub const RIGHT: Point = Point {
        x: 1.0,
        y: 0.0,
        z: 0.0,
        w: 1.0,
    };
    pub const FORWARD: Point = Point {
        x: 0.0,
        y: 0.0,
        z: -1.0,
        w: 1.0,
    };
    pub const BACK: Point = Point {
        x: 0.0,
        y: 0.0,
        z: 1.0,
        w: 1.0,
    };

    pub const fn new(x: Float, y: Float, z: Float, w: Float) -> Self {
        Self { x, y, z, w }
    }
    pub const fn new_position(x: Float, y: Float, z: Float) -> Self {
        Self { x, y, z, w: 1.0 }
    }
    pub const fn new_direction(x: Float, y: Float, z: Float) -> Self {
        Self { x, y, z, w: 0.0 }
    }
    pub const fn from_val(v: Float, w: Float) -> Self {
        Self {
            x: v,
            y: v,
            z: v,
            w,
        }
    }

    pub fn into_buffer(&self) -> [Float; 4] {
        [self.x, self.y, self.z, self.w]
    }
    pub fn into_buffer_32(&self) -> [f32; 4] {
        [self.x as f32, self.y as f32, self.z as f32, self.w as f32]
    }

    pub fn copy_to_buffer_32(&self, buf: &mut [f32]) {
        buf[0] = self.x as f32;
        buf[1] = self.y as f32;
        buf[2] = self.z as f32;
        buf[3] = self.w as f32;
    }

    pub fn is_finite(&self) -> bool {
        self.w != 0.0
    }

    /*
    pub fn join_simd(self, rhs: Point) -> Line {
        unsafe {
            let mut r = MaybeUninit::<Line>::uninit();

            let ca = _mm256_set_pd(self.z, self.z, self.y, self.x);
            let cb = _mm256_set_pd(rhs.y, rhs.w, rhs.w, rhs.w);
            let c = _mm256_mul_pd(ca, cb);
            let a = _mm256_set_pd(self.y, self.w, self.w, self.w);
            let b = _mm256_set_pd(rhs.z, rhs.z, rhs.y, rhs.x);

            _mm256_storeu_pd(addr_of_mut!((*r.as_mut_ptr()).vx), _mm256_fmsub_pd(a, b, c));

            let ca = _mm_set_pd(self.y, self.z);
            let cb = _mm_set_pd(rhs.x, rhs.z);
            let c = _mm_mul_pd(ca, cb);
            let a = _mm_set_pd(self.x, self.z);
            let b = _mm_set_pd(rhs.y, rhs.x);

            _mm_storeu_pd(addr_of_mut!((*r.as_mut_ptr()).my), _mm_fmsub_pd(a, b, c));

            r.assume_init()
        }
    }
    */

    pub fn join(self, rhs: Point) -> Line {
        Line::new(
            self.w * rhs.x - self.x * rhs.w,
            self.w * rhs.y - self.y * rhs.w,
            self.w * rhs.z - self.z * rhs.w,
            self.y * rhs.z - self.z * rhs.y,
            self.z * rhs.x - self.x * rhs.z,
            self.x * rhs.y - self.y * rhs.x,
        )
    }

    pub fn round(self) -> Self {
        Self::new(self.x.round(), self.y.round(), self.z.round(), self.w)
    }

    pub fn dot(self, rhs: Point) -> Float {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z + self.w * rhs.w
    }
    pub fn antidot(self, rhs: Point) -> Float {
        self.w * rhs.w
    }

    pub fn magnitude_squared(self) -> Float {
        self.dot(self) / self.antidot(self)
    }
    pub fn magnitude(self) -> Float {
        self.magnitude_squared().sqrt()
    }

    pub fn scaled(&self) -> Self {
        Self {
            x: self.x / self.w,
            y: self.y / self.w,
            z: self.z / self.w,
            w: self.w / self.w,
        }
    }

    pub fn dist(self, other: Point) -> Float {
        (other - self).magnitude()
    }

    pub fn expand_plane(self, rhs: Plane) -> Line {
        Line::new(
            -self.w * rhs.x,
            -self.w * rhs.y,
            -self.w * rhs.z,
            self.z * rhs.y - self.y * rhs.z,
            self.x * rhs.z - self.z * rhs.x,
            self.y * rhs.x - self.x * rhs.y,
        )
    }
    pub fn expand_line(self, rhs: Line) -> Plane {
        Plane::new(
            -self.w * rhs.vx,
            -self.w * rhs.vy,
            -self.w * rhs.vz,
            self.x * rhs.vx + self.y * rhs.vy + self.z * rhs.vz,
        )
    }
}
impl Add<Point> for Point {
    type Output = Self;
    fn add(self, rhs: Point) -> Self::Output {
        Self {
            x: rhs.x * self.w + self.x * rhs.w,
            y: rhs.y * self.w + self.y * rhs.w,
            z: rhs.z * self.w + self.z * rhs.w,
            w: rhs.w * self.w,
        }
    }
}
impl Sub<Point> for Point {
    type Output = Self;
    fn sub(self, rhs: Point) -> Self::Output {
        Self {
            x: rhs.x * self.w - self.x * rhs.w,
            y: rhs.y * self.w - self.y * rhs.w,
            z: rhs.z * self.w - self.z * rhs.w,
            w: rhs.w * self.w,
        }
    }
}
