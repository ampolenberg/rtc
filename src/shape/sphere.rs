//! A fundamental object for rendering.
use crate::{
    core::{material::Material, Intersection, IntersectionList, Ray},
    math::{Matrix, Point, Vec3},
};

use super::Shape;

/// Spheres are the most basic and fundamental shape to implement. We're assuming all spheres are
/// centered at the origin and have radius one. This can be modified via matrix transformations.
#[derive(Debug, Clone, PartialEq)]
pub struct Sphere {
    pub transform: Matrix<4>,
    pub material: Material,
}

impl Sphere {
    /// Applies the transformation to the sphere.
    pub fn with_transform(mut self, m: &Matrix<4>) -> Self {
        self.transform = *m;
        self
    }

    /// Assigns the given material to the associated sphere.
    pub fn with_material(mut self, m: &Material) -> Self {
        self.material = (*m).clone();
        self
    }

    /// Small helper function just to make things a bit less tedious.
    pub fn as_shape(&self) -> Shape {
        Shape::from(self)
    }

    /// Computes the normal vector of the sphere at the given _world_ point.
    ///
    /// Relies on the inverse of the transform matrix applied to the sphere. Returns [`None`] if
    /// the inverse doesn't exist.
    pub(in crate::shape) fn normal_at_world_pt(&self, world_pt: Point) -> Option<Vec3> {
        if let Some(inv) = self.transform.inverse() {
            let object_pt = inv * world_pt;
            let object_normal = object_pt - Point(0.0, 0.0, 0.0);
            let world_normal = inv.transpose() * object_normal;

            Some(world_normal.normalize())
        } else {
            None
        }
    }

    pub(in crate::shape) fn intersect(&self, r: Ray) -> Option<IntersectionList> {
        let tr = r.transform(self.transform.inverse()?);
        let sphere_to_ray = tr.origin - Point(0.0, 0.0, 0.0); // assuming every sphere is centered at the world origin

        let a = tr.direction.dot(&tr.direction);
        let b = 2.0 * tr.direction.dot(&sphere_to_ray);
        let c = sphere_to_ray.dot(&sphere_to_ray) - 1.0;

        let discrim = b * b - 4.0 * a * c;

        if discrim < 0.0 {
            return None;
        }

        let t1 = (-b - f64::sqrt(discrim)) / (2.0 * a);
        let t2 = (-b + f64::sqrt(discrim)) / (2.0 * a);
        let i1 = Intersection::new(t1, Shape::from(self));
        let i2 = Intersection::new(t2, Shape::from(self));

        Some(IntersectionList { data: vec![i1, i2] })
    }
}

impl Default for Sphere {
    fn default() -> Self {
        Self {
            transform: Matrix::identity(),
            material: Material::default(),
        }
    }
}

impl From<Sphere> for Shape {
    fn from(s: Sphere) -> Self {
        Self::Sphere(s)
        // Self::Sphere
    }
}

impl From<&Sphere> for Shape {
    fn from(s: &Sphere) -> Self {
        Self::Sphere((*s).clone())
        // Self::Sphere
    }
}

#[cfg(test)]
mod sphere_tests {
    use super::*;
    use crate::{core::IntersectionList, math::Vec3};
    use std::f64::consts::FRAC_1_SQRT_2;
    use std::f64::consts::PI;

    #[test]
    fn spheres_can_be_assigned_materials() {
        let mut s = Sphere::default();
        let m = Material {
            ambient: 1.0,
            ..Default::default()
        };
        s.material = m.clone();

        assert_eq!(s.material, m);
    }

    #[test]
    fn spheres_have_default_material() {
        let s = Sphere::default();
        let m = s.material;

        assert_eq!(m, Material::default());
    }

