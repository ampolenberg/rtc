//! Patterns can be applied to shapes and respect applied transformations.
use crate::{
    math::{Matrix, Point},
    shape::Shape,
    visuals::Color,
};

pub mod blended;
pub mod checkers;
pub mod gradient;
pub mod rings;
pub mod stripes;

pub use self::{
    blended::Blended, checkers::Checkers, gradient::Gradient, rings::Rings, stripes::StripePattern,
};

/// An enumeration of different patterns.
///
/// # Example
///
/// ```
/// # use rtc::{shape::Sphere, visuals::Color, math::{Point, Matrix},
/// # core::pattern::{StripePattern, Pattern}};
/// let stripe_pattern = Pattern::new_stripes(vec![Color::white(), Color::black()]);
///
/// // get the color at a certain point
/// assert_eq!(stripe_pattern.color_at(&Point(0.0, 0.0, 0.0)), Color::white());
///
/// // get the color of the pattern at some object, accounting for transformations
/// let s = Sphere::default().with_transform(&Matrix::scaling(2.0, 2.0, 2.0)).as_shape();
/// assert_eq!(stripe_pattern.color_at_object(&s, &Point(1.5, 0.0, 0.0)).unwrap(), Color::white());
/// ```
#[derive(Debug, Clone, PartialEq)]
pub enum Pattern {
    /// A pattern consisting of alternating stripes
    Stripes(StripePattern),

    /// A smooth gradient between two colors
    Gradient(Gradient),

    /// Concentric rings alternating between two colors
    Rings(Rings),

    /// A checkerboard, alternating between two colors
    Checkers(Checkers),

    /// A pattern obtained by blending two other patterns together
    Blended(Blended),
}

impl Pattern {
    /// Stores any number of colors for an alternating stripe pattern.
    pub fn new_stripes(colors: Vec<Color>) -> Self {
        Self::Stripes(StripePattern::new(colors))
    }

    /// Stores two colors to linearly interpolate between when computing the color at a point.
    pub fn new_gradient(color1: Color, color2: Color) -> Self {
        Self::Gradient(Gradient::new(color1, color2))
    }

    /// Stores any number of colors to construct a concentric ring pattern.
    pub fn new_rings(colors: Vec<Color>) -> Self {
        Self::Rings(Rings::new(colors))
    }

    /// TODO: docs
    pub fn new_checkers(color1: Color, color2: Color) -> Self {
        Self::Checkers(Checkers::new(color1, color2))
    }

    /// Create a new pattern which blends the supplied patterns, taking the average color at each
    /// point.
    pub fn new_blended(pattern1: Self, pattern2: Self) -> Self {
        Self::Blended(Blended::new(pattern1, pattern2))
    }

    /// Given a `Point`, returns the color of the pattern at that point.
    pub(crate) fn color_at(&self, pt: &Point) -> Color {
        match self {
            Self::Stripes(stripe_pattern) => stripe_pattern.color_at(pt),
            Self::Gradient(gradient_pattern) => gradient_pattern.color_at(pt),
            Self::Rings(ring_pattern) => ring_pattern.color_at(pt),
            Self::Checkers(checker_pattern) => checker_pattern.color_at(pt),
            Self::Blended(blended_pattern) => blended_pattern.color_at(pt),
        }
    }

    /// Given a `Shape`, returns the color of the object at the specified world-space point by
    /// converting to pattern-space coordinates. Returns `None` if either the object or the pattern
    /// inverse transformation matrices don't exist.
    pub(crate) fn color_at_object(&self, object: &Shape, world_pt: &Point) -> Option<Color> {
        let object_pt = object.transform().inverse()? * *world_pt;
        let pattern_pt = self.transform().inverse()? * object_pt;

        Some(self.color_at(&pattern_pt))
    }

    /// Sets the transformation matrix for the pattern.
    pub fn with_transform(mut self, m: &Matrix<4>) -> Self {
        match self {
            Self::Stripes(ref mut sp) => sp.transform = *m,
            Self::Gradient(ref mut gp) => gp.transform = *m,
            Self::Rings(ref mut rp) => rp.transform = *m,
            Self::Checkers(ref mut cp) => cp.transform = *m,
            Self::Blended(ref mut bp) => bp.transform = *m,
        }

        self
    }

    fn transform(&self) -> Matrix<4> {
        match self {
            Self::Stripes(sp) => sp.transform,
            Self::Gradient(gp) => gp.transform,
            Self::Rings(rp) => rp.transform,
            Self::Checkers(cp) => cp.transform,
            Self::Blended(bp) => bp.transform,
        }
    }
}

#[cfg(test)]
mod pattern_tests {
    use crate::{math::Matrix, shape::Sphere};

    use super::*;

    #[test]
    fn checkers_alternate_in_x() {
        let pat = Pattern::new_checkers(Color::white(), Color::black());

        assert_eq!(pat.color_at(&Point(0.0, 0.0, 0.0)), Color::white());
        assert_eq!(pat.color_at(&Point(0.99, 0.0, 0.0)), Color::white());
        assert_eq!(pat.color_at(&Point(1.01, 0.0, 0.0)), Color::black());
    }

