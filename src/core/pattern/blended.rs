use crate::{
    math::{Matrix, Point},
    visuals::Color,
};

use super::Pattern;

/// TODO: docs
#[derive(Debug, PartialEq, Clone)]
pub struct Blended {
    pattern1: Box<Pattern>,
    pattern2: Box<Pattern>,
    pub(super) transform: Matrix<4>,
}

impl Blended {
    pub(super) fn new(pattern1: Pattern, pattern2: Pattern) -> Self {
        Self {
            pattern1: Box::new(pattern1),
            pattern2: Box::new(pattern2),
            transform: Matrix::identity(),
        }
    }

    pub(super) fn color_at(&self, pt: &Point) -> Color {
        let p1 = self.pattern1.transform().inverse().unwrap() * *pt;
        let p2 = self.pattern2.transform().inverse().unwrap() * *pt;

        let c1 = self.pattern1.color_at(&p1);
        let c2 = self.pattern2.color_at(&p2);

        (c1 + c2) / 2.0
    }
}
