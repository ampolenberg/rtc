//! Cameras organize the view of the world.
//!
//! TODO: implement `Default` for `Camera`; it'll make parsing yaml files easier (e.g., if
//! something important is missing, use default).
use super::{
    antialias::{AAMethod, AntiAliasing},
    world::World,
    Ray,
};
use crate::{
    io::error::RenderError,
    math::{Matrix, Point},
    visuals::{canvas::Canvas, Color},
};

use rayon::iter::{IntoParallelIterator, ParallelIterator};
use std::sync::{Arc, Mutex};

/// Cameras are specified with a horizontal size, vertical size, and a field-of-view.
///
/// # Example
/// ```ignore
/// let cam = Camera::new(1920, 1080, std::f64::consts::PI / 3.0);
/// ```
///
/// They can then render an established [World](crate::core::world::World) onto a
/// [Canvas](crate::visuals::canvas::Canvas):
///
/// ```ignore
/// let canvas = cam.render(&world).unwrap();
/// ```
pub struct Camera {
    hsize: usize,
    vsize: usize,
    #[allow(dead_code)]
    fov: f64,
    transform: Matrix<4>,
    pixel_size: f64,
    half_width: f64,
    half_height: f64,
    aa: AntiAliasing,
}

impl Camera {
    /// Constructs a new camera object with specified horizontal and vertical sizes as well as
    /// field of view.
    pub fn new(hsize: usize, vsize: usize, fov: f64) -> Self {
        let (pixel_size, half_width, half_height) = Self::set_private_fields(hsize, vsize, fov);

        Self {
            hsize,
            vsize,
            fov,
            transform: Matrix::identity(),
            pixel_size,
            half_width,
            half_height,
            aa: AntiAliasing::default(),
        }
    }

    /// Creates a ray with origin at the camera and passes through the given pixel coordinates on
    /// the canvas. Returns an `Option<Ray>` since the inverse of the transform matrix may not
    /// exist.
    pub(crate) fn ray_for_pixel(
        &self,
        px: usize,
        py: usize,
        x_offset: f64,
        y_offset: f64,
    ) -> Option<Ray> {
        let x_offset = (px as f64 + x_offset) * self.pixel_size;
        let y_offset = (py as f64 + y_offset) * self.pixel_size;

        let world_x = self.half_width - x_offset;
        let world_y = self.half_height - y_offset;

        if let Some(inv) = self.transform.inverse() {
            let pixel = inv * Point(world_x, world_y, -1.0);
            let origin = inv * Point(0.0, 0.0, 0.0);
            let direction = (pixel - origin).normalize();

            Some(Ray::new(origin, direction))
        } else {
            None
        }
    }

