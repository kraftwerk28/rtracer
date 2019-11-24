use super::vector::Vec3;

pub struct Camera {
    pub pos: Vec3<f64>,
    pub dir: Vec3<f64>,
    pub fov: f64,
}
