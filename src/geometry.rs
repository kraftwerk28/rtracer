use super::vector::Vec3;
use std::rc::Rc;

pub struct Face(Rc<Vec3>, Rc<Vec3>, Rc<Vec3>);

impl Face {
    pub fn new(a: Rc<Vec3>, b: Rc<Vec3>, c: Rc<Vec3>) -> Self {
        Self(a, b, c)
    }
}
