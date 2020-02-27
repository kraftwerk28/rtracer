use super::vector::Vec3;

#[cfg(test)]
mod tests {
    use crate::vector::Vec3;
    #[test]
    fn vec3_neg() {
        Vec3::new_int(1, 2, 3);
        // let v = Vec3::new(9, -4, 3);
        // assert_eq!(-v, Vec3::new(-9., 4., -3.));
    }

    #[test]
    fn vec3_cross() {
        let a = Vec3::new(7., 0., 0.);
        let b = Vec3::new(3., 4., 0.);
        let cr = Vec3::cross(a, b);
        let cr2 = Vec3::cross(b, a);
        assert_eq!(cr.len(), 28.);
        assert_eq!(cr2.len(), 28.);
        assert_eq!(cr, Vec3::new(0., 0., 28.));
        assert_eq!(cr2, Vec3::new(0., 0., -28.));
    }

    #[test]
    fn vec3_dot() {
        let a = Vec3::new(4., 3., -8.);
        let b = Vec3::new(-2., 9., 0.);
        assert_eq!(a.dot(b), 19.);
    }

    #[test]
    fn vec3_len() {
        let a = 3.0f64; let b = -8.0f64; let c = 3.0f64;
        let v = Vec3::new(a, b, c);
        assert_eq!(v.len(), (a.powi(2) + b.powi(2) + c.powi(2)).sqrt())
    }
}
