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
    #[inline]
    pub fn from_translation(x: Float, y: Float, z: Float) -> Self {
        Self::new(0.0, 0.0, 0.0, 1.0, x * 0.5, y * 0.5, z * 0.5, 0.0)
    }
    //Order is z, x, then y
    #[inline]
    pub fn from_euler_angles(x: Float, y: Float, z: Float) -> Self {
        let z = z * 0.5;
        let sin_z = z.sin();
        let cos_z = z.cos();

        let vz1 = sin_z;
        let vw1 = cos_z;

        let x = x * 0.5;
        let sin_x = x.sin();
        let cos_x = x.cos();

        let vx2 = sin_x * vw1;
        let vy2 = -sin_x * vz1;
        let vz2 = cos_x * vz1;
        let vw2 = cos_x * vw1;

        let y = y * 0.5;
        let sin_y = y.sin();
        let cos_y = y.cos();

        let vx3 = cos_y * vx2 + sin_y * vz2;
        let vy3 = sin_y * vw2 + cos_y * vy2;
        let vz3 = cos_y * vz2 - sin_y * vx2;
        let vw3 = cos_y * vw2 - sin_y * vy2;

        Self { vx: vx3, vy: vy3, vz: vz3, vw: vw3, mx: 0.0, my: 0.0, mz: 0.0, mw: 0.0 }
    }
    #[inline]
    pub fn from_euler_pos_and_rot(pos_x: Float, pos_y: Float, pos_z: Float, rot_x: Float, rot_y: Float, rot_z: Float) -> Self {
        let rot_z = rot_z * 0.5;
        let sin_z = rot_z.sin();
        let cos_z = rot_z.cos();

        let vz1 = sin_z;
        let vw1 = cos_z;

        let rot_x = rot_x * 0.5;
        let sin_x = rot_x.sin();
        let cos_x = rot_x.cos();

        let vx2 = sin_x * vw1;
        let vy2 = -sin_x * vz1;
        let vz2 = cos_x * vz1;
        let vw2 = cos_x * vw1;

        let rot_y = rot_y * 0.5;
        let sin_y = rot_y.sin();
        let cos_y = rot_y.cos();

        let vx3 = cos_y * vx2 + sin_y * vz2;
        let vy3 = sin_y * vw2 + cos_y * vy2;
        let vz3 = cos_y * vz2 - sin_y * vx2;
        let vw3 = cos_y * vw2 - sin_y * vy2;

        let mx = pos_x * 0.5;
        let my = pos_y * 0.5;
        let mz = pos_z * 0.5;

        Self { vx: vx3, vy: vy3, vz: vz3, vw: vw3,
            mx: mx * vw3 - mz * vy3 + my * vz3,
            my: my * vw3 - mx * vz3 + mz * vx3,
            mz: mz * vw3 - my * vx3 + mx * vy3,
            mw: -(mx * vx3 + my * vy3 + mz * vz3)
        }
    }
    #[inline]
    pub fn from_rotation_around_axis(axis_x: Float, axis_y: Float, axis_z: Float, angle: Float) -> Self {
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
    #[inline]
    pub fn combine(&self, other: Motor) -> Self {
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
    #[inline]
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

    #[inline]
    pub fn inverse(&self) -> Self {
        Self::new(
            -self.vx, -self.vy, -self.vz, self.vw, -self.mx, -self.my, -self.mz, self.mw,
        )
    }

    #[inline]
    pub fn translation_euler(&self) -> Point {
        Point::new(
            2.0 * (self.vw * self.mx + self.vy * self.mz - self.vz * self.my - self.mw * self.vx),
            2.0 * (self.vw * self.my + self.vz * self.mx - self.vx * self.mz - self.mw * self.vy),
            2.0 * (self.vw * self.mz + self.vx * self.my - self.vy * self.mx - self.mw * self.vz),
            1.0,
        )
    }

    //Returns (translation, rotation)
    #[inline]
    pub fn factorize(&self) -> (Motor, Motor) {
        let translation = self.factor_translation();
        let rotation = self.combine(translation.inverse());
        (translation, rotation)
    }
    #[inline]
    pub fn factor_translation(&self) -> Motor {
        Motor::from_translation(
            self.vw * self.mx + self.vy * self.mz - self.vz * self.my - self.mw * self.vx,
            self.vw * self.my + self.vz * self.mx - self.vx * self.mz - self.mw * self.vy,
            self.vw * self.mz + self.vx * self.my - self.vy * self.mx - self.mw * self.vz,
        )
    }
    #[inline]
    pub fn factor_rotation(&self) -> Motor {
        self.combine(self.factor_translation().inverse())
    }
}
