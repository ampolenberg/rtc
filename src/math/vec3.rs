//! Represents a vector in 3D space.
use super::{Point, Tuple};
use std::ops;

/// Typical 3D vector.
#[derive(Debug, PartialEq, Copy, Clone, PartialOrd)]
pub struct Vec3(pub f64, pub f64, pub f64);

impl Vec3 {
    /// Computes the magnitude of a vector.
    pub fn magnitude(&self) -> f64 {
        f64::sqrt(
            self.x() * self.x() + self.y() * self.y() + self.z() * self.z() + self.w() * self.w(),
        )
    }

    /// Normalizes a vector, producing a unit vector.
    pub fn normalize(&self) -> Self {
        *self / self.magnitude()
    }

    /// Computes the dot product of two vectors.
    pub fn dot(&self, other: &Vec3) -> f64 {
        self.x() * other.x() + self.y() * other.y() + self.z() * other.z() + self.w() * other.w()
    }

    /// Computes the cross product of two vectors.
    pub fn cross(&self, other: &Vec3) -> Self {
        Vec3(
            self.y() * other.z() - self.z() * other.y(),
            self.z() * other.x() - self.x() * other.z(),
            self.x() * other.y() - self.y() * other.x(),
        )
    }

    pub fn reflect(&self, other: &Vec3) -> Self {
        *self - other * 2.0 * self.dot(other)
    }
}

impl Tuple for Vec3 {
    fn new(x: f64, y: f64, z: f64) -> Self {
        Self(x, y, z)
    }

    fn x(&self) -> f64 {
        self.0
    }

    fn y(&self) -> f64 {
        self.1
    }

    fn z(&self) -> f64 {
        self.2
    }

    fn w(&self) -> f64 {
        0.0
    }
}

impl ops::Add for Vec3 {
    type Output = Vec3;
    fn add(self, rhs: Self) -> Self::Output {
        Self(self.x() + rhs.x(), self.y() + rhs.y(), self.z() + rhs.z())
    }
}

impl ops::Add<Point> for Vec3 {
    type Output = Point;
    fn add(self, rhs: Point) -> Point {
        Point(self.x() + rhs.x(), self.y() + rhs.y(), self.z() + rhs.z())
    }
}

impl ops::Sub for Vec3 {
    type Output = Vec3;
    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.x() - rhs.x(), self.y() - rhs.y(), self.z() - rhs.z())
    }
}

impl ops::Neg for Vec3 {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self(-self.x(), -self.y(), -self.z())
    }
}

impl ops::Mul<f64> for Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: f64) -> Self::Output {
        Self(self.x() * rhs, self.y() * rhs, self.z() * rhs)
    }
}

impl ops::Mul<f64> for &Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: f64) -> Self::Output {
        Vec3(self.x() * rhs, self.y() * rhs, self.z() * rhs)
    }
}

impl ops::Mul<Vec3> for f64 {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Vec3 {
        Vec3(self * rhs.x(), self * rhs.y(), self * rhs.z())
    }
}

impl ops::Mul<&Vec3> for f64 {
    type Output = Vec3;
    fn mul(self, rhs: &Vec3) -> Vec3 {
        Vec3(self * rhs.x(), self * rhs.y(), self * rhs.z())
    }
}

impl ops::Div<f64> for Vec3 {
    type Output = Self;
    fn div(self, rhs: f64) -> Self::Output {
        Self(self.x() / rhs, self.y() / rhs, self.z() / rhs)
    }
}

/// __Should not be used.__ Implemented just for testing purposes.
impl ops::Index<usize> for Vec3 {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.0,
            1 => &self.1,
            2 => &self.2,
            _ => panic!("index {} out of bounds for array with length", index),
        }
    }
}

#[cfg(test)]
mod vec_tests {
    use super::*;

    #[test]
    fn reflecting_at_45_degs() {
        let v = Vec3(1.0, -1.0, 0.0);
        let n = Vec3(0.0, 1.0, 0.0);
        let r = v.reflect(&n);

        assert_eq!(r, Vec3(1.0, 1.0, 0.0));
    }

    #[test]
    fn reflecting_off_slanted_surface() {
        let v = Vec3(0.0, -1.0, 0.0);
        let n = Vec3(2.0_f64.sqrt() / 2.0, 2.0_f64.sqrt() / 2.0, 0.0);
        let r = v.reflect(&n);

        // assert_eq!(r, Vec3(1.0, 0.0, 0.0));
        for i in 0..3 {
            assert!((r[i] - Vec3(1.0, 0.0, 0.0)[i]).abs() < 1e-4);
        }
    }

    #[test]
    fn vecs_can_be_negated() {
        let v = Vec3(1.0, 2.0, -3.0);
        assert_eq!(-v, Vec3(-1.0, -2.0, 3.0));
    }

    #[test]
    fn vecs_can_be_scaled() {
        let v = Vec3(1.0, -2.0, 3.0);
        let s = 3.5;
        assert_eq!(v * s, Vec3(3.5, -7.0, 10.5));
    }

    #[test]
    fn vecs_can_be_divided_by_floats() {
        let v = Vec3(1.0, -2.0, 3.0);
        let f = 2.0;
        assert_eq!(v / f, Vec3(0.5, -1.0, 1.5));
    }

    #[test]
    fn vecs_have_magnitude_simple() {
        let v = Vec3(0.0, 1.0, 0.0);
        assert_eq!(v.magnitude(), 1.0);
    }

    #[test]
    fn vecs_have_magnitude() {
        let v = Vec3(1.0, 2.0, 3.0);
        assert_eq!(v.magnitude(), f64::sqrt(14.0));
    }

    #[test]
    fn vecs_have_magnitude_negative_components() {
        let v = Vec3(1.0, 2.0, 3.0);
        let v = -v;
        assert_eq!(v.magnitude(), f64::sqrt(14.0));
    }

    #[test]
    fn vecs_can_be_normalized_simple() {
        let v = Vec3(4.0, 0.0, 0.0);
        assert_eq!(v.normalize(), Vec3(1.0, 0.0, 0.0));
    }

    #[test]
    fn vecs_can_be_normalized() {
        let v = Vec3(1.0, 2.0, 3.0);
        let sq = f64::sqrt(14.0);
        assert_eq!(v.normalize(), Vec3(1.0 / sq, 2.0 / sq, 3.0 / sq));
    }

    #[test]
    fn dot_product_works() {
        let v1 = Vec3(1.0, 2.0, 3.0);
        let v2 = Vec3(2.0, 3.0, 4.0);
        assert_eq!(v1.dot(&v2), 20.0);
    }

    #[test]
    fn cross_product_works() {
        let v1 = Vec3(1.0, 2.0, 3.0);
        let v2 = Vec3(2.0, 3.0, 4.0);
        assert_eq!(v1.cross(&v2), Vec3(-1.0, 2.0, -1.0));
        assert_eq!(v2.cross(&v1), Vec3(1.0, -2.0, 1.0));
    }
}
