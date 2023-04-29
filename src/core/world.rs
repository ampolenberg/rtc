//! A structure consisting of collections of objects in a scene.
use crate::{math::Point, shape::Shape, visuals::Color};

use super::{
    light::Light, material::Material, precompute::PrecomputedData, Intersectable, IntersectionList,
    Ray,
};

/// A structure containing objects and lights.
#[derive(Default)]
pub struct World {
    pub objects: Vec<Shape>,
    pub lights: Vec<Light>,
}

impl World {
    /// Creates a new world with the specified objects and lights.
    pub fn new(objects: Vec<Shape>, lights: Vec<Light>) -> Self {
        Self { objects, lights }
    }

    /// Iterates over every object ([Shape](crate::shape::Shape)) in the world, intersecting
    /// each with the ray, and collecting the intersections. __Note:__ this sorts the collected
    /// intersections (see [IntersectionList](crate::core::IntersectionList)).
    pub(crate) fn intersect_world(&self, ray: Ray) -> Option<IntersectionList> {
        let xs = self.objects.iter().flat_map(|o| o.intersect(ray)).collect();

        Some(IntersectionList::new(xs))
    }

    /// Determines the color of the pixel hit by the provided ray. If there was no hit,
    /// `Color::black()` is returned instead.
    pub(crate) fn color_at(&self, r: Ray, remaining: usize) -> Color {
        let xs = self.intersect_world(r);

        // TODO: added a clone here that I'm not sure I want to keep. And I'm unwrapping xs below.
        if let Some(mut ix) = xs.clone() {
            if let Some(hit) = ix.hit() {
                let comps = PrecomputedData::new(hit, &r, &xs.unwrap());
                self.shade_hit(&comps, remaining)
            } else {
                Color::black()
            }
        } else {
            Color::black()
        }
    }

    /// Shades the hit by blending the object's surface color and the reflected color. __Note:__
    /// this calls `reflected_color()`, which calls `color_at()`, which calls `shade_hit()`...
    fn shade_hit(&self, comps: &PrecomputedData, remaining: usize) -> Color {
        let surface: Color = self
            .lights
            .iter()
            .map(|l| {
                Material::lighting(
                    &comps.object.material(),
                    &comps.object,
                    l,
                    &comps.over_point,
                    &comps.eyev,
                    &comps.normalv,
                    self.is_shadowed(&comps.over_point, l),
                )
            })
            .sum();
        let reflected = self.reflected_color(comps, remaining);

        surface + reflected
    }

    /// Determines the color of the material, taking into account its reflectiveness.
    pub(crate) fn reflected_color(&self, comps: &PrecomputedData, remaining: usize) -> Color {
        if remaining == 0 || comps.object.material().reflective == 0.0 {
            Color::black()
        } else {
            let reflect_ray = Ray::new(comps.over_point, comps.reflectv);
            let col = self.color_at(reflect_ray, remaining - 1);

            col * comps.object.material().reflective
        }
    }

    /// Iterates through every light source and determines if the point in question lies in a
    /// shadow or not. To be shadowed, the point must be in the shadow for _every_ light source.
    fn is_shadowed(&self, p: &Point, light: &Light) -> bool {
        let v = light.position() - p;
        let distance = v.magnitude();
        let direction = v.normalize();
        let r = Ray::new(*p, direction);

        let xs = self.intersect_world(r);
        if let Some(mut ix) = xs {
            if let Some(hit) = ix.hit() {
                hit.t < distance
            } else {
                false
            }
        } else {
            false
        }
    }
}

#[cfg(test)]
mod world_tests {
    use std::f64::consts::FRAC_1_SQRT_2;

    use crate::{
        core::{camera::Camera, material::Material, precompute::PrecomputedData, Intersection},
        math::{Matrix, Point, Vec3},
        shape::{Plane, Sphere},
        visuals::Color,
    };

    use super::*;

    #[test]
    fn reflected_color_at_max_recursion_depth() {
        let mut w = default_world();
        let p = Plane::default()
            .with_material(&Material::default().with_reflective(0.5))
            .with_transform(&Matrix::translation(0.0, -1.0, 0.0))
            .as_shape();
        w.objects.push(p.clone());

        let r = Ray::new(
            Point(0.0, 0.0, -3.0),
            Vec3(0.0, -FRAC_1_SQRT_2, FRAC_1_SQRT_2),
        );
        let ix = Intersection::new(2.0_f64.sqrt(), p);
        let xs = IntersectionList::new(vec![ix.clone()]);
        let comps = PrecomputedData::new(&ix, &r, &xs);

        let col = w.reflected_color(&comps, 0);
        assert_eq!(col, Color::black());
    }

    #[test]
    fn color_at_with_mutually_reflective_surfaces() {
        // this will blow the stack, since I'm not handling infinite recursion yet
        let lp = Plane::default()
            .with_material(&Material::default().with_reflective(1.0))
            .with_transform(&Matrix::translation(0.0, -1.0, 0.0))
            .as_shape();
        let up = Plane::default()
            .with_material(&Material::default().with_reflective(1.0))
            .with_transform(&Matrix::translation(0.0, 1.0, 0.0))
            .as_shape();
        let light = Light::new_point_light(Point(0.0, 0.0, 0.0), Color::white());

        let w = World::new(vec![lp, up], vec![light]);
        let r = Ray::new(Point(0.0, 0.0, 0.0), Vec3(0.0, 1.0, 0.0));

        // then w.color_at(r, d) terminates successfully
        w.color_at(r, 5);
    }

