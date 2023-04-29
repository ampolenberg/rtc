#![allow(dead_code)]

use rtc::{
    math::{Point, Tuple, Vec3},
    visuals::{canvas::Canvas, Color},
};

#[derive(Copy, Clone, Debug)]
struct Projectile {
    position: Point,
    velocity: Vec3,
}

#[derive(Copy, Clone, Debug)]
struct Environment {
    gravity: Vec3,
    wind: Vec3,
}

fn tick(env: Environment, proj: Projectile) -> Projectile {
    Projectile {
        position: proj.position + proj.velocity,
        velocity: proj.velocity + env.gravity + env.wind,
    }
}

fn main() {
    let position = Point(0.0, 1.0, 0.0);
    let velocity = Vec3(1.0, 1.8, 0.0).normalize() * 11.25;
    let gravity = Vec3(0.0, -0.1, 0.0);
    let wind = Vec3(-0.01, 0.0, 0.0);

    let mut p = Projectile { position, velocity };
    let e = Environment { gravity, wind };

    let mut canvas = Canvas::new(900, 550);

    while p.position.y() > 0.0 {
        canvas.write_pixel(
            p.position.x().round() as u32,
            550 - p.position.y().round() as u32,
            Color::red(),
        );
        p = tick(e, p);
    }

    canvas.export("img/cannon.png").unwrap();
}

#[cfg(test)]
mod cannon_tests {
    use rtc::math::Tuple;

    use super::*;

    #[test]
    #[ignore]
    fn projectiles_update() {
        // velocity normalized to 1 unit/tick.
        let mut p = Projectile {
            position: Point(0.0, 1.0, 0.0),
            velocity: Vec3(1.0, 1.0, 0.0).normalize(),
        };

        // gravity = -0.1 unit/tick and wind = -0.01 unit/tick.
        let e = Environment {
            gravity: Vec3(0.0, -0.1, 0.0),
            wind: Vec3(-0.01, 0.0, 0.0),
        };

        while p.position.y() > 0.0 {
            println!("{}", p.position.y());
            p = tick(e, p);
        }
    }
}
