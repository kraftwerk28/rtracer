use super::vector::Vec3;
use std::{fmt, rc::Rc};

pub struct Face(pub Rc<Vec3>, pub Rc<Vec3>, pub Rc<Vec3>);

impl Face {
    pub fn new(a: Rc<Vec3>, b: Rc<Vec3>, c: Rc<Vec3>) -> Self {
        Self(a, b, c)
    }
}

impl fmt::Debug for Face {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}, {}, {}]", self.0, self.1, self.2)
    }
}

pub struct Straight {
    point: Vec3,
    direction: Vec3,
}

pub struct Ray {
    origin: Vec3,
    direction: Vec3,
}

pub struct Camera {
    pub pos: Vec3<f64>,
    pub dir: Vec3<f64>,
    pub fov: f64,
}
