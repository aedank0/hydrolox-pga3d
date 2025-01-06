use crate::{plane::Plane, point::Point};

#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "serde", serde::Serialize, serde::Deserialize)]
#[repr(C)]
pub struct Line {
    pub vx: f64,
    pub vy: f64,
    pub vz: f64,
    pub mx: f64,
    pub my: f64,
    pub mz: f64,
}
impl Line {
    pub const fn new(vx: f64, vy: f64, vz: f64, mx: f64, my: f64, mz: f64) -> Self {
        Self {
            vx,
            vy,
            vz,
            mx,
            my,
            mz,
        }
    }

    pub fn join(self, rhs: Point) -> Plane {
        Plane::new(
            self.vy * rhs.z - self.vz * rhs.y + self.mx * rhs.w,
            self.vz * rhs.x - self.vx * rhs.z + self.my * rhs.w,
            self.vx * rhs.y - self.vy * rhs.x + self.mz * rhs.w,
            -(self.mx * rhs.x + self.my * rhs.y + self.mz * rhs.z),
        )
    }

    pub fn expand(self, rhs: Plane) -> Plane {
        Plane::new(
            self.vy * rhs.z - self.vz * rhs.y,
            self.vz * rhs.x - self.vx * rhs.z,
            self.vx * rhs.y - self.vy * rhs.x,
            -(self.mx * rhs.x + self.my * rhs.y + self.mz * rhs.z),
        )
    }
}
