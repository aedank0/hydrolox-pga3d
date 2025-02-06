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
    pub fn translation(x: Float, y: Float, z: Float) -> Self {
        Self::new(0.0, 0.0, 0.0, 1.0, x * 0.5, y * 0.5, z * 0.5, 0.0)
    }
    #[inline]
    pub fn rotation_around_axis(axis_x: Float, axis_y: Float, axis_z: Float, angle: Float) -> Self {
        debug_assert!(
            (axis_x * axis_x + axis_y * axis_y + axis_z * axis_z - 1.0).abs() < 0.01,
            "Axis not normalized"
        );
        let angle = angle * 0.5;
        let sin_angle = angle.sin();
        let cos_angle = angle.cos();

        Self::new(
            axis_x * sin_angle,
            axis_y * sin_angle,
            axis_z * sin_angle,
            cos_angle,
            0.0,
            0.0,
            0.0,
            0.0,
        )
    }
    pub fn combine(self, other: Motor) -> Self {
        Self::new(
            other.vx * self.vw + other.vw * self.vx + (other.vy * self.vz - other.vz * self.vy),
            other.vy * self.vw + other.vw * self.vy + (other.vz * self.vx - other.vx * self.vz),
            other.vz * self.vw + other.vw * self.vz + (other.vx * self.vy - other.vy * self.vx),
            other.vw * self.vw - (other.vx * self.vx + other.vy * self.vy + other.vz * self.vz),
            other.mx * self.vw + other.vw * self.mx + other.vy * self.mz - other.mz * self.vy
                + other.my * self.vz
                - other.vz * self.my,
            other.my * self.vw + other.vw * self.my + other.vz * self.mx - other.mx * self.vz
                + other.mz * self.vx
                - other.vx * self.mz,
            other.mz * self.vw + other.vw * self.mz + other.vx * self.my - other.my * self.vx
                + other.mx * self.vy
                - other.vy * self.mx,
            other.vw * self.mw + self.vw * other.mw
                - (other.vx * self.mx
                    + other.mx * self.vx
                    + other.vy * self.my
                    + other.my * self.vy
                    + other.vz * self.mz
                    + other.mz * self.vz),
        )
    }
    pub fn transform(self, p: Point) -> Point {
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

    pub fn inverse(self) -> Self {
        Self::new(
            -self.vx, -self.vy, -self.vz, self.vw, -self.mx, -self.my, -self.mz, self.mw,
        )
    }
}
