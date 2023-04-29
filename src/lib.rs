//! A Rust implementation of the book: "The Ray Tracer Challenge", by Jamis Buck.
//!
//! # Example usage
//! Use the API directly, or write a world in a YAML file and use the builtin parser to render it.
//! ```text
//! ---
//! - add: camera
//!   hsize: 800
//!   vsize: 750
//!   fov: 1.0471975512
//!   from: [0.0, 1.5, -5.0]
//!   to: [0.0, 1.0, 0.0]
//!   up: [0.0, 1.0, 0.0]
//!   aa: 5
//!
//! - add: light
//!   type: point
//!   at: [-10.0, 10.0, -10.0]
//!   intensity: [1.0, 1.0, 1.0]
//!
//! - add: plane
//!   material:
//!     pattern:
//!       type: striped
//!       colors:
//!         - [1.0, 1.0, 1.0]
//!         - [0.0, 0.0, 0.0]
//!
//! - add: sphere
//!   transform:
//!     - [translate, -0.5, 1.0, 0.5]
//!   material:
//!     color: [0.1, 1.0, 0.5]
//!     diffuse: 0.7
//!     specular: 0.3
//! ```
//!
//! And to render this YAML to a file:
//!
//! ```no_run
//! use rtc::{core::world::World, io::yaml::parse_yaml};
//!
//! let (cam, world) = parse_yaml("world.yml").unwrap();
//!
//! let canvas = cam.unwrap().render(&world).unwrap();
//! canvas.export("render.png").unwrap();
//! ```

pub mod core;

pub mod io;

pub mod math;

pub mod shape;

pub mod visuals;
