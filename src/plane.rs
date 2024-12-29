#[derive(Debug, Clone, Copy)]
pub struct Plane {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}
impl Plane {
    pub const fn new(x: f64, y: f64, z: f64, w: f64) -> Self {
        Self { x, y, z, w }
    }
}
