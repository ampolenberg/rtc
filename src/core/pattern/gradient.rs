use crate::{
    math::{Matrix, Point, Tuple},
    visuals::Color,
};

/// A simple gradient pattern which linearly interpolates between two colors.
#[derive(Debug, Clone, PartialEq)]
pub struct Gradient {
    color1: Color,
    color2: Color,
    pub(super) transform: Matrix<4>,
}

impl Gradient {
    pub(super) fn new(color1: Color, color2: Color) -> Self {
        Self {
            color1,
            color2,
            transform: Matrix::identity(),
        }
    }

    pub(super) fn color_at(&self, pt: &Point) -> Color {
        let c1 = self.color1;
        let c2 = self.color2;

        c1 + (c2 - c1) * pt.x()
    }
}
