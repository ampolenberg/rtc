use std::f64::consts::PI;

use rtc::{
    math::{matrix::Axis, Matrix, Point, Tuple, Vec3},
    visuals::{canvas::Canvas, Color},
};

fn main() -> image::ImageResult<()> {
    let mut canvas = Canvas::new(400, 400);

    let center = Vec3(200.0, 0.0, 200.0);
    let twelve = Point(0.0, 0.0, 1.0);
    let clock_radius = canvas.width * 3 / 8;

    for i in 0..12 {
        let r = Matrix::rotation(Axis::Y, i as f64 * PI / 6.0);
        let next_dot = (r * twelve) * clock_radius as f64 + center;
        canvas.write_pixel(next_dot.x() as u32, next_dot.z() as u32, Color::white());
    }

    canvas.export("img/clock.png")
}