    #[test]
    fn normal_of_transformed_sphere() {
        let m = Matrix::scaling(1.0, 0.5, 1.0)
            * Matrix::rotation(crate::math::matrix::Axis::Z, PI / 5.0);
        let s = Sphere::default().with_transform(&m);
        let n = s
            .normal_at_world_pt(Point(0.0, FRAC_1_SQRT_2, -FRAC_1_SQRT_2))
            .unwrap();
        let v = Vec3(0.0, 0.97014, -0.24254);

        for i in 0..3 {
            assert!((n[i] - v[i]).abs() < 1e-4);
        }
    }

    #[test]
    fn normal_of_translated_sphere() {
        let s = Sphere::default().with_transform(&Matrix::translation(0.0, 1.0, 0.0));
        let n = s
            .normal_at_world_pt(Point(0.0, 1.70711, -FRAC_1_SQRT_2))
            .unwrap();
        let want = Vec3(0.0, FRAC_1_SQRT_2, -FRAC_1_SQRT_2);

        for i in 0..3 {
            assert!((n[i] - want[i]).abs() < 1e-4);
        }
    }

    #[test]
    fn normal_at_nonaxial_point_on_sphere() {
        let s = Sphere::default();
        let p = Point(
            3.0_f64.sqrt() / 3.0,
            3.0_f64.sqrt() / 3.0,
            3.0_f64.sqrt() / 3.0,
        );
        let n = s.normal_at_world_pt(p).unwrap();
        let v = Vec3(
            3.0_f64.sqrt() / 3.0,
            3.0_f64.sqrt() / 3.0,
            3.0_f64.sqrt() / 3.0,
        );

        assert_eq!(n, v);
    }

    #[test]
    fn normal_at_point_on_z_axis() {
        let s = Sphere::default();
        let n = s.normal_at_world_pt(Point(0.0, 0.0, 1.0)).unwrap();

        assert_eq!(n, Vec3(0.0, 0.0, 1.0));
    }

    #[test]
    fn normal_at_point_on_y_axis() {
        let s = Sphere::default();
        let n = s.normal_at_world_pt(Point(0.0, 1.0, 0.0)).unwrap();

        assert_eq!(n, Vec3(0.0, 1.0, 0.0));
    }

    #[test]
    fn normal_at_point_on_x_axis() {
        let s = Sphere::default();
        let n = s.normal_at_world_pt(Point(1.0, 0.0, 0.0)).unwrap();

        assert_eq!(n, Vec3(1.0, 0.0, 0.0));
    }

    #[test]
    fn default_sphere_transform() {
        let s = Sphere::default();

        assert_eq!(s.transform, Matrix::identity());
    }

    #[test]
    fn sphere_transforms_can_be_changed() {
        let s = Sphere::default();
        let t = Matrix::translation(2.0, 3.0, 4.0);
        let s = s.with_transform(&t);

        assert_eq!(s.transform, t);
    }

    #[test]
    fn intersecting_translated_sphere_with_ray() {
        let s = Sphere::default().with_transform(&Matrix::translation(5.0, 0.0, 0.0));
        let r = Ray::new(Point(0.0, 0.0, -5.0), Vec3(0.0, 0.0, 1.0));
        let xs = s.intersect(r);

        assert_eq!(xs, None);
    }

    #[test]
    fn scaled_sphere_intersect_with_ray() {
        let r = Ray::new(Point(0.0, 0.0, -5.0), Vec3(0.0, 0.0, 1.0));
        let s = Sphere::default().with_transform(&Matrix::scaling(2.0, 2.0, 2.0));
        let xs = s.intersect(r).unwrap();

        assert_eq!(xs.data.len(), 2);
        assert_eq!(xs[0].t, 3.0);
        assert_eq!(xs[1].t, 7.0);
    }

    #[test]
    fn can_find_hit_in_mixed_intersections() {
        let s = Sphere::default();
        let i1 = Intersection::new(5.0, s.as_shape());
        let i2 = Intersection::new(7.0, s.as_shape());
        let i3 = Intersection::new(-3.0, s.as_shape());
        let i4 = Intersection::new(2.0, s.as_shape());
        let mut xs = IntersectionList::new(vec![i1, i2, i3, i4.clone()]);

        assert_eq!(xs.hit().unwrap(), &i4);
    }

