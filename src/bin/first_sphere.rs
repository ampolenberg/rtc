use std::sync::{Arc, Mutex};

use rayon::iter::{IntoParallelIterator, ParallelIterator};
use rtc::{
    core::{light::Light, material::Material, ray::Ray, Intersectable},
    math::Point,
    shape::{Shape, Sphere},
    visuals::{canvas::Canvas, Color},
};

fn main() -> image::ImageResult<()> {
    let canvas_pixels = 500;
    let canvas = Arc::new(Mutex::new(Canvas::new(canvas_pixels, canvas_pixels)));

    let wall_z = 10.0;
    let wall_size = 20.0;
    let pixel_size = wall_size / canvas_pixels as f64;
    let half = wall_size / 2.0;

    let ray_origin = Point(0.0, 0.0, -5.0);
    let sphere_mat = Material::default().with_color(&Color(1.0, 0.2, 1.0));
    let sphere = Sphere {
        material: sphere_mat,
        transform: rtc::math::Matrix::scaling(2.5, 2.5, 2.5),
    };

    let light_pos = Point(-10.0, 10.0, -10.0);
    let light_col = Color::white();
    let light = Light::new_point_light(light_pos, light_col);

    // rows of pixels
    (0..canvas_pixels - 1)
        .into_par_iter()
        .map(|y| {
            // world y coordinate (top = half, bottom = -half)
            let world_y = half - pixel_size * y as f64;
            // each pixel in the row
            (0..canvas_pixels - 1)
                .into_par_iter()
                .map(|x| {
                    // world x coordinate (left = -half, right = half)
                    let world_x = -half + pixel_size * x as f64;

                    // the point to be targeted by the ray
                    let position = Point(world_x, world_y, wall_z);
                    let r = Ray::new(ray_origin, (position - ray_origin).normalize());
                    let xs = sphere.as_shape().intersect(r);

                    if let Some(mut intersections) = xs {
                        if let Some(hit) = intersections.hit() {
                            let hit_point = r.position(hit.t);
                            let normal = hit.object.normal_at(hit_point).unwrap();
                            let eye = -r.direction;
                            let color = hit.object.clone();
                            let color = get_material(&color).lighting(
                                &sphere.as_shape(),
                                &light,
                                &hit_point,
                                &eye,
                                &normal,
                                false,
                            );
                            canvas.lock().unwrap().write_pixel(x, y, color);
                        }
                    }
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let x = canvas.lock().unwrap().export("img/first_sphere.png");

    x
}

fn get_material(shape: &Shape) -> Material {
    match *shape {
        Shape::Sphere(ref sphere) => sphere.material.clone(),
        Shape::Plane(ref plane) => plane.material.clone(),
    }
}
