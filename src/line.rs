use crate::{plane::Plane, point::Point, Float};

#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "bytemuck", derive(bytemuck::Pod, bytemuck::Zeroable))]
#[repr(C)]
pub struct Line {
    pub vx: Float,
    pub vy: Float,
    pub vz: Float,
    pub mx: Float,
    pub my: Float,
    pub mz: Float,
}
impl Line {
    pub const fn new(vx: Float, vy: Float, vz: Float, mx: Float, my: Float, mz: Float) -> Self {
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
