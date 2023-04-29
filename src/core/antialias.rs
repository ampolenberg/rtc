//! I want to rewrite this. As it is now, the order of method calls matters:
//!
//! # Example
//! ```ignore
//! let aa = AntiAliasing::default();
//! // This:
//! aa
//!     .with_method(AAMethod::Multisampling(
//!         Multisampling::default()
//!     ))
//!     .with_tolerance(etol)
//! // is not equivalent to this:
//! aa
//!     .with_tolerance(etol)
//!     .with_method(AAMethod::Multisampling(
//!         Multisampling::default()
//!     ))
//! ```
//!
//! It's probably a better idea to:
//! * make `AAMethod` into a trait (probably called `AntiAliasing`)
//! * remove the `AntiAliasing` struct
//! * just have `Stochastic` and `Multisampling` implement the new `AAMethod`.
//!
//! This would require refactoring in the yaml parser and (probably) `Camera` too, though.
use super::{Camera, World};
use crate::visuals::Color;
use rand::{distributions::Uniform, prelude::*};

pub enum AAMethod {
    Stochastic(Stochastic),
    Multisampling(Multisampling),
}

/// Holds the information needed to apply the antialiasing.
pub struct AntiAliasing {
    pub method: AAMethod,
    pub level: usize,
    pub error_tolerance: f64,
}

impl AntiAliasing {
    /// Does the actual antialiasing using an [AAMethod](crate::core::antialias::AAMethod). At the
    /// moment, only [Stochastic](crate::core::antialias::Stochastic) and
    /// [Multisampling](crate::core::antialias::Stochastic) are available.
    pub fn anti_alias(
        &self,
        px: usize,
        py: usize,
        world: &World,
        world_depth: usize,
        cam: &Camera,
    ) -> Color {
        match self.method {
            AAMethod::Stochastic(ref s) => s.anti_alias(px, py, world, world_depth, cam),
            AAMethod::Multisampling(ref m) => m.anti_alias(px, py, world, world_depth, cam),
        }
    }

    pub fn with_method(mut self, aa_method: AAMethod) -> Self {
        self.method = aa_method;
        self
    }

    pub fn with_level(mut self, aa_level: usize) -> Self {
        self.level = aa_level;
        self.set_method_level(aa_level);
        self
    }

    pub fn with_tolerance(mut self, etol: f64) -> Self {
        self.error_tolerance = etol;
        self.set_method_tolerance(etol);
        self
    }

    fn set_method_tolerance(&mut self, etol: f64) {
        match self.method {
            AAMethod::Multisampling(ref mut m) => m.error_tolerance = etol,
            _ => return,
        }
    }

    fn set_method_level(&mut self, aa_level: usize) {
        match self.method {
            AAMethod::Stochastic(ref mut s) => s.level = aa_level,
            AAMethod::Multisampling(ref mut m) => m.level = aa_level,
        }
    }
}

#[derive(Clone)]
pub struct Stochastic {
    level: usize,
}

impl Stochastic {
    fn anti_alias(
        &self,
        px: usize,
        py: usize,
        world: &World,
        world_depth: usize,
        cam: &Camera,
    ) -> Color {
        let mut color = Color::black();
        let mut rng = thread_rng();
        let uniform = Uniform::new(0.0, 1.0);

        for _ in 0..self.level {
            let xoffset = uniform.sample(&mut rng);
            let yoffset = uniform.sample(&mut rng);

            if let Some(ray) = cam.ray_for_pixel(px, py, xoffset, yoffset) {
                color = color + world.color_at(ray, world_depth)
            }
        }

        color / self.level as f64
    }
}

#[derive(Clone)]
pub struct Multisampling {
    level: usize,
    error_tolerance: f64,
}

impl Multisampling {
    fn anti_alias(
        &self,
        px: usize,
        py: usize,
        world: &World,
        world_depth: usize,
        cam: &Camera,
    ) -> Color {
        let mut rng = thread_rng();
        let uniform = Uniform::new(0.0, 1.0);

        let color = Color::black();
        let mut color_squared_sum = color;
        let mut color_sum = color;
        let mut n = 0.0;

        while n < self.level as f64 {
            let xoffset = uniform.sample(&mut rng);
            let yoffset = uniform.sample(&mut rng);

            if let Some(ray) = cam.ray_for_pixel(px, py, xoffset, yoffset) {
                let color = world.color_at(ray, world_depth);
                color_sum = color_sum + color;
                color_squared_sum = color_squared_sum + color * color;
            }

            n += 1.0;
        }

        while self.color_mean_variance(n, color_squared_sum, color_sum)
            > self.error_tolerance * self.error_tolerance
        {
            let xoffset = uniform.sample(&mut rng);
            let yoffset = uniform.sample(&mut rng);

            if let Some(ray) = cam.ray_for_pixel(px, py, xoffset, yoffset) {
                let color = world.color_at(ray, world_depth);
                color_sum = color_sum + color;
                color_squared_sum = color_squared_sum + color * color;
                n += 1.0;
            }
        }

        color_sum / n
    }

    fn color_mean_variance(&self, n: f64, sum_of_squares: Color, sum: Color) -> f64 {
        let color_mean = sum / n;
        let color_var = sum_of_squares / n - color_mean * color_mean;
        let total_var = color_var.r() + color_var.g() + color_var.b();

        total_var / n
    }
}

impl Default for AntiAliasing {
    fn default() -> Self {
        Self {
            method: AAMethod::Stochastic(Stochastic::default()),
            error_tolerance: 1.0,
            level: 0,
        }
    }
}

impl Default for Stochastic {
    fn default() -> Self {
        Self { level: 5 }
    }
}

impl Default for Multisampling {
    fn default() -> Self {
        Self {
            level: 5,
            error_tolerance: 1.0,
        }
    }
}
