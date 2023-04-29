use crate::{
    math::{Matrix, Point, Tuple},
    visuals::Color,
};

/// A pattern of concentric rings, alternating between an arbitrary number of colors.
#[derive(Debug, Clone, PartialEq)]
pub struct Rings {
    colors: Vec<Color>,
    pub(super) transform: Matrix<4>,
}

impl Rings {
    pub(super) fn new(colors: Vec<Color>) -> Self {
        Self {
            colors,
            transform: Matrix::identity(),
        }
    }

    pub(super) fn color_at(&self, pt: &Point) -> Color {
        let idx = (pt.x() * pt.x() + pt.z() * pt.z()).sqrt().floor() as usize % self.colors.len();

        self.colors[idx]
    }
}
