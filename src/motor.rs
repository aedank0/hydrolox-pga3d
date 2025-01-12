use crate::{point::Point, Float};

#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "bytemuck", derive(bytemuck::Pod, bytemuck::Zeroable))]
#[repr(C)]
pub struct Motor {
    pub vx: Float,
    pub vy: Float,
    pub vz: Float,
    pub vw: Float,
    pub mx: Float,
    pub my: Float,
    pub mz: Float,
    pub mw: Float,
}
impl Motor {
    pub const IDENTITY: Self = Self::new(0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0);
    pub const fn new(
        vx: Float,
        vy: Float,
        vz: Float,
        vw: Float,
        mx: Float,
        my: Float,
        mz: Float,
        mw: Float,
    ) -> Self {
        Self {
            vx,
            vy,
            vz,
            vw,
            mx,
            my,
            mz,
            mw,
        }
    }
    pub fn combine(self, other: Motor) -> Self {
        Self::new(
            self.vx * other.vw + self.vw * other.vx + (self.vy * other.vz - self.vz * other.vy),
            self.vy * other.vw + self.vw * other.vy + (self.vz * other.vx - self.vx * other.vz),
            self.vz * other.vw + self.vw * other.vz + (self.vx * other.vy - self.vy * other.vx),
            self.vw * other.vw - (self.vx * other.vx + self.vy * other.vy + self.vz * other.vz),
            self.mx * other.vw + self.vw * other.mx + self.vy * other.mz - self.mz * other.vy
                + self.my * other.vz
                - self.vz * other.my,
            self.my * other.vw + self.vw * other.my + self.vz * other.mx - self.mx * other.vz
                + self.mz * other.vx
                - self.vx * other.mz,
            self.mz * other.vw + self.vw * other.mz + self.vx * other.my - self.my * other.vx
                + self.mx * other.vy
                - self.vy * other.mx,
            self.vw * other.mx + other.mx * self.mw
                - (self.mx * other.vx
                    + self.vx * other.mx
                    + self.my * other.vy
                    + self.vy * other.my
                    + self.mz * other.vz
                    + self.vz * other.mz),
        )
    }
    pub fn transform(&self, p: Point) -> Point {
        let ax = self.vy * p.z - self.vz * p.y + p.w * self.mx;
        let ay = self.vz * p.x - self.vx * p.z + p.w * self.my;
        let az = self.vx * p.y - self.vy * p.x + p.w * self.mz;

        Point::new(
            p.x + 2.0 * (self.vw * ax + (self.vy * az - self.vz * ay) - self.mw * p.w * self.vx),
            p.y + 2.0 * (self.vw * ay + (self.vz * ax - self.vx * az) - self.mw * p.w * self.vy),
            p.z + 2.0 * (self.vw * az + (self.vx * ay - self.vy * ax) - self.mw * p.w * self.vz),
            p.w,
        )
    }
}
