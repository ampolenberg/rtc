//! Rays of light sent from the viewpoint into the scene.
//!
//! This light is then traced through its reflections and refractions until it is absorbed. The
//! resulting data is then rendered onto the canvas.
use crate::math::{Matrix, Point, Vec3};

/// Rays are created with a starting point (the origin) and a direction vector. They are then cast
/// from the camera into the scene and their collisions are tracked.
#[derive(Clone, Copy)]
pub struct Ray {
    pub origin: Point,
    pub direction: Vec3,
}

impl Ray {
    /// Constructs a new ray with the given origin (a [`Point`]) and direction (a [`Vec3`]).
    pub fn new(origin: Point, direction: Vec3) -> Self {
        Self { origin, direction }
    }

    /// Given a time `t`, determines the position of the ray.
    pub fn position(&self, t: f64) -> Point {
        self.origin + self.direction * t
    }

    /// Applies the transformation matrix m to the ray, which allows us to manipulate simple rays
    /// instead of complicated shapes/objects.
    pub(crate) fn transform(&self, m: Matrix<4>) -> Self {
        Self::new(m * self.origin, m * self.direction)
    }
}

#[cfg(test)]
mod ray_tests {
    use super::*;

    #[test]
    fn scaling_a_ray() {
        let r = Ray::new(Point(1.0, 2.0, 3.0), Vec3(0.0, 1.0, 0.0));
        let m = Matrix::scaling(2.0, 3.0, 4.0);
        let r2 = r.transform(m);

        assert_eq!(r2.origin, Point(2.0, 6.0, 12.0));
        assert_eq!(r2.direction, Vec3(0.0, 3.0, 0.0));
    }

    #[test]
    fn translating_a_ray() {
        let r = Ray::new(Point(1.0, 2.0, 3.0), Vec3(0.0, 1.0, 0.0));
        let m = Matrix::translation(3.0, 4.0, 5.0);
        let r2 = r.transform(m);

        assert_eq!(r2.origin, Point(4.0, 6.0, 8.0));
        assert_eq!(r2.direction, Vec3(0.0, 1.0, 0.0));
    }

    #[test]
    fn tracking_ray_position_over_time() {
        let r = Ray::new(Point(2.0, 3.0, 4.0), Vec3(1.0, 0.0, 0.0));

        assert_eq!(r.position(0.0), Point(2.0, 3.0, 4.0));
        assert_eq!(r.position(1.0), Point(3.0, 3.0, 4.0));
        assert_eq!(r.position(-1.0), Point(1.0, 3.0, 4.0));
        assert_eq!(r.position(2.5), Point(4.5, 3.0, 4.0));
    }

    #[test]
    fn test_making_rays() {
        let o = Point(1.0, 2.0, 3.0);
        let d = Vec3(4.0, 5.0, 6.0);
        let r = Ray::new(o, d);

        assert_eq!(r.origin, o);
        assert_eq!(r.direction, d);
    }
}