    #[test]
    fn finding_hit_with_all_negative_times() {
        let s = Sphere::default();
        let i1 = Intersection::new(-2.0, s.as_shape());
        let i2 = Intersection::new(-1.0, s.as_shape());
        let mut xs = IntersectionList::new(vec![i2, i1]);

        assert!(xs.hit().is_none());
    }

    #[test]
    fn finding_hit_with_some_negative_times() {
        let s = Sphere::default();
        let i1 = Intersection::new(-1.0, s.as_shape());
        let i2 = Intersection::new(1.0, s.as_shape());
        let mut xs = IntersectionList::new(vec![i2.clone(), i1]);

        assert_eq!(xs.hit().unwrap(), &i2);
    }

    #[test]
    fn finding_hit_with_positive_times() {
        let s = Sphere::default();
        let i1 = Intersection::new(1.0, s.as_shape());
        let i2 = Intersection::new(2.0, s.as_shape());
        let mut xs = IntersectionList::new(vec![i1.clone(), i2]);

        assert_eq!(*xs.hit().unwrap(), i1);
    }

    #[test]
    fn intersection_sets_object() {
        let r = Ray::new(Point(0.0, 0.0, -5.0), Vec3(0.0, 0.0, 1.0));
        let s = Sphere::default();
        let xs = s.intersect(r).unwrap();

        assert_eq!(xs[0].object, Shape::from(&s));
        assert_eq!(xs[1].object, Shape::from(s));
    }

    #[test]
    fn aggregating_intersections_into_list() {
        let s = Sphere::default();
        let i1 = Intersection::new(1.0, Shape::from(&s));
        let i2 = Intersection::new(2.0, Shape::from(&s));
        let xs = IntersectionList { data: vec![i1, i2] };

        assert_eq!(xs[0].t, 1.0);
        assert_eq!(xs[1].t, 2.0);
    }

    #[test]
    fn sphere_intersection_behind_rays() {
        let r = Ray::new(Point(0.0, 0.0, 5.0), Vec3(0.0, 0.0, 1.0));
        let s = Sphere::default();
        let xs = s.intersect(r).unwrap();

        assert_eq!(xs[0].t, -6.0);
        assert_eq!(xs[1].t, -4.0);
    }

    #[test]
    fn rays_originating_inside_sphere() {
        let r = Ray::new(Point(0.0, 0.0, 0.0), Vec3(0.0, 0.0, 1.0));
        let s = Sphere::default();
        let xs = s.intersect(r).unwrap();

        assert_eq!(xs[0].t, -1.0);
        assert_eq!(xs[1].t, 1.0);
    }

    #[test]
    fn rays_can_miss_spheres() {
        let p = Point(0.0, 2.0, -5.0);
        let v = Vec3(0.0, 0.0, 1.0);
        let r = Ray::new(p, v);
        let s = Sphere::default();
        let xs = s.intersect(r);

        assert!(xs.is_none());
    }

    #[test]
    fn ray_intersecting_at_tangent() {
        let p = Point(0.0, 1.0, -5.0);
        let v = Vec3(0.0, 0.0, 1.0);
        let r = Ray::new(p, v);
        let s = Sphere::default();
        let xs = s.intersect(r).unwrap();

        assert_eq!(xs[0].t, 5.0);
        assert_eq!(xs[1].t, 5.0);
    }

    #[test]
    fn ray_intersects_at_two_points() {
        let r = Ray::new(Point(0.0, 0.0, -5.0), Vec3(0.0, 0.0, 1.0));
        let s = Sphere::default();
        let xs = s.intersect(r).unwrap();

        assert_eq!(xs[0].t, 4.0);
        assert_eq!(xs[1].t, 6.0);
    }
}