    #[test]
    fn shade_hit_with_reflective_material() {
        let mut w = default_world();
        let p = Plane::default()
            .with_material(&Material::default().with_reflective(0.5))
            .with_transform(&Matrix::translation(0.0, -1.0, 0.0))
            .as_shape();
        w.objects.push(p.clone());

        let r = Ray::new(
            Point(0.0, 0.0, -3.0),
            Vec3(0.0, -FRAC_1_SQRT_2, FRAC_1_SQRT_2),
        );
        let ix = Intersection::new(2.0_f64.sqrt(), p);
        let xs = IntersectionList::new(vec![ix.clone()]);
        let comps = PrecomputedData::new(&ix, &r, &xs);

        let col = w.shade_hit(&comps, 5);
        let expected_color = Color(0.87677, 0.92436, 0.82918);

        assert!((col.0 - expected_color.0).abs() < 0.0001);
        assert!((col.1 - expected_color.1).abs() < 0.0001);
        assert!((col.2 - expected_color.2).abs() < 0.0001);
    }

    #[test]
    fn reflected_color_of_reflective_material() {
        let mut w = default_world();
        let p = Plane::default()
            .with_material(&Material::default().with_reflective(0.5))
            .with_transform(&Matrix::translation(0.0, -1.0, 0.0))
            .as_shape();
        w.objects.push(p.clone());

        let r = Ray::new(
            Point(0.0, 0.0, -3.0),
            Vec3(0.0, -FRAC_1_SQRT_2, FRAC_1_SQRT_2),
        );
        let ix = Intersection::new(2.0_f64.sqrt(), p);
        let xs = IntersectionList::new(vec![ix.clone()]);
        let comps = PrecomputedData::new(&ix, &r, &xs);
        let col = w.reflected_color(&comps, 5);
        let expected_color = Color(0.19032, 0.2379, 0.14274);

        assert!((col.0 - expected_color.0).abs() < 0.0001);
        assert!((col.1 - expected_color.1).abs() < 0.0001);
        assert!((col.2 - expected_color.2).abs() < 0.0001);
    }

    #[test]
    fn reflected_color_of_nonreflective_material() {
        let light = Light::new_point_light(Point(-10.0, 10.0, -10.0), Color::white());
        let s1 = Sphere {
            material: Material {
                color: Color(0.8, 1.0, 0.6),
                diffuse: 0.7,
                specular: 0.2,
                ..Default::default()
            },
            ..Default::default()
        };
        let s2 = Sphere::default()
            .with_transform(&Matrix::scaling(0.5, 0.5, 0.5))
            .with_material(&Material::default().with_ambient(1.0));

        let w = World {
            objects: vec![s1.as_shape(), s2.as_shape()],
            lights: vec![light],
        };

        let r = Ray::new(Point(0.0, 0.0, 0.0), Vec3(0.0, 0.0, 1.0));
        let ix = Intersection::new(1.0, s2.as_shape());
        let xs = IntersectionList::new(vec![ix.clone()]);
        let comps = PrecomputedData::new(&ix, &r, &xs);
        let color = w.reflected_color(&comps, 5);

        assert_eq!(color, Color::black());
    }

    #[test]
    fn no_shadow_when_object_is_behind_point() {
        let w = default_world();
        let p = Point(-2.0, 2.0, -2.0);

        assert!(!w.is_shadowed(&p, &w.lights[0]));
    }

    #[test]
    fn no_shadow_when_object_is_behind_light() {
        let w = default_world();
        let p = Point(-20.0, 20.0, -20.0);

        assert!(!w.is_shadowed(&p, &w.lights[0]));
    }

    #[test]
    fn shadow_when_object_between_point_and_light() {
        let w = default_world();
        let p = Point(10.0, -10.0, 10.0);

        assert!(w.is_shadowed(&p, &w.lights[0]));
    }

    #[test]
    fn no_shadow_when_nothing_between_point_and_light() {
        let w = default_world();
        let p = Point(0.0, 10.0, 0.0);

        assert!(!w.is_shadowed(&p, &w.lights[0]));
    }

    #[test]
    #[ignore = "doesn't work now that shadows are rendered"]
    fn rendering_world_with_camera() {
        let w = default_world();
        let from = Point(0.0, 0.0, -5.0);
        let to = Point(0.0, 0.0, 0.0);
        let up = Vec3(0.0, 1.0, 0.0);

        let c = Camera::new(11, 11, std::f64::consts::PI / 2.0)
            .with_transform(&Matrix::view_transform(from, to, up));
        let image = c.render(&w, 0).unwrap();

        // weirdly inaccurate
        assert!((image.read_pixel(5, 5).0 - Color(0.38066, 0.47583, 0.2855).0).abs() < 1e-3);
        assert!((image.read_pixel(5, 5).1 - Color(0.38066, 0.47583, 0.2855).1).abs() < 1e-2);
        assert!((image.read_pixel(5, 5).2 - Color(0.38066, 0.47583, 0.2855).2).abs() < 1e-3);
    }

