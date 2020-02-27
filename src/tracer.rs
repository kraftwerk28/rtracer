use super::{
    geometry::{Face, Straight},
    vector::Vec3,
};
use std::error::Error;

pub struct Tracer;

impl Tracer {
    // fn trace() -> Result<(), Error> {}
}

/// Returns
pub fn barycentric_intersection(ray: &Straight, face: &Face) -> Option<Vec3> {
    let a = *face.0.clone();
    let b = *face.1.clone();
    let c = *face.2.clone();
    let v0v1 = b - a;
    let v0v2 = c - a;
    let N = Vec3::cross(v0v1, v0v2);
    let area = N.len() / 2f64;

    let C: Vec3;
    Some(Vec3::dummy())
}
