use crate::{
    math::{Point, Vec3},
    shape::Shape,
    visuals::Color,
};

use super::{light::Light, pattern::Pattern};

/// Phong materials. Each attribute should be nonnegative. For `ambient`, `diffuse`, and
/// `specular`, values between 0.0 and 1.0 are typical. For `shininess`, a value of 10.0 is
/// considered very large and 200.0 very small (there is no hard upper-bound).
#[derive(Debug, PartialEq, Clone)]
pub struct Material {
    pub(crate) color: Color,
    pub(crate) pattern: Option<Pattern>,
    pub(crate) ambient: f64,
    pub(crate) diffuse: f64,
    pub(crate) specular: f64,
    pub(crate) shininess: f64,
    pub(crate) reflective: f64,
    pub(crate) transparency: f64,
    pub(crate) refractive_index: f64,
}

impl Material {
    /// Computes the lighting associated with the material.
    pub fn lighting(
        &self,
        object: &Shape,
        light: &Light,
        point: &Point,
        eyev: &Vec3,
        normalv: &Vec3,
        in_shadow: bool,
    ) -> Color {
        let mut color = self.color;
        if let Some(pat) = self.pattern.clone() {
            color = pat.color_at_object(object, point).unwrap();
        }

        // combines surface color with the light's color/intensity
        let effective_color = color * light.intensity();

        // direction to light source
        let lightv = (light.position() - point).normalize();

        // ambient contribution
        let ambient = effective_color * self.ambient;

        // The (cosine of the) angle between the light vector and the surface normal
        // light_dot_normal < 0.0 implies the light is on the other side of the surface
        let light_dot_normal = lightv.dot(normalv);

        // If we are in a shadowed region, specular and diffuse are ignored and only ambient
        // contributes to the color.
        if in_shadow {
            return ambient;
        }

        // compute the specular and diffuse contributions
        let (specular, diffuse) = if light_dot_normal < 0.0 {
            (Color::black(), Color::black())
        } else {
            let reflectv = -lightv.reflect(normalv);

            // (cosine of the) angle between the reflection vector and the eye vector
            // reflect_dot_eye <= 0.0 means the light reflects away from the eye
            let reflect_dot_eye = reflectv.dot(eyev);
            if reflect_dot_eye <= 0.0 {
                (
                    Color::black(),
                    effective_color * self.diffuse * light_dot_normal,
                )
            } else {
                let factor = reflect_dot_eye.powi(self.shininess as i32); // specular contribution component
                (
                    light.intensity() * self.specular * factor,
                    effective_color * self.diffuse * light_dot_normal,
                )
            }
        };

        ambient + diffuse + specular
    }

    pub fn with_pattern(mut self, pattern: &Pattern) -> Self {
        self.pattern = Some((*pattern).clone());
        self
    }

    pub fn with_color(mut self, color: &Color) -> Self {
        self.color = *color;
        self
    }

    pub fn with_ambient(mut self, ambient: f64) -> Self {
        self.ambient = ambient;
        self
    }

    pub fn with_diffuse(mut self, diffuse: f64) -> Self {
        self.diffuse = diffuse;
        self
    }

    pub fn with_specular(mut self, specular: f64) -> Self {
        self.specular = specular;
        self
    }

    pub fn with_shininess(mut self, shininess: f64) -> Self {
        self.shininess = shininess;
        self
    }

    pub fn with_reflective(mut self, reflective: f64) -> Self {
        self.reflective = reflective;
        self
    }

    pub fn with_transparency(mut self, transparency: f64) -> Self {
        self.transparency = transparency;
        self
    }

    pub fn with_refractive_index(mut self, refractive_index: f64) -> Self {
        self.refractive_index = refractive_index;
        self
    }
}

impl Default for Material {
    fn default() -> Self {
        Self {
            color: Color::white(),
            pattern: None,
            ambient: 0.1,
            diffuse: 0.9,
            specular: 0.9,
            shininess: 200.0,
            reflective: 0.0,
            transparency: 0.0,
            refractive_index: 1.0,
        }
    }
}

#[cfg(test)]
mod material_tests {
    use crate::{
        core::{precompute::PrecomputedData, Intersection, IntersectionList, Ray},
        math::Matrix,
        shape::Sphere,
    };

    use super::*;

    const ROOT2: f64 = std::f64::consts::FRAC_1_SQRT_2;

    // A helper function for a glassy sphere. Used in some tests below.
    fn glass_sphere() -> Sphere {
        Sphere::default().with_material(
            &Material::default()
                .with_refractive_index(1.5)
                .with_transparency(1.0),
        )
    }

