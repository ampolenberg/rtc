use std::f64::consts::PI;

use rtc::{
    core::{camera::Camera, light::Light, material::Material, world::World},
    math::{matrix::Axis, Matrix, Point, Vec3},
    shape::Sphere,
    visuals::Color,
};

fn main() -> image::ImageResult<()> {
    let floor_mat = Material::default()
        .with_color(&Color(1.0, 0.9, 0.9))
        .with_specular(0.0);

    let floor = Sphere::default()
        .with_transform(&Matrix::scaling(10.0, 0.01, 10.0))
        .with_material(&floor_mat)
        .as_shape();

    let left_wall = Sphere::default()
        .with_transform(
            &(Matrix::translation(0.0, 0.0, 5.0)
                * Matrix::rotation(Axis::Y, -PI / 4.0)
                * Matrix::rotation(Axis::X, PI / 2.0)
                * Matrix::scaling(10.0, 0.01, 10.0)),
        )
        .with_material(&floor_mat)
        .as_shape();

    let right_wall = Sphere::default()
        .with_transform(
            &(Matrix::translation(0.0, 0.0, 5.0)
                * Matrix::rotation(Axis::Y, PI / 4.0)
                * Matrix::rotation(Axis::X, PI / 2.0)
                * Matrix::scaling(10.0, 0.01, 10.0)),
        )
        .with_material(&floor_mat)
        .as_shape();

    let middle_sphere = Sphere::default()
        .with_transform(&Matrix::translation(-0.5, 1.0, 0.5))
        .with_material(
            &Material::default()
                .with_color(&Color(0.1, 1.0, 0.5))
                .with_diffuse(0.7)
                .with_specular(0.3),
        )
        .as_shape();

    let right_sphere = Sphere::default()
        .with_transform(&(Matrix::translation(1.5, 0.5, -0.5) * Matrix::scaling(0.5, 0.5, 0.5)))
        .with_material(
            &Material::default()
                .with_color(&Color(0.1, 1.0, 0.5))
                .with_diffuse(0.7)
                .with_specular(0.3),
        )
        .as_shape();

    let left_sphere = Sphere::default()
        .with_transform(
            &(Matrix::translation(-1.5, 0.33, -0.75) * Matrix::scaling(0.33, 0.33, 0.33)),
        )
        .with_material(
            &Material::default()
                .with_color(&Color(1.0, 0.8, 0.1))
                .with_diffuse(0.7)
                .with_specular(0.3),
        )
        .as_shape();

    let light_source = Light::new_point_light(Point(-10.0, 10.0, -10.0), Color::white());

    let world = World {
        objects: vec![
            floor,
            left_wall,
            right_wall,
            left_sphere,
            middle_sphere,
            right_sphere,
        ],
        lights: vec![light_source],
    };

    let cam = Camera::new(800, 750, PI / 3.0)
        .with_transform(&Matrix::view_transform(
            Point(0.0, 1.5, -5.0),
            Point(0.0, 1.0, 0.0),
            Vec3(0.0, 1.0, 0.0),
        ))
        .with_antialiasing(20);

    let canvas = cam.render(&world, 5).unwrap();
    canvas.export("img/chapter7_aa.png")
}