    /// Uses the camera to render an image of the given world with specified recursion depth (for
    /// drawing reflections). This method can fail in whichever fashion any other parallelized
    /// function can. Also because I'm unwrapping a lot.
    pub fn render(&self, world: &World, depth: usize) -> Result<Canvas, RenderError> {
        let image = Arc::new(Mutex::new(Canvas::new(
            self.hsize as u32,
            self.vsize as u32,
        )));

        (0..self.vsize)
            .into_par_iter()
            .map(|y| {
                (0..self.hsize)
                    .into_par_iter()
                    .map(|x| match self.aa.level {
                        // No anti-aliasing (default), so we define a ray through the current pixel
                        // using the default offsets. Uses `World::color_at` to set the color of
                        // the pixel.
                        0 => {
                            if let Some(r) = self.ray_for_pixel(x, y, 0.5, 0.5) {
                                image.lock().unwrap().write_pixel(
                                    x as u32,
                                    y as u32,
                                    world.color_at(r, depth),
                                )
                            }
                        }
                        // For any anti-aliasing level > 0, we use the `Camera::color_at` method to
                        // set the color of the current pixel.
                        _ => {
                            let color = self.color_at(x, y, world, depth);
                            image.lock().unwrap().write_pixel(x as u32, y as u32, color);
                        }
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        let lock = Arc::try_unwrap(image).expect("lock has multiple owners, cannot unwrap");
        let canv = lock
            .into_inner()
            .expect("mutex is poisoned and cannot be locked");

        Ok(canv)
    }

    /// Sets the transformation matrix for the camera.
    pub fn with_transform(mut self, m: &Matrix<4>) -> Self {
        self.transform = *m;
        self
    }

    /// Sets the anti-aliasing level. __Note: a large number here slows the renderer down
    /// considerably.__ Use/adjust it as needed.
    pub fn with_antialiasing(mut self, level: usize) -> Self {
        self.aa.level = level;
        self
    }

    /// Sets the anti-aliasing method. Currently the two available
    /// [methods](crate::core::antialias::AAMethod) are stochastic and a multisampling-based
    /// method.
    pub fn with_aa_method(mut self, method: AAMethod) -> Self {
        self.aa.method = method;
        self
    }

    /// Uses the specified method to perform anti-aliasing.
    fn color_at(&self, x: usize, y: usize, world: &World, world_depth: usize) -> Color {
        self.aa.anti_alias(x, y, world, world_depth, &self)
    }

    /// For initializing private fields.
    fn set_private_fields(hsize: usize, vsize: usize, fov: f64) -> (f64, f64, f64) {
        let half_view = f64::tan(fov / 2.0);
        let aspect = hsize as f64 / vsize as f64;
        let half_width;
        let half_height;

        if aspect >= 1.0 {
            half_width = half_view;
            half_height = half_view / aspect;
        } else {
            half_width = half_view * aspect;
            half_height = half_view;
        }

        ((half_width * 2.0) / hsize as f64, half_width, half_height)
    }
}

#[cfg(test)]
mod camera_tests {
    use std::f64::consts::{FRAC_1_SQRT_2, PI};

    use super::*;
    use crate::{assert_vpeq, math::Vec3};

    const EPS: f64 = 1e-4;

    #[test]
    fn constructing_ray_with_transformed_camera() {
        let t =
            Matrix::rotation(crate::math::Axis::Y, PI / 4.0) * Matrix::translation(0.0, -2.0, 5.0);
        let c = Camera::new(201, 101, PI / 2.0).with_transform(&t);
        let r = c.ray_for_pixel(100, 50, 0.5, 0.5);

        assert_eq!(r.unwrap().origin, Point(0.0, 2.0, -5.0));
        assert_vpeq!(
            r.unwrap().direction,
            Vec3(FRAC_1_SQRT_2, 0.0, -FRAC_1_SQRT_2),
            EPS
        );
    }

    #[test]
    fn ray_through_corner_of_canvas() {
        let c = Camera::new(201, 101, PI / 2.0);
        let r = c.ray_for_pixel(0, 0, 0.5, 0.5);

        assert_eq!(r.unwrap().origin, Point(0.0, 0.0, 0.0));
        assert_vpeq!(r.unwrap().direction, Vec3(0.66519, 0.33259, -0.66851), EPS);
    }

    #[test]
    fn ray_through_center_of_canvas() {
        let c = Camera::new(201, 101, PI / 2.0);
        let r = c.ray_for_pixel(100, 50, 0.5, 0.5);

        assert_eq!(r.unwrap().origin, Point(0.0, 0.0, 0.0));
        assert_vpeq!(r.unwrap().direction, Vec3(0.0, 0.0, -1.0), EPS);
    }

    #[test]
    fn pixel_size_for_vertical_canvas() {
        let c = Camera::new(125, 200, PI / 2.0);

        assert!((c.pixel_size - 0.01).abs() < 1e-4);
    }

    #[test]
    fn pixel_size_for_horizontal_canvas() {
        let c = Camera::new(200, 125, PI / 2.0);

        assert!((c.pixel_size - 0.01).abs() < 1e-4);
    }

    #[test]
    fn can_set_transforms() {
        let t = Matrix::scaling(1.0, 1.0, 1.0);
        let c = Camera::new(200, 125, PI / 2.0).with_transform(&t);

        assert_eq!(c.transform, t);
    }

    #[test]
    fn constructing_a_camera() {
        let hsize = 160;
        let vsize = 120;
        let fov = PI / 2.0;
        let c = Camera::new(hsize, vsize, fov);

        assert_eq!(c.hsize, 160);
        assert_eq!(c.vsize, 120);
        assert_eq!(c.fov, PI / 2.0);
        assert_eq!(c.transform, Matrix::identity());
    }
}
