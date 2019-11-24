#[cfg(test)]
pub mod tests {
    use crate::vector::Vec3;
    #[test]
    fn vec_print() {
        let v = Vec3::dummy();
        assert_eq!(v, Vec3::new(0., 0., 0.))
    }

    #[test]
    fn vec_multiplication() {
        let v = Vec3::new(10., 20., -30.2);
        assert_eq!(v * 3., Vec3::new(30., 60., -90.));
    }

    #[test]
    fn vec_prod() {
        let v1 = Vec3::new(-1., 2., -3.);
        let v2 = Vec3::new(0., -4., 1.);
        assert_eq!(Vec3::vec_prod(v1, v2), Vec3::new(-10., 1., 4.));
    }
}