    #[test]
    fn checkers_alternate_in_y() {
        let pat = Pattern::new_checkers(Color::white(), Color::black());

        assert_eq!(pat.color_at(&Point(0.0, 0.0, 0.0)), Color::white());
        assert_eq!(pat.color_at(&Point(0.0, 0.99, 0.0)), Color::white());
        assert_eq!(pat.color_at(&Point(0.0, 1.01, 0.0)), Color::black());
    }

    #[test]
    fn checkers_alternate_in_z() {
        let pat = Pattern::new_checkers(Color::white(), Color::black());

        assert_eq!(pat.color_at(&Point(0.0, 0.0, 0.0)), Color::white());
        assert_eq!(pat.color_at(&Point(0.0, 0.0, 0.99)), Color::white());
        assert_eq!(pat.color_at(&Point(0.0, 0.0, 1.01)), Color::black());
    }

    #[test]
    fn rings_extend_in_x_and_z() {
        let pat = Pattern::new_rings(vec![Color::white(), Color::black()]);

        assert_eq!(pat.color_at(&Point(0.0, 0.0, 0.0)), Color::white());
        assert_eq!(pat.color_at(&Point(1.0, 0.0, 0.0)), Color::black());
        assert_eq!(pat.color_at(&Point(0.0, 0.0, 1.0)), Color::black());
        assert_eq!(pat.color_at(&Point(0.708, 0.0, 0.708)), Color::black());
    }

    #[test]
    fn gradient_linearly_interpolates_colors() {
        let pat = Pattern::new_gradient(Color::white(), Color::black());

        assert_eq!(pat.color_at(&Point(0.0, 0.0, 0.0)), Color::white());
        assert_eq!(
            pat.color_at(&Point(0.25, 0.0, 0.0)),
            Color(0.75, 0.75, 0.75)
        );
        assert_eq!(pat.color_at(&Point(0.5, 0.0, 0.0)), Color(0.5, 0.5, 0.5));
        assert_eq!(
            pat.color_at(&Point(0.75, 0.0, 0.0)),
            Color(0.25, 0.25, 0.25)
        );
    }

    #[test]
    fn stripes_with_object_and_pattern_transformation() {
        let object = Sphere::default().with_transform(&Matrix::scaling(2.0, 2.0, 2.0));
        let pat = Pattern::new_stripes(vec![Color::white(), Color::black()])
            .with_transform(&Matrix::translation(0.5, 0.0, 0.0));

        assert_eq!(
            pat.color_at_object(&object.as_shape(), &Point(2.5, 0.0, 0.0))
                .unwrap(),
            Color::white()
        );
    }

    #[test]
    fn stripes_with_pattern_transformation() {
        let object = Sphere::default();
        let pat = Pattern::new_stripes(vec![Color::white(), Color::black()])
            .with_transform(&Matrix::scaling(2.0, 2.0, 2.0));

        assert_eq!(
            pat.color_at_object(&object.as_shape(), &Point(1.5, 0.0, 0.0))
                .unwrap(),
            Color::white()
        );
    }

    #[test]
    fn stripes_with_object_transformation() {
        let object = Sphere::default().with_transform(&Matrix::scaling(2.0, 2.0, 2.0));
        let pat = Pattern::new_stripes(vec![Color::white(), Color::black()]);

        assert_eq!(
            pat.color_at_object(&object.as_shape(), &Point(1.5, 0.0, 0.0))
                .unwrap(),
            Color::white()
        );
    }

    #[test]
    fn stripes_alternate_in_x() {
        let pat = StripePattern {
            colors: vec![Color::white(), Color::black()],
            transform: Matrix::identity(),
        };

        assert_eq!(pat.color_at(&Point(0.0, 0.0, 0.0)), Color::white());
        assert_eq!(pat.color_at(&Point(0.9, 0.0, 0.0)), Color::white());
        assert_eq!(pat.color_at(&Point(1.0, 0.0, 0.0)), Color::black());
        assert_eq!(pat.color_at(&Point(-0.1, 0.0, 0.0)), Color::black());
        assert_eq!(pat.color_at(&Point(-1.0, 0.0, 0.0)), Color::black());
        assert_eq!(pat.color_at(&Point(-1.1, 0.0, 0.0)), Color::white());
    }

    #[test]
    fn stripes_constant_in_z() {
        let pat = StripePattern {
            colors: vec![Color::white(), Color::black()],
            transform: Matrix::identity(),
        };

        assert_eq!(pat.color_at(&Point(0.0, 0.0, 0.0)), Color::white());
        assert_eq!(pat.color_at(&Point(0.0, 0.0, 1.0)), Color::white());
        assert_eq!(pat.color_at(&Point(0.0, 0.0, 2.0)), Color::white());
    }

    #[test]
    fn stripes_constant_in_y() {
        let pat = StripePattern {
            colors: vec![Color::white(), Color::black()],
            transform: Matrix::identity(),
        };

        assert_eq!(pat.color_at(&Point(0.0, 0.0, 0.0)), Color::white());
        assert_eq!(pat.color_at(&Point(0.0, 1.0, 0.0)), Color::white());
        assert_eq!(pat.color_at(&Point(0.0, 2.0, 0.0)), Color::white());
    }

    #[test]
    fn stripe_patterns_hold_colors() {
        let pat = StripePattern {
            colors: vec![Color::white(), Color::black()],
            transform: Matrix::identity(),
        };

        assert_eq!(pat.colors[0], Color::white());
        assert_eq!(pat.colors[1], Color::black());
    }
}