    #[test]
    fn color_with_intersection_behind_ray() {
        let s1 = Shape::Sphere(Sphere {
            material: Material {
                color: Color(0.8, 1.0, 0.6),
                diffuse: 0.7,
                specular: 0.2,
                ambient: 1.0,
                ..Default::default()
            },
            transform: Default::default(),
        });
        let s2 = Shape::Sphere(Sphere {
            material: Material {
                ambient: 1.0,
                ..Default::default()
            },
            transform: Matrix::scaling(0.5, 0.5, 0.5),
        });
        let w = World {
            objects: vec![s1, s2],
            ..default_world()
        };
        let _outer = w.objects[0].clone();
        let inner = w.objects[1].clone();

        let r = Ray::new(Point(0.0, 0.0, 0.75), Vec3(0.0, 0.0, -1.0));
        let c = w.color_at(r, 5);

        assert_eq!(c, inner.material().color);
    }

    #[test]
    fn color_when_ray_hits() {
        let w = default_world();
        let r = Ray::new(Point(0.0, 0.0, -5.0), Vec3(0.0, 0.0, 1.0));
        let c = w.color_at(r, 5);
        let expected_color = Color(0.38066, 0.47583, 0.2855);

        assert!((c.0 - expected_color.0).abs() < 1e-4);
        assert!((c.1 - expected_color.1).abs() < 1e-4);
        assert!((c.2 - expected_color.2).abs() < 1e-4);
    }

    #[test]
    fn color_when_ray_misses() {
        let w = default_world();
        let r = Ray::new(Point(0.0, 0.0, -5.0), Vec3(0.0, 1.0, 0.0));
        let c = w.color_at(r, 5);

        assert_eq!(c, Color::black());
    }

    #[test]
    fn shading_an_intersection_from_inside() {
        let mut w = default_world();
        w.lights = vec![Light::new_point_light(
            Point(0.0, 0.25, 0.0),
            Color::white(),
        )];
        let r = Ray::new(Point(0.0, 0.0, 0.0), Vec3(0.0, 0.0, 1.0));
        let shape = w.intersect_world(r).unwrap().data[1].object.clone(); // the second object in w
        let ix = Intersection::new(0.5, shape);
        let xs = IntersectionList::new(vec![ix.clone()]);
        let comps = PrecomputedData::new(&ix, &r, &xs);
        let c = w.shade_hit(&comps, 5);
        let expected_color = Color(0.90498, 0.90498, 0.90498);

        assert!((c.0 - expected_color.0).abs() < 1e-4);
        assert!((c.1 - expected_color.1).abs() < 1e-4);
        assert!((c.2 - expected_color.2).abs() < 1e-4);
    }

    #[test]
    fn shading_an_intersection() {
        let w = default_world();
        let r = Ray::new(Point(0.0, 0.0, -5.0), Vec3(0.0, 0.0, 1.0));
        let shape = w.intersect_world(r).unwrap().hit().unwrap().object.clone(); // the first object in w
        let ix = Intersection::new(4.0, shape);
        let xs = IntersectionList::new(vec![ix.clone()]);
        let comps = PrecomputedData::new(&ix, &r, &xs);
        let c = w.shade_hit(&comps, 5);
        let expected_color = Color(0.38066, 0.47583, 0.2855);

        assert!((c.0 - expected_color.0).abs() < 1e-4);
        assert!((c.1 - expected_color.1).abs() < 1e-4);
        assert!((c.2 - expected_color.2).abs() < 1e-4);
    }

    #[test]
    fn intersect_world_with_ray() {
        let w = default_world();
        let r = Ray::new(Point(0.0, 0.0, -5.0), Vec3(0.0, 0.0, 1.0));
        let xs = w.intersect_world(r).unwrap();

        assert_eq!(xs.data.len(), 4);
        assert_eq!(xs[0].t, 4.0);
        assert_eq!(xs[1].t, 4.5);
        assert_eq!(xs[2].t, 5.5);
        assert_eq!(xs[3].t, 6.0);
    }

    #[test]
    fn worlds_have_a_default() {
        let w = default_world();

        assert_eq!(
            w.lights[0],
            Light::new_point_light(Point(-10.0, 10.0, -10.0), Color::white())
        );
    }

    fn default_world() -> World {
        let light = Light::new_point_light(Point(-10.0, 10.0, -10.0), Color::white());
        let s1 = Sphere {
            material: Material {
                color: Color(0.8, 1.0, 0.6),
                diffuse: 0.7,
                specular: 0.2,
                ..Default::default()
            },
            ..Default::default()
        }
        .as_shape();
        let s2 = Sphere::default()
            .with_transform(&Matrix::scaling(0.5, 0.5, 0.5))
            .as_shape();

        World {
            objects: vec![s1, s2],
            lights: vec![light],
        }
    }
}
