//! An enumeration of intersectable shapes.
use crate::{
    core::{material::Material, Intersectable, IntersectionList},
    math::Matrix,
};

pub mod plane;
pub mod sphere;

pub use plane::Plane;
pub use sphere::Sphere;

/// A catalogue of shapes to render.
#[derive(Debug, PartialEq, Clone)]
pub enum Shape {
    Sphere(Sphere),
    Plane(Plane),
}

impl Shape {
    pub(crate) fn material(&self) -> Material {
        match *self {
            Self::Sphere(ref sphere) => sphere.material.clone(),
            Self::Plane(ref plane) => plane.material.clone(),
        }
    }

    /// Gets the shape's transform.
    pub(crate) fn transform(&self) -> Matrix<4> {
        match *self {
            Self::Sphere(ref sphere) => sphere.transform,
            Self::Plane(ref plane) => plane.transform,
        }
    }
}

impl Intersectable for Shape {
    fn intersect(&self, r: crate::core::Ray) -> Option<IntersectionList> {
        match *self {
            Shape::Sphere(ref sphere) => sphere.intersect(r),
            Shape::Plane(ref plane) => plane.intersect(r),
        }
    }

    fn normal_at(&self, world_pt: crate::math::Point) -> Option<crate::math::Vec3> {
        match *self {
            Shape::Sphere(ref sphere) => sphere.normal_at_world_pt(world_pt),
            Shape::Plane(ref plane) => plane.normal_at_world_pt(world_pt),
        }
    }
}
