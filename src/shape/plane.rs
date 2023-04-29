use crate::{
    core::{material::Material, Intersection, IntersectionList, Ray, EPS},
    math::{Matrix, Point, Tuple, Vec3},
};

use super::Shape;

#[derive(Clone, PartialEq, Debug)]
pub struct Plane {
    pub transform: Matrix<4>,
    pub material: Material,
}

impl Plane {
    /// Applies the given transformation matrix to the plane.
    pub fn with_transform(mut self, m: &Matrix<4>) -> Self {
        self.transform = *m;
        self
    }

    /// Assigns the given material to the associated plane.
    pub fn with_material(mut self, m: &Material) -> Self {
        self.material = (*m).clone();
        self
    }

    /// Small helper function just to make things a bit less tedious.
    pub fn as_shape(&self) -> Shape {
        Shape::from(self)
    }

    /// Planes in `xz`-space always have `Vec3(0.0, 1.0, 0.0)` as normal vector.
    pub(super) fn normal_at_world_pt(&self, _world_pt: Point) -> Option<Vec3> {
        if let Some(inv) = self.transform.inverse() {
            let object_normal = Vec3(0.0, 1.0, 0.0);
            let world_normal = inv.transpose() * object_normal;

            Some(world_normal.normalize())
        } else {
            None
        }
    }

    /// Checks if the ray intersects with the plane and stores the intersection data in a `Vec`.
    pub(super) fn intersect(&self, r: Ray) -> Option<IntersectionList> {
        let tr = r.transform(self.transform.inverse()?);

        if tr.direction.y().abs() < EPS {
            None
        } else {
            let t = -tr.origin.y() / tr.direction.y();
            let i1 = Intersection::new(t, Shape::from(self));

            Some(IntersectionList::new(vec![i1]))
        }
    }
}

impl Default for Plane {
    fn default() -> Self {
        Self {
            transform: Matrix::identity(),
            material: Material::default(),
        }
    }
}

impl From<Plane> for Shape {
    fn from(p: Plane) -> Self {
        Self::Plane(p)
    }
}

impl From<&Plane> for Shape {
    fn from(p: &Plane) -> Self {
        Self::Plane((*p).clone())
    }
}

#[cfg(test)]
mod plane_tests {
    use super::*;
    use crate::math::{Point, Vec3};

    #[test]
    fn ray_intersecting_plane_from_below() {
        let p = Plane::default();
        let r = Ray::new(Point(0.0, -1.0, 0.0), Vec3(0.0, 1.0, 0.0));
        let xs = p.intersect(r).unwrap();

        assert_eq!(xs.data.len(), 1);
        assert_eq!(xs.data[0].t, 1.0);
        assert_eq!(xs.data[0].object, p.as_shape());
    }

    #[test]
    fn ray_intersecting_plane_from_above() {
        let p = Plane::default();
        let r = Ray::new(Point(0.0, 1.0, 0.0), Vec3(0.0, -1.0, 0.0));
        let xs = p.intersect(r).unwrap();

        assert_eq!(xs.data.len(), 1);
        assert_eq!(xs.data[0].t, 1.0);
        assert_eq!(xs.data[0].object, p.as_shape());
    }

    #[test]
    fn intersect_plane_with_coplanar_ray() {
        let p = Plane::default();
        let r = Ray::new(Point(0.0, 0.0, 0.0), Vec3(0.0, 0.0, 1.0));
        let xs = p.intersect(r);

        assert!(xs.is_none());
    }

    #[test]
    fn intersect_plane_with_parallel_ray() {
        let p = Plane::default();
        let r = Ray::new(Point(0.0, 10.0, 0.0), Vec3(0.0, 0.0, 1.0));
        let xs = p.intersect(r);

        assert!(xs.is_none());
    }

    #[test]
    fn normal_of_plane_is_constant_everywhere() {
        let p = Plane::default();
        let n1 = p.normal_at_world_pt(Point(0.0, 0.0, 0.0)).unwrap();
        let n2 = p.normal_at_world_pt(Point(10.0, 0.0, -10.0)).unwrap();
        let n3 = p.normal_at_world_pt(Point(-5.0, 0.0, 150.0)).unwrap();

        assert_eq!(n1, Vec3(0.0, 1.0, 0.0));
        assert_eq!(n2, Vec3(0.0, 1.0, 0.0));
        assert_eq!(n3, Vec3(0.0, 1.0, 0.0));
    }
}