    #[test]
    fn finding_n1_and_n2_at_various_intersections() {
        let a = glass_sphere()
            .with_transform(&Matrix::scaling(2.0, 2.0, 2.0))
            .as_shape();
        let b = glass_sphere()
            .with_transform(&Matrix::translation(0.0, 0.0, -0.25))
            .with_material(&Material::default().with_refractive_index(2.0))
            .as_shape();
        let c = glass_sphere()
            .with_transform(&Matrix::translation(0.0, 0.0, 0.25))
            .with_material(&Material::default().with_refractive_index(2.5))
            .as_shape();

        let r = Ray::new(Point(0.0, 0.0, -4.0), Vec3(0.0, 0.0, 1.0));
        let i1 = Intersection::new(2.0, a.clone());
        let i2 = Intersection::new(2.75, b.clone());
        let i3 = Intersection::new(3.25, c.clone());
        let i4 = Intersection::new(4.75, b);
        let i5 = Intersection::new(5.25, c);
        let i6 = Intersection::new(6.00, a);
        let ix = vec![i1, i2, i3, i4, i5, i6];

        let xs = IntersectionList::new(ix);

        let expected_n1 = vec![1.0, 1.5, 2.0, 2.5, 2.5, 1.5];
        let expected_n2 = vec![1.5, 2.0, 2.5, 2.5, 1.5, 1.0];

        for idx in 0..6 {
            let comps = PrecomputedData::new(&xs[idx], &r, &xs);
            assert_eq!(comps.n1, expected_n1[idx]);
            assert_eq!(comps.n2, expected_n2[idx]);
        }
    }

    #[test]
    fn lighting_with_surface_in_shadow() {
        let object = Sphere::default().as_shape();
        let m = Material::default();
        let eyev = Vec3(0.0, 0.0, -1.0);
        let normalv = Vec3(0.0, 0.0, -1.0);
        let light = Light::new_point_light(Point(0.0, 0.0, -10.0), Color::white());
        let in_shadow = true;

        let result = m.lighting(
            &object,
            &light,
            &Point(0.0, 0.0, 0.0),
            &eyev,
            &normalv,
            in_shadow,
        );
        assert_eq!(result, Color(0.1, 0.1, 0.1));
    }

    #[test]
    fn lighting_with_light_behind_surface() {
        let object = Sphere::default().as_shape();
        let m = Material::default();
        let pos = Point(0.0, 0.0, 0.0);
        let eyev = Vec3(0.0, 0.0, -1.0);
        let normalv = Vec3(0.0, 0.0, -1.0);
        let light = Light::new_point_light(Point(0.0, 0.0, 10.0), Color::white());
        let res = m.lighting(&object, &light, &pos, &eyev, &normalv, false);

        let exact = 0.1;
        assert_eq!(res, Color(exact, exact, exact));
    }

    #[test]
    fn lighting_eye_in_path_of_reflection_vec() {
        let object = Sphere::default().as_shape();
        let m = Material::default();
        let pos = Point(0.0, 0.0, 0.0);
        let eyev = Vec3(0.0, -ROOT2, -ROOT2);
        let normalv = Vec3(0.0, 0.0, -1.0);
        let light = Light::new_point_light(Point(0.0, 10.0, -10.0), Color::white());
        let res = m.lighting(&object, &light, &pos, &eyev, &normalv, false);

        let exact = 0.1 + 0.9 * ROOT2 + 0.9;
        assert_eq!(res, Color(exact, exact, exact));
    }

    #[test]
    fn lighting_eye_opposite_surface_light_offset_45() {
        let object = Sphere::default().as_shape();
        let m = Material::default();
        let pos = Point(0.0, 0.0, 0.0);
        let eyev = Vec3(0.0, 0.0, -1.0);
        let normalv = Vec3(0.0, 0.0, -1.0);
        let light = Light::new_point_light(Point(0.0, 10.0, -10.0), Color::white());
        let res = m.lighting(&object, &light, &pos, &eyev, &normalv, false);

        let exact = 0.1 + 0.9 * ROOT2;
        let want = Color(exact, exact, exact);
        assert!((res.0 - want.0).abs() < 1e-4);
        assert!((res.1 - want.1).abs() < 1e-4);
        assert!((res.2 - want.2).abs() < 1e-4);
    }

    #[test]
    fn lighting_eye_between_light_and_surface_offset_45() {
        let object = Sphere::default().as_shape();
        let m = Material::default();
        let pos = Point(0.0, 0.0, 0.0);
        let eyev = Vec3(0.0, ROOT2, -ROOT2);
        let normalv = Vec3(0.0, 0.0, -1.0);
        let light = Light::new_point_light(Point(0.0, 0.0, -10.0), Color::white());
        let res = m.lighting(&object, &light, &pos, &eyev, &normalv, false);

        assert_eq!(res, Color::white());
    }

    #[test]
    fn lighting_eye_between_light_and_surface() {
        let object = Sphere::default().as_shape();
        let m = Material::default();
        let pos = Point(0.0, 0.0, 0.0);
        let eyev = Vec3(0.0, 0.0, -1.0);
        let normalv = Vec3(0.0, 0.0, -1.0);
        let light = Light::new_point_light(Point(0.0, 0.0, -10.0), Color::white());
        let res = m.lighting(&object, &light, &pos, &eyev, &normalv, false);

        assert_eq!(res, Color(1.9, 1.9, 1.9));
    }

    #[test]
    fn materials_have_a_default() {
        let m = Material::default();

        assert_eq!(m.color, Color::white());
        assert_eq!(m.ambient, 0.1);
        assert_eq!(m.diffuse, 0.9);
        assert_eq!(m.specular, 0.9);
        assert_eq!(m.shininess, 200.0);
    }
}
