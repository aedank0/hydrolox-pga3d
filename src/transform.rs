use crate::point::Point;

#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Transform {
    pub vx: f64,
    pub vy: f64,
    pub vz: f64,
    pub vw: f64,
    pub mx: f64,
    pub my: f64,
    pub mz: f64,
    pub mw: f64,
}
impl Transform {
    pub const IDENTITY: Self = Self::new(0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0);
    pub const fn new(
        vx: f64,
        vy: f64,
        vz: f64,
        vw: f64,
        mx: f64,
        my: f64,
        mz: f64,
        mw: f64,
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
    pub fn combine(self, other: Transform) -> Self {
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
