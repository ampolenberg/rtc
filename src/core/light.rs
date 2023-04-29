use crate::{math::Point, visuals::Color};

#[derive(Debug, PartialEq)]
pub enum LightType {
    PointLight(PointLight),
}

#[derive(Debug, PartialEq)]
pub struct Light {
    pub light_type: LightType,
}

impl Light {
    pub fn new_point_light(position: Point, intensity: Color) -> Self {
        Light {
            light_type: LightType::PointLight(PointLight::new(position, intensity)),
        }
    }

    pub fn position(&self) -> Point {
        match &self.light_type {
            LightType::PointLight(pl) => pl.position,
        }
    }

    pub fn intensity(&self) -> Color {
        match &self.light_type {
            LightType::PointLight(pl) => pl.intensity,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct PointLight {
    position: Point,
    intensity: Color,
}

impl PointLight {
    /// Creates a new PointLight with specified position and intensity.
    pub fn new(position: Point, intensity: Color) -> Self {
        Self {
            position,
            intensity,
        }
    }
}

#[cfg(test)]
mod light_tests {
    use super::*;
    use crate::{math::Point, visuals::Color};

    #[test]
    fn point_light_has_position_and_intensity() {
        let intensity = Color::white();
        let pos = Point::default();
        let light = PointLight::new(pos, intensity);

        assert_eq!(light.position, pos);
        assert_eq!(light.intensity, intensity);
    }
}
