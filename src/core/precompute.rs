use super::{Intersectable, Intersection, IntersectionList, Ray, EPS};
use crate::{
    math::{Point, Vec3},
    shape::Shape,
};

/// Storage for computations to be used by other methods/in other modules.
#[allow(dead_code)]
pub struct PrecomputedData {
    pub(crate) t: f64,
    pub(crate) object: Shape,
    pub(crate) point: Point,

    /// Eye vector.
    pub(crate) eyev: Vec3,

    /// Normal vector. Relies on the [normal_at](crate::core::Intersectable::normal_at) function,
    /// which itself relies on the inverse transformation matrix. So this should _technically_ be
    /// an `Option<Vec3>`, but we'll see.
    pub(crate) normalv: Vec3,

    /// Whether or not the current ray is inside the object.
    pub(crate) inside: bool,

    /// Corrects for floating point error, i.e. shadow acne.
    pub(crate) over_point: Point,

    /// The reflection vector.
    pub(crate) reflectv: Vec3,

    /// TODO: docs
    pub(crate) n1: f64,
    pub(crate) n2: f64,
}

impl PrecomputedData {
    pub(crate) fn new(ix: &Intersection, ray: &Ray, xs: &IntersectionList) -> Self {
        let t = ix.t;
        let object = ix.object.clone();
        let world_point = ray.position(t);
        let eyev = -ray.direction;
        let mut normalv = object
            .normal_at(world_point)
            .expect("singular transform matrix! Could not invert.");
        let inside = normalv.dot(&eyev) < 0.0;

        if inside {
            normalv = -normalv;
        }

        let reflectv = ray.direction.reflect(&normalv);
        let over_point = world_point + normalv * EPS;

        let (n1, n2) = set_refractive_indices(ix, xs);

        Self {
            t,
            object,
            point: world_point,
            eyev,
            normalv,
            inside,
            over_point,
            reflectv,
            n1,
            n2,
        }
    }
}

/// This is super un-optimized.
fn set_refractive_indices(ix: &Intersection, xs: &IntersectionList) -> (f64, f64) {
    let mut containers: Vec<Shape> = Vec::new();
    let mut n1 = None;
    let mut n2 = None;

    for interesction in xs.data.iter() {
        if interesction == ix {
            n1 = containers.last().map(|o| o.material().refractive_index);
        }

        let contents = containers.iter().position(|o| *o == interesction.object);
        if let Some(object_at) = contents {
            containers.remove(object_at);
        } else {
            containers.push(interesction.clone().object);
        }

        if interesction == ix {
            n2 = containers.last().map(|o| o.material().refractive_index);

            break;
        }
    }

    (n1.unwrap_or(1.0), n2.unwrap_or(1.0))
}

#[cfg(test)]
mod precomputed_data_tests {
    use std::f64::consts::FRAC_1_SQRT_2;

    use crate::shape::{Plane, Sphere};

    use super::*;

    #[test]
    fn precomputing_reflection_vector() {
        let p = Plane::default().as_shape();
        let r = Ray::new(
            Point(0.0, 1.0, -1.0),
            Vec3(0.0, -FRAC_1_SQRT_2, FRAC_1_SQRT_2),
        );
        let ix = Intersection::new(2.0_f64.sqrt(), p);
        let xs = IntersectionList::new(vec![ix.clone()]);
        let comps = PrecomputedData::new(&ix, &r, &xs);

        assert_eq!(comps.reflectv, Vec3(0.0, FRAC_1_SQRT_2, FRAC_1_SQRT_2));
    }

    #[test]
    fn hit_when_intersection_on_inside() {
        let r = Ray::new(Point(0.0, 0.0, 0.0), Vec3(0.0, 0.0, 1.0));
        let s = Sphere::default();
        let i = Intersection::new(1.0, s.as_shape());
        let xs = IntersectionList::new(vec![i.clone()]);
        let comps = PrecomputedData::new(&i, &r, &xs);

        assert_eq!(comps.point, Point(0.0, 0.0, 1.0));
        assert_eq!(comps.eyev, Vec3(0.0, 0.0, -1.0));
        assert!(comps.inside);
        // normal would be (0, 0, 1), but it's inverted since we're inside the sphere
        assert_eq!(comps.normalv, Vec3(0.0, 0.0, -1.0));
    }

    #[test]
    fn hit_when_intersection_on_outside() {
        let r = Ray::new(Point(0.0, 0.0, -5.0), Vec3(0.0, 0.0, 1.0));
        let s = Sphere::default();
        let i = Intersection::new(4.0, s.as_shape());
        let xs = IntersectionList::new(vec![i.clone()]);
        let comps = PrecomputedData::new(&i, &r, &xs);

        assert!(!comps.inside);
    }

    #[test]
    fn precomputing_state_of_intersection() {
        let r = Ray::new(Point(0.0, 0.0, -5.0), Vec3(0.0, 0.0, 1.0));
        let s = Sphere::default();
        let i = Intersection::new(4.0, s.as_shape());
        let xs = IntersectionList::new(vec![i.clone()]);
        let comps = PrecomputedData::new(&i, &r, &xs);

        assert_eq!(comps.t, i.t);
        assert_eq!(comps.object, i.object);
        assert_eq!(comps.point, Point(0.0, 0.0, -1.0));
        assert_eq!(comps.eyev, Vec3(0.0, 0.0, -1.0));
        assert_eq!(comps.normalv, Vec3(0.0, 0.0, -1.0));
    }
}
