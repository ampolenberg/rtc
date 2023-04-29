//! A module for working with matrices and performing matrix computations/transformations.
use std::ops;

use super::{Point, Tuple, Vec3};

/// A matrix of arbitrary dimension.
///
/// One can transpose and create identity matrices of arbitrary dimension. More specific (and
/// useful) methods are reserved for matrices of dimension up to 4.
#[derive(PartialEq, Debug, Clone, Copy)]
pub struct Matrix<const D: usize> {
    data: [[f64; D]; D],
}

pub enum Axis {
    X,
    Y,
    Z,
}

impl<const D: usize> Matrix<D> {
    /// Constructs a new DxD identity matrix.
    pub fn identity() -> Self {
        let mut m = Matrix::default();
        for i in 0..D {
            for j in 0..D {
                if i == j {
                    m[i][j] = 1.0;
                }
            }
        }

        m
    }

    /// Transposes the given matrix.
    pub fn transpose(&self) -> Self {
        let mut m = Matrix::default();
        for i in 0..D {
            for j in 0..D {
                m[i][j] = self[j][i]
            }
        }

        m
    }
}

impl Matrix<4> {
    pub fn view_transform(from: Point, to: Point, up: Vec3) -> Self {
        let forward = (to - from).normalize();
        let left = forward.cross(&up.normalize());
        let true_up = left.cross(&forward);

        let orientation = Self {
            data: [
                [left.x(), left.y(), left.z(), 0.0],
                [true_up.x(), true_up.y(), true_up.z(), 0.0],
                [-forward.x(), -forward.y(), -forward.z(), 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        };

        orientation * Matrix::translation(-from.x(), -from.y(), -from.z())
    }

    /// Returns the matrix which translates points by `x, y, z` units in the corresponding
    /// dimension. Has no affect on vectors.
    pub fn translation(x: f64, y: f64, z: f64) -> Self {
        let mut res = Self::identity();
        res[0][3] = x;
        res[1][3] = y;
        res[2][3] = z;

        res
    }

    /// Returns a scaling matrix, where each dimension is scaled by the provided `x, y, z`.
    pub fn scaling(x: f64, y: f64, z: f64) -> Self {
        let mut res = Self::identity();
        res[0][0] = x;
        res[1][1] = y;
        res[2][2] = z;

        res
    }

    /// Produces a new rotation matrix along the provided axis.
    pub fn rotation(ax: Axis, rads: f64) -> Self {
        match ax {
            Axis::X => Self {
                data: [
                    [1.0, 0.0, 0.0, 0.0],
                    [0.0, rads.cos(), -(rads.sin()), 0.0],
                    [0.0, rads.sin(), rads.cos(), 0.0],
                    [0.0, 0.0, 0.0, 1.0],
                ],
            },
            Axis::Y => Self {
                data: [
                    [rads.cos(), 0.0, rads.sin(), 0.0],
                    [0.0, 1.0, 0.0, 0.0],
                    [-(rads.sin()), 0.0, rads.cos(), 0.0],
                    [0.0, 0.0, 0.0, 1.0],
                ],
            },
            Axis::Z => Self {
                data: [
                    [rads.cos(), -(rads.sin()), 0.0, 0.0],
                    [rads.sin(), rads.cos(), 0.0, 0.0],
                    [0.0, 0.0, 1.0, 0.0],
                    [0.0, 0.0, 0.0, 1.0],
                ],
            },
        }
    }

    /// Produces a shear transformation matrix, where each argument moves in proportion to the
    /// other coordinates. For instance, if the `xy` argument is set to 1, then `x` will move in
    /// proportion to `y`.
    ///
    /// So a shear matrix `Matrix::shear(1.0, 0.0, 0.0, 0.0, 0.0, 0.0)` multiplied by a point
    /// `Point(2.0, 3.0, 4.0)` produces another point `Point(5.0, 3.0, 4.0)`.
    pub fn shear(xy: f64, xz: f64, yx: f64, yz: f64, zx: f64, zy: f64) -> Self {
        Self {
            data: [
                [1.0, xy, xz, 0.0],
                [yx, 1.0, yz, 0.0],
                [zx, zy, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }

    /// Computes the inverse of the matrix.
    pub fn inverse(&self) -> Option<Self> {
        if !self.is_invertible() {
            return None;
        }
        let mut inverse = Matrix::default();

        for row in 0..self.data.len() {
            for col in 0..self.data.len() {
                let c = self.cofactor(row, col);
                inverse[col][row] = c / self.determinant();
            }
        }

        Some(inverse)
    }

    /// Checks if the matrix is invertible by checking its determinant.
    fn is_invertible(&self) -> bool {
        self.determinant() != 0.0
    }

    /// Computes the determinant of the matrix.
    fn determinant(&self) -> f64 {
        let c1 = self[0][0] * self.cofactor(0, 0);
        let c2 = self[0][1] * self.cofactor(0, 1);
        let c3 = self[0][2] * self.cofactor(0, 2);
        let c4 = self[0][3] * self.cofactor(0, 3);

        c1 + c2 + c3 + c4
    }

    /// Computes the cofactor of a 4x4 matrix for the given row/column.
    fn cofactor(&self, row: usize, col: usize) -> f64 {
        let sgn = (row + col) % 2;
        let minor = self.minor(row, col);
        if sgn == 0 {
            minor
        } else {
            -minor
        }
    }

    /// Computes the minor of a 4x4 matrix at `(i, j)`.
    fn minor(&self, row: usize, col: usize) -> f64 {
        let sub_matrix = self.submatrix(row, col);
        sub_matrix.determinant()
    }

    /// Produces the 3x3 submatrix of a 4x4 matrix.
    fn submatrix(&self, row: usize, col: usize) -> Matrix<3> {
        let mut m = Matrix::<3>::default();

        let mut row_count = 0;
        for i in 0..self.data.len() {
            if i == row {
                continue;
            } else {
                let mut col_count = 0;
                for j in 0..self.data.len() {
                    if j == col {
                        continue;
                    } else {
                        m[row_count][col_count] = self[i][j]
                    }
                    col_count += 1;
                }
            }
            row_count += 1;
        }

        m
    }
}

impl Matrix<3> {
    /// Computes the determinant of a 3x3 matrix.
    pub fn determinant(&self) -> f64 {
        let c1 = self.cofactor(0, 0);
        let c2 = self.cofactor(0, 1);
        let c3 = self.cofactor(0, 2);

        self[0][0] * c1 + self[0][1] * c2 + self[0][2] * c3
    }

    /// Computes the cofactor of a 3x3 matrix for the given row and column.
    fn cofactor(&self, row: usize, col: usize) -> f64 {
        let sgn = (row + col) % 2;
        let minor = self.minor(row, col);
        if sgn == 0 {
            minor
        } else {
            -minor
        }
    }

    /// Computes the minor of a 3x3 matrix at the specified `(row, col)` pair.
    fn minor(&self, row: usize, col: usize) -> f64 {
        let sub_matrix = self.submatrix(row, col);
        sub_matrix.determinant()
    }

    /// Produces the 2x2 submatrix of a 3x3 matrix.
    fn submatrix(&self, row: usize, col: usize) -> Matrix<2> {
        let mut m = Matrix::<2>::default();

        let mut row_count = 0;
        for i in 0..self.data.len() {
            if i == row {
                continue;
            } else {
                let mut col_count = 0;
                for j in 0..self.data.len() {
                    if j == col {
                        continue;
                    } else {
                        m[row_count][col_count] = self[i][j]
                    }
                    col_count += 1;
                }
            }
            row_count += 1;
        }

        m
    }
}

impl Matrix<2> {
    /// Computes the determinant of a 2x2 matrix.
    fn determinant(&self) -> f64 {
        self[0][0] * self[1][1] - self[0][1] * self[1][0]
    }
}

impl<const D: usize> ops::Index<usize> for Matrix<D> {
    type Output = [f64; D];

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl<const D: usize> ops::IndexMut<usize> for Matrix<D> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}

impl ops::Mul for Matrix<4> {
    type Output = Matrix<4>;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut res = Self::default();
        for i in 0..4 {
            for j in 0..4 {
                res[i][j] = self[i][0] * rhs[0][j]
                    + self[i][1] * rhs[1][j]
                    + self[i][2] * rhs[2][j]
                    + self[i][3] * rhs[3][j];
            }
        }

        res
    }
}

impl ops::Mul<Point> for Matrix<4> {
    type Output = Point;

    fn mul(self, rhs: Point) -> Point {
        let x = self[0][0] * rhs.x()
            + self[0][1] * rhs.y()
            + self[0][2] * rhs.z()
            + self[0][3] * rhs.w();
        let y = self[1][0] * rhs.x()
            + self[1][1] * rhs.y()
            + self[1][2] * rhs.z()
            + self[1][3] * rhs.w();
        let z = self[2][0] * rhs.x()
            + self[2][1] * rhs.y()
            + self[2][2] * rhs.z()
            + self[2][3] * rhs.w();

        Point(x, y, z)
    }
}

impl ops::Mul<Vec3> for Matrix<4> {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Vec3 {
        let x = self[0][0] * rhs.x()
            + self[0][1] * rhs.y()
            + self[0][2] * rhs.z()
            + self[0][3] * rhs.w();
        let y = self[1][0] * rhs.x()
            + self[1][1] * rhs.y()
            + self[1][2] * rhs.z()
            + self[1][3] * rhs.w();
        let z = self[2][0] * rhs.x()
            + self[2][1] * rhs.y()
            + self[2][2] * rhs.z()
            + self[2][3] * rhs.w();

        Vec3(x, y, z)
    }
}

impl ops::Neg for Matrix<4> {
    type Output = Self;
    fn neg(self) -> Self::Output {
        let mut res = Self::default();
        for i in 0..4 {
            for j in 0..4 {
                res[i][j] = -self[i][j];
            }
        }

        res
    }
}

impl<const D: usize> Default for Matrix<D> {
    fn default() -> Self {
        Self {
            data: [[0.0; D]; D],
        }
    }
}

#[cfg(test)]
mod matrix_tests {
    use std::f64::consts::PI;

    use super::*;
    use crate::math::{Point, Vec3};

    const EPS: f64 = 1e-5;

    #[test]
    fn arbitrary_view_transformation() {
        let from = Point(1.0, 3.0, 2.0);
        let to = Point(4.0, -2.0, 8.0);
        let up = Vec3(1.0, 1.0, 0.0);
        let t = Matrix::view_transform(from, to, up);
        let expected = Matrix {
            data: [
                [-0.50709, 0.50709, 0.67612, -2.36643],
                [0.76772, 0.60609, 0.12122, -2.82843],
                [-0.35857, 0.59761, -0.71714, 0.00000],
                [0.00000, 0.00000, 0.00000, 1.00000],
            ],
        };

        for i in 0..4 {
            for j in 0..4 {
                assert!((t[i][j] - expected[i][j]).abs() < EPS);
            }
        }
    }

    #[test]
    fn view_transform_moves_world_not_eye() {
        let from = Point(0.0, 0.0, 8.0);
        let to = Point(0.0, 0.0, 0.0);
        let up = Vec3(0.0, 1.0, 0.0);
        let t = Matrix::view_transform(from, to, up);

        assert_eq!(t, Matrix::translation(0.0, 0.0, -8.0));
    }

    #[test]
    fn transformation_matrix_looking_in_positive_z_direction() {
        let from = Point(0.0, 0.0, 0.0);
        let to = Point(0.0, 0.0, 1.0);
        let up = Vec3(0.0, 1.0, 0.0);
        let t = Matrix::view_transform(from, to, up);

        assert_eq!(t, Matrix::scaling(-1.0, 1.0, -1.0));
    }

    #[test]
    fn transformation_matrix_for_default_orientation() {
        let from = Point(0.0, 0.0, 0.0);
        let to = Point(0.0, 0.0, -1.0);
        let up = Vec3(0.0, 1.0, 0.0);
        let t = Matrix::view_transform(from, to, up);

        assert_eq!(t, Matrix::identity());
    }

    #[test]
    fn chaining_transformations_works() {
        let p = Point(1.0, 0.0, 1.0);
        let rotation = Matrix::rotation(Axis::X, PI / 2.0);
        let scaling = Matrix::scaling(5.0, 5.0, 5.0);
        let translation = Matrix::translation(10.0, 5.0, 7.0);
        let t = translation * scaling * rotation;

        assert_eq!(t * p, Point(15.0, 0.0, 7.0));
    }

    #[test]
    fn shearing_transformations_work() {
        let t1 = Matrix::shear(1.0, 0.0, 0.0, 0.0, 0.0, 0.0);
        let t2 = Matrix::shear(0.0, 0.0, 1.0, 0.0, 1.0, 0.0);
        let p = Point(2.0, 3.0, 4.0);

        assert_eq!(t1 * p, Point(5.0, 3.0, 4.0));
        assert_eq!(t2 * p, Point(2.0, 5.0, 6.0));
    }

    #[test]
    fn z_axis_rotations() {
        let p = Point(0.0, 1.0, 0.0);
        let hq = Matrix::rotation(Axis::Z, PI / 4.0);
        let expected = Point(-(2.0_f64.sqrt()) / 2.0, 2.0_f64.sqrt() / 2.0, 0.0);

        assert!(((hq * p).x() - expected.x()).abs() < EPS);
        assert!(((hq * p).y() - expected.y()).abs() < EPS);
        assert!(((hq * p).z() - expected.z()).abs() < EPS);
    }

    #[test]
    fn y_axis_rotations() {
        let p = Point(0.0, 0.0, 1.0);
        let hq = Matrix::rotation(Axis::Y, PI / 4.0);
        let expected = Point(2.0_f64.sqrt() / 2.0, 0.0, 2.0_f64.sqrt() / 2.0);

        assert!(((hq * p).x() - expected.x()).abs() < EPS);
        assert!(((hq * p).y() - expected.y()).abs() < EPS);
        assert!(((hq * p).z() - expected.z()).abs() < EPS);
    }

    #[test]
    fn inverse_x_rotation() {
        let p = Point(0.0, 1.0, 0.0);
        let hq = Matrix::rotation(Axis::X, PI / 4.0);
        let inv = hq.inverse().unwrap();
        let expected = Point(0.0, 2.0_f64.sqrt() / 2.0, -(2.0_f64.sqrt()) / 2.0);

        assert!(((inv * p).x() - expected.x()).abs() < EPS);
        assert!(((inv * p).y() - expected.y()).abs() < EPS);
        assert!(((inv * p).z() - expected.z()).abs() < EPS);
    }

    #[test]
    fn rotating_point_around_x_axis() {
        let p = Point(0.0, 1.0, 0.0);
        let half_quarter = Matrix::rotation(Axis::X, PI / 4.0);
        let full_quarter = Matrix::rotation(Axis::X, PI / 2.0);

        assert!(
            ((half_quarter * p).x() - Point(0.0, f64::sqrt(2.0) / 2.0, f64::sqrt(2.0) / 2.0).x())
                .abs()
                < EPS
        );
        assert!(
            ((half_quarter * p).y() - Point(0.0, f64::sqrt(2.0) / 2.0, f64::sqrt(2.0) / 2.0).y())
                .abs()
                < EPS
        );
        assert!(
            ((half_quarter * p).z() - Point(0.0, f64::sqrt(2.0) / 2.0, f64::sqrt(2.0) / 2.0).z())
                .abs()
                < EPS
        );
        assert!(((full_quarter * p).x() - Point(0.0, 0.0, 1.0).x()).abs() < EPS);
        assert!(((full_quarter * p).y() - Point(0.0, 0.0, 1.0).y()).abs() < EPS);
        assert!(((full_quarter * p).z() - Point(0.0, 0.0, 1.0).z()).abs() < EPS);
    }

    #[test]
    fn reflection_is_scaling_by_a_negative() {
        let t = Matrix::scaling(-1.0, 1.0, 1.0);
        let p = Point(2.0, 3.0, 4.0);

        assert_eq!(t * p, Point(-2.0, 3.0, 4.0));
    }

    #[test]
    fn multiplying_tuple_by_inverse_scaling() {
        let t = Matrix::scaling(2.0, 3.0, 4.0);
        let inv = t.inverse().unwrap();
        let v = Vec3(-4.0, 6.0, 8.0);

        assert_eq!(inv * v, Vec3(-2.0, 2.0, 2.0));
    }

    #[test]
    fn scaling_matrix_applied_to_vector() {
        let t = Matrix::scaling(2.0, 3.0, 4.0);
        let v = Vec3(-4.0, 6.0, 8.0);

        assert_eq!(t * v, Vec3(-8.0, 18.0, 32.0));
    }

    #[test]
    fn scaling_matrix_applied_to_point() {
        let t = Matrix::scaling(2.0, 3.0, 4.0);
        let p = Point(-4.0, 6.0, 8.0);

        assert_eq!(t * p, Point(-8.0, 18.0, 32.0));
    }

    #[test]
    fn translation_does_not_affect_vectors() {
        let t = Matrix::translation(5.0, -3.0, 2.0);
        let v = Vec3(-3.0, 4.0, 5.0);

        assert_eq!(t * v, v);
    }

    #[test]
    fn multiplying_point_by_inverse_translation() {
        let t = Matrix::translation(5.0, -3.0, 2.0);
        let inv = t.inverse().unwrap();
        let p = Point(-3.0, 4.0, 5.0);

        assert_eq!(inv * p, Point(-8.0, 7.0, 3.0));
    }

    #[test]
    fn multiplying_point_by_translation() {
        let t = Matrix::translation(5.0, -3.0, 2.0);
        let p = Point(-3.0, 4.0, 5.0);

        assert_eq!(t * p, Point(2.0, 1.0, 7.0));
    }

    #[test]
    fn product_times_inverse_gives_original() {
        let a = Matrix {
            data: [
                [3.0, -9.0, 7.0, 3.0],
                [3.0, -8.0, 2.0, -9.0],
                [-4.0, 4.0, 4.0, 1.0],
                [-6.0, 5.0, -1.0, 1.0],
            ],
        };
        let b = Matrix {
            data: [
                [8.0, 2.0, 2.0, 2.0],
                [3.0, -1.0, 7.0, 0.0],
                [7.0, 0.0, 5.0, 4.0],
                [6.0, -2.0, 0.0, 5.0],
            ],
        };
        let c = a * b;
        let want = c * b.inverse().unwrap();

        for i in 0..4 {
            for j in 0..4 {
                assert!((want[i][j] - a[i][j]).abs() < EPS);
            }
        }
    }

    #[test]
    fn computes_inverse_of_4x4_matrices_3() {
        let a = Matrix {
            data: [
                [9.0, 3.0, 0.0, 9.0],
                [-5.0, -2.0, -6.0, -3.0],
                [-4.0, 9.0, 6.0, 4.0],
                [-7.0, 6.0, 6.0, 2.0],
            ],
        };
        let a_inv = Matrix {
            data: [
                [-0.04074, -0.07778, 0.14444, -0.22222],
                [-0.07778, 0.03333, 0.36667, -0.33333],
                [-0.02901, -0.14630, -0.10926, 0.12963],
                [0.17778, 0.06667, -0.26667, 0.33333],
            ],
        };

        for i in 0..4 {
            for j in 0..4 {
                assert!((a.inverse().unwrap()[i][j] - a_inv[i][j]).abs() < EPS);
            }
        }
    }

    #[test]
    fn computes_inverse_of_4x4_matrices_2() {
        let a = Matrix {
            data: [
                [8.0, -5.0, 9.0, 2.0],
                [7.0, 5.0, 6.0, 1.0],
                [-6.0, 0.0, 9.0, 6.0],
                [-3.0, 0.0, -9.0, -4.0],
            ],
        };
        let a_inv = Matrix {
            data: [
                [-0.15385, -0.15385, -0.28205, -0.53846],
                [-0.07692, 0.12308, 0.02564, 0.03077],
                [0.35897, 0.35897, 0.43590, 0.92308],
                [-0.69231, -0.69231, -0.76923, -1.92308],
            ],
        };

        for i in 0..4 {
            for j in 0..4 {
                assert!((a.inverse().unwrap()[i][j] - a_inv[i][j]).abs() < EPS);
            }
        }
    }

    #[test]
    fn computes_inverse_of_4x4_matrices() {
        let a = Matrix {
            data: [
                [-5.0, 2.0, 6.0, -8.0],
                [1.0, -5.0, 1.0, 8.0],
                [7.0, 7.0, -6.0, -7.0],
                [1.0, -3.0, 7.0, 4.0],
            ],
        };

        let a_inv = Matrix {
            data: [
                [0.21805, 0.45113, 0.24060, -0.04511],
                [-0.80827, -1.45677, -0.44361, 0.52068],
                [-0.07895, -0.22368, -0.05263, 0.19737],
                [-0.52256, -0.81391, -0.30075, 0.30639],
            ],
        };

        for i in 0..4 {
            for j in 0..4 {
                assert!((a.inverse().unwrap()[i][j] - a_inv[i][j]).abs() < EPS);
            }
        }
    }

    #[test]
    fn invertible_4x4_test() {
        let a = Matrix {
            data: [
                [6.0, 4.0, 4.0, 4.0],
                [5.0, 5.0, 7.0, 6.0],
                [4.0, -9.0, 3.0, -7.0],
                [9.0, 1.0, 7.0, -6.0],
            ],
        };

        assert!(a.is_invertible());
    }

    #[test]
    fn non_invertible_4x4_test() {
        let a = Matrix {
            data: [
                [-4.0, 2.0, -2.0, -3.0],
                [9.0, 6.0, 2.0, 6.0],
                [0.0, -5.0, 1.0, -5.0],
                [0.0, 0.0, 0.0, 0.0],
            ],
        };

        assert!(!a.is_invertible());
    }

    #[test]
    fn determinant_of_4x4() {
        let a = Matrix {
            data: [
                [-2.0, -8.0, 3.0, 5.0],
                [-3.0, 1.0, 7.0, 3.0],
                [1.0, 2.0, -9.0, 6.0],
                [-6.0, 7.0, 7.0, -9.0],
            ],
        };

        assert_eq!(a.determinant(), -4071.0);
    }

    #[test]
    fn determinant_of_3x3() {
        let a = Matrix {
            data: [[1.0, 2.0, 6.0], [-5.0, 8.0, -4.0], [2.0, 6.0, 4.0]],
        };

        assert_eq!(a.determinant(), -196.0);
    }

    #[test]
    fn cofactors_of_3x3() {
        let a = Matrix {
            data: [[3.0, 5.0, 0.0], [2.0, -1.0, -7.0], [6.0, -1.0, 5.0]],
        };

        assert_eq!(a.cofactor(0, 0), -12.0);
        assert_eq!(a.cofactor(1, 0), -25.0);
    }

    #[test]
    fn minors_of_3x3() {
        let a = Matrix {
            data: [[3.0, 5.0, 0.0], [2.0, -1.0, -7.0], [6.0, -1.0, 5.0]],
        };

        assert_eq!(a.minor(1, 0), 25.0);
    }

    #[test]
    fn submatrix_of_4x4() {
        let a = Matrix {
            data: [
                [-6.0, 1.0, 1.0, 6.0],
                [-8.0, 5.0, 8.0, 6.0],
                [-1.0, 0.0, 8.0, 2.0],
                [-7.0, 1.0, -1.0, 1.0],
            ],
        };
        let b = Matrix {
            data: [[-6.0, 1.0, 6.0], [-8.0, 8.0, 6.0], [-7.0, -1.0, 1.0]],
        };

        assert_eq!(a.submatrix(2, 1), b);
    }

    #[test]
    fn submatrix_of_3x3() {
        let a = Matrix {
            data: [[1.0, 5.0, 0.0], [-3.0, 2.0, 7.0], [0.0, 6.0, -3.0]],
        };
        let b = Matrix {
            data: [[-3.0, 2.0], [0.0, 6.0]],
        };

        assert_eq!(a.submatrix(0, 2), b);
    }

    #[test]
    fn determinant_of_2x2() {
        let a = Matrix {
            data: [[1.0, 5.0], [-3.0, 2.0]],
        };

        assert_eq!(a.determinant(), 17.0);
    }

    #[test]
    fn can_transpose() {
        let a = Matrix {
            data: [
                [0.0, 9.0, 3.0, 0.0],
                [9.0, 8.0, 0.0, 8.0],
                [1.0, 8.0, 5.0, 3.0],
                [0.0, 0.0, 5.0, 8.0],
            ],
        };
        let b = Matrix {
            data: [
                [0.0, 9.0, 1.0, 0.0],
                [9.0, 8.0, 8.0, 0.0],
                [3.0, 0.0, 5.0, 5.0],
                [0.0, 8.0, 3.0, 8.0],
            ],
        };

        assert_eq!(a.transpose(), b);
    }

    #[test]
    fn can_create_identity_matrices() {
        let id4 = Matrix::<4>::identity();
        let expect4x4 = Matrix {
            data: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        };

        let id3 = Matrix::<3>::identity();
        let expect3x3 = Matrix {
            data: [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]],
        };
        let pt3 = Point(3.0, 9.0, 4.0);

        assert_eq!(id4, expect4x4); // 4x4 matrix
        assert_eq!(id3, expect3x3); // 3x3 matrix
        assert_eq!(pt3, id4 * pt3); // 3x3 * 3x1
    }

    #[test]
    fn matrices_can_be_indexed() {
        let m = Matrix {
            data: [
                [1.0, 2.0, 3.0, 4.0],
                [5.5, 6.5, 7.5, 8.5],
                [9.0, 10.0, 11.0, 12.0],
                [13.5, 14.5, 15.5, 16.5],
            ],
        };

        assert_eq!(m[0][0], 1.0);
        assert_eq!(m[2][3], 12.0);
        assert_eq!(m[1][2], 7.5);
        assert_eq!(m[0][..], [1.0, 2.0, 3.0, 4.0]);
    }

    #[test]
    fn makes_2d_matrices() {
        let m = Matrix {
            data: [[-3.0, 5.0], [1.0, -2.0]],
        };

        assert_eq!(m[0][0], -3.0);
        assert_eq!(m[0][1], 5.0);
        assert_eq!(m[1][0], 1.0);
        assert_eq!(m[1][1], -2.0);
    }

    #[test]
    fn makes_3d_matrices() {
        let m = Matrix {
            data: [[-3.0, 5.0, 0.0], [1.0, -2.0, -7.0], [0.0, 1.0, 1.0]],
        };

        assert_eq!(m[0][0], -3.0);
        assert_eq!(m[1][1], -2.0);
        assert_eq!(m[2][2], 1.0);
    }

    #[test]
    fn can_be_equal() {
        let a = Matrix {
            data: [
                [1.0, 2.0, 3.0, 4.0],
                [5.0, 6.0, 7.0, 8.0],
                [9.0, 8.0, 7.0, 6.0],
                [5.0, 4.0, 3.0, 2.0],
            ],
        };

        let b = Matrix {
            data: [
                [1.0, 2.0, 3.0, 4.0],
                [5.0, 6.0, 7.0, 8.0],
                [9.0, 8.0, 7.0, 6.0],
                [5.0, 4.0, 3.0, 2.0],
            ],
        };

        let c = Matrix {
            data: [
                [2.0, 3.0, 4.0, 5.0],
                [5.0, 6.0, 7.0, 8.0],
                [9.0, 8.0, 7.0, 6.0],
                [5.0, 4.0, 3.0, 2.0],
            ],
        };

        assert!(a == b);
        assert!(a != c);
    }

    #[test]
    fn matrix_product_works() {
        let a = Matrix {
            data: [
                [1.0, 2.0, 3.0, 4.0],
                [5.0, 6.0, 7.0, 8.0],
                [9.0, 8.0, 7.0, 6.0],
                [5.0, 4.0, 3.0, 2.0],
            ],
        };

        let b = Matrix {
            data: [
                [-2.0, 1.0, 2.0, 3.0],
                [3.0, 2.0, 1.0, -1.0],
                [4.0, 3.0, 6.0, 5.0],
                [1.0, 2.0, 7.0, 8.0],
            ],
        };

        let prod = Matrix {
            data: [
                [20.0, 22.0, 50.0, 48.0],
                [44.0, 54.0, 114.0, 108.0],
                [40.0, 58.0, 110.0, 102.0],
                [16.0, 26.0, 46.0, 42.0],
            ],
        };

        assert_eq!(a * b, prod);
    }

    #[test]
    fn matrix_tuple_products() {
        let a = Matrix {
            data: [
                [1.0, 2.0, 3.0, 4.0],
                [2.0, 4.0, 4.0, 2.0],
                [8.0, 6.0, 4.0, 1.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        };
        let b = Point(1.0, 2.0, 3.0);
        let prod = Point(18.0, 24.0, 33.0);

        assert_eq!(a * b, prod);
    }
}
