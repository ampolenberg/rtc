//! A representation of a point in 3D space.
use super::{Tuple, Vec3};
use std::ops;

/// Typical 3D point.
#[derive(Debug, PartialEq, Clone, Copy, Default)]
pub struct Point(pub f64, pub f64, pub f64);

impl Point {}

impl Tuple for Point {
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
        1.0
    }
}

impl ops::Add<Vec3> for Point {
    type Output = Point;
    fn add(self, rhs: Vec3) -> Point {
        Point(self.x() + rhs.x(), self.y() + rhs.y(), self.z() + rhs.z())
    }
}

impl ops::Sub for Point {
    type Output = Vec3;
    fn sub(self, rhs: Self) -> Vec3 {
        Vec3(self.x() - rhs.x(), self.y() - rhs.y(), self.z() - rhs.z())
    }
}

impl ops::Sub<&Point> for Point {
    type Output = Vec3;
    fn sub(self, rhs: &Self) -> Vec3 {
        Vec3(self.x() - rhs.x(), self.y() - rhs.y(), self.z() - rhs.z())
    }
}

impl ops::Sub<Vec3> for Point {
    type Output = Point;
    fn sub(self, rhs: Vec3) -> Self {
        Self(self.x() - rhs.x(), self.y() - rhs.y(), self.z() - rhs.z())
    }
}

impl ops::Neg for Point {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self(-self.x(), -self.y(), -self.z())
    }
}

impl ops::Mul<f64> for Point {
    type Output = Point;
    fn mul(self, rhs: f64) -> Self::Output {
        Self(self.x() * rhs, self.y() * rhs, self.z() * rhs)
    }
}

impl ops::Mul<Point> for f64 {
    type Output = Point;
    fn mul(self, rhs: Point) -> Point {
        Point(self * rhs.x(), self * rhs.y(), self * rhs.z())
    }
}

impl ops::Div<f64> for Point {
    type Output = Self;
    fn div(self, rhs: f64) -> Self::Output {
        Self(self.x() / rhs, self.y() / rhs, self.z() / rhs)
    }
}

#[cfg(test)]
mod point_tests {
    use super::*;

    #[test]
    fn points_can_be_negated() {
        let p = Point(1.0, 2.0, -3.0);
        assert_eq!(-p, Point(-1.0, -2.0, 3.0));
    }

    #[test]
    fn points_can_be_scaled() {
        let p = Point(1.0, -2.0, 3.0);
        let s = 3.5;
        assert_eq!(p * s, Point(3.5, -7.0, 10.5));
    }

    #[test]
    fn points_can_be_divided_by_floats() {
        let p = Point(1.0, -2.0, 3.0);
        let f = 2.0;
        assert_eq!(p / f, Point(0.5, -1.0, 1.5));
    }
}
