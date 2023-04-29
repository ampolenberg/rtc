use crate::{
    math::{Matrix, Point, Tuple},
    visuals::Color,
};

/// TODO: docs
#[derive(Debug, Clone, PartialEq)]
pub struct Checkers {
    color1: Color,
    color2: Color,
    pub(super) transform: Matrix<4>,
}

impl Checkers {
    pub(super) fn new(color1: Color, color2: Color) -> Self {
        Self {
            color1,
            color2,
            transform: Matrix::identity(),
        }
    }

    pub(super) fn color_at(&self, pt: &Point) -> Color {
        let picker =
            (pt.x().floor().abs() + pt.y().floor().abs() + pt.z().floor().abs()) as usize % 2;

        if picker == 0 {
            self.color1
        } else {
            self.color2
        }
    }
}
