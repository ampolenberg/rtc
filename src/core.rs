//! The most important methods and data structures for rendering.
//!
//! Allows for the creation of "intersectable" objects/shapes, organizing them into meaningful
//! lists, and determining collisions between rays and those objects.
use crate::math::{Point, Vec3};
use crate::shape::Shape;

pub mod antialias;
pub mod camera;
pub mod light;
pub mod material;
pub mod pattern;
pub mod precompute;
pub mod ray;
pub mod world;

pub use crate::core::camera::Camera;
pub use crate::core::light::Light;
pub use crate::core::material::Material;
pub use crate::core::pattern::Pattern;
pub use crate::core::ray::Ray;
pub use crate::core::world::World;

pub const EPS: f64 = 0.00001;

/// A trait for defining which objects are able to be hit by rays.
pub trait Intersectable {
    /// Intersects the object with the specified ray. Stores each intersection in a growable list.
    /// Returns `None` if no hits were found.
    fn intersect(&self, r: ray::Ray) -> Option<IntersectionList>;

    /// Computes the normal vector at the given point in world-space coordinates. Returns `None` if
    /// the normal can't be computed. This happens when the inverse transform matrix doesn't exist.
    fn normal_at(&self, world_pt: Point) -> Option<Vec3>;
}

/// Stores data from intersections; specifically, the times `t` of the intersection(s) and the
/// object that was hit.
#[derive(Debug, PartialEq, Clone)]
pub struct Intersection {
    pub t: f64,
    pub object: Shape,
}

impl Intersection {
    /// Creates a new intersection from a time-value `t` and an object type (the object's `Shape`).
    pub fn new(t: f64, object: Shape) -> Self {
        Self { t, object }
    }
}

/// Growable list of intersection data.
#[derive(Default, Clone, PartialEq, Debug)]
pub struct IntersectionList {
    pub data: Vec<Intersection>,
}

impl IntersectionList {
    /// Creating a new `IntersectionList` automatically sorts the intersection data by `t`. This
    /// function is just for testing purposes, since you need to accumulate all your intersection
    /// data into a `Vec<Intersection>` before calling `IntersectionList::new(..)`, which isn't
    /// practical.
    #[allow(dead_code)]
    pub(crate) fn new(mut list: Vec<Intersection>) -> Self {
        list.sort_unstable_by(|a, b| a.t.partial_cmp(&b.t).unwrap());

        Self { data: list }
    }

    /// Sorts the `IntersectionList` data and finds the minimum positive `t` value. Right now it
    /// filters to ensure `t` is positive and that `t` is neither [INF](f64::INFINITY) nor
    /// [NaN](f64::NAN). Infinity may be useful in the future? So this may need to be adjusted.
    /// (Note to self...)
    pub fn hit(&mut self) -> Option<&Intersection> {
        self.data
            .sort_unstable_by(|a, b| a.t.partial_cmp(&b.t).unwrap());

        self.data
            .iter()
            .filter(|x| x.t.is_finite() && x.t.is_sign_positive())
            .min_by(|x, y| x.t.partial_cmp(&y.t).unwrap())
    }
}

impl std::ops::Index<usize> for IntersectionList {
    type Output = Intersection;

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl FromIterator<IntersectionList> for Vec<Intersection> {
    fn from_iter<T: IntoIterator<Item = IntersectionList>>(iter: T) -> Self {
        let mut data = Vec::new();
        for it in iter.into_iter().flatten() {
            data.push(it)
        }

        data
    }
}

impl IntoIterator for IntersectionList {
    type Item = Intersection;
    type IntoIter = std::vec::IntoIter<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter()
    }
}
