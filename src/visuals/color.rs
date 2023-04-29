use std::ops;

/// Struct for storing color information.
#[derive(Debug, PartialEq, Copy, Clone, PartialOrd)]
pub struct Color(pub f64, pub f64, pub f64);

impl Color {
    pub fn new(r: f64, g: f64, b: f64) -> Self {
        Self(r, g, b)
    }

    /// Red channel.
    pub fn r(&self) -> f64 {
        self.0
    }

    /// Green channel.
    pub fn g(&self) -> f64 {
        self.1
    }

    /// Blue channel.
    pub fn b(&self) -> f64 {
        self.2
    }

    pub fn black() -> Color {
        Self(0.0, 0.0, 0.0)
    }

    pub fn white() -> Color {
        Self(1.0, 1.0, 1.0)
    }

    pub fn red() -> Color {
        Self(1.0, 0.0, 0.0)
    }

    pub fn green() -> Color {
        Self(0.0, 1.0, 0.0)
    }

    pub fn blue() -> Color {
        Self(0.0, 0.0, 1.0)
    }
}

impl ops::Add for Color {
    type Output = Color;
    fn add(self, rhs: Self) -> Self::Output {
        Self(self.r() + rhs.r(), self.g() + rhs.g(), self.b() + rhs.b())
    }
}

impl ops::Sub for Color {
    type Output = Color;
    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.r() - rhs.r(), self.g() - rhs.g(), self.b() - rhs.b())
    }
}

impl ops::Mul<f64> for Color {
    type Output = Color;
    fn mul(self, rhs: f64) -> Self::Output {
        Self(self.r() * rhs, self.g() * rhs, self.b() * rhs)
    }
}

impl ops::Mul<Color> for f64 {
    type Output = Color;
    fn mul(self, rhs: Color) -> Color {
        Color(self * rhs.r(), self * rhs.g(), self * rhs.b())
    }
}

impl ops::Mul for Color {
    type Output = Color;
    fn mul(self, rhs: Self) -> Self::Output {
        Self(self.r() * rhs.r(), self.g() * rhs.g(), self.b() * rhs.b())
    }
}

impl ops::Div<f64> for Color {
    type Output = Color;
    fn div(self, rhs: f64) -> Self::Output {
        Self(self.r() / rhs, self.g() / rhs, self.b() / rhs)
    }
}

impl std::iter::Sum for Color {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        let mut c = Color::black();
        for i in iter {
            c = c + i;
        }

        c
    }
}

#[cfg(test)]
mod color_tests {
    use super::*;

    #[test]
    fn colors_have_channels() {
        let c = Color(-0.5, 0.4, 1.7);
        assert_eq!(c.r(), -0.5);
        assert_eq!(c.g(), 0.4);
        assert_eq!(c.b(), 1.7);
    }

    #[test]
    fn colors_can_be_added() {
        let c1 = Color(0.9, 0.6, 0.75);
        let c2 = Color(0.7, 0.1, 0.25);
        assert_eq!(c1 + c2, Color(1.6, 0.7, 1.0));
    }

    #[test]
    fn colors_can_be_subtracted() {
        let c1 = Color(0.9, 0.6, 0.75);
        let c2 = Color(0.7, 0.1, 0.25);
        let expected = c1 - c2;
        assert!(expected - Color(0.2, 0.5, 0.5) < Color(1e-6, 1e-6, 1e-6));
    }

    #[test]
    fn can_mult_colors_and_scalars() {
        let c = Color(0.2, 0.3, 0.4);
        let f = 2.0;
        assert_eq!(c * f, Color(0.4, 0.6, 0.8));
        assert_eq!(f * c, Color(0.4, 0.6, 0.8));
    }

    #[test]
    fn color_hadamard_product() {
        let c1 = Color(1.0, 0.2, 0.4);
        let c2 = Color(0.9, 1.0, 0.1);
        assert!((c1 * c2 - Color(0.9, 0.2, 0.04)) < Color(1e-6, 1e-6, 1e-6));
    }
}
