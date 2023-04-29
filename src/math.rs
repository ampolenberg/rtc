//! Mathematical structures for working with rays, vectors, matrices, etc.
pub mod matrix;
pub mod point;
pub mod vec3;

pub use crate::math::matrix::Axis;
pub use crate::math::matrix::Matrix;
pub use crate::math::point::Point;
pub use crate::math::vec3::Vec3;

/// A trait that allows for the comparison of vectors and points.
///
/// Tuples have a 4th dimension `w`. For points, `w = 1.0`; for vecs, `w = 0.0`.
pub trait Tuple {
    fn new(x: f64, y: f64, z: f64) -> Self;
    fn x(&self) -> f64;
    fn y(&self) -> f64;
    fn z(&self) -> f64;
    fn w(&self) -> f64;
}

/// A truly terrible macro that should never be used, so I'm using it for tests. Could just be
/// written as a function, but I'm a child and wanted to play with macros.
#[macro_export]
macro_rules! assert_vpeq {
    ($a:expr, $b:expr, $eps:expr) => {{
        let (a, b, eps) = ($a, $b, $eps);
        for i in 0..3 {
            assert!((a[i] - b[i]).abs() < eps);
        }
    }};
}

#[cfg(test)]
mod tuple_tests {
    use crate::math::{Point, Vec3};

    #[test]
    fn subtracting_two_points_gives_vec() {
        let p1 = Point(3.0, 2.0, 1.0);
        let p2 = Point(5.0, 6.0, 7.0);
        let res = p1 - p2;
        assert_eq!(res, Vec3(-2.0, -4.0, -6.0));
    }

    #[test]
    fn subtracting_vec_from_point_gives_point() {
        let p = Point(3.0, 2.0, 1.0);
        let v = Vec3(5.0, 6.0, 7.0);
        let res = p - v;
        assert_eq!(res, Point(-2.0, -4.0, -6.0));
    }

    #[test]
    fn subtracting_two_vecs_gives_vec() {
        let v1 = Vec3(3.0, 2.0, 1.0);
        let v2 = Vec3(5.0, 6.0, 7.0);
        assert_eq!(v1 - v2, Vec3(-2.0, -4.0, -6.0));
    }
}
