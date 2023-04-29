use crate::{
    math::{Matrix, Point, Tuple},
    visuals::Color,
};

/// Accepts a vector of colors to construct a striped pattern.
#[derive(Debug, Clone, PartialEq)]
pub struct StripePattern {
    pub(super) colors: Vec<Color>,
    pub(super) transform: Matrix<4>,
}

impl StripePattern {
    pub(super) fn new(colors: Vec<Color>) -> Self {
        Self {
            colors,
            transform: Matrix::identity(),
        }
    }

    /// Gets the color at the given point. Generalized for arbitrarily many colors.
    pub(super) fn color_at(&self, pt: &Point) -> Color {
        let idx = pt.x().floor().abs() as usize % self.colors.len();

        self.colors[idx]
    }
}
