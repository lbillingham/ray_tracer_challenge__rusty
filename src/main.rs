#[macro_use]
extern crate approx;

pub mod canvas;
pub mod color;
pub mod f64_helpers;
pub mod tuple;

#[derive(Debug, Copy, Clone)]
struct Projectile {
    pub position: tuple::Point,
    pub velocity: tuple::Vector,
}

#[derive(Debug, Copy, Clone)]
struct Environment {
    pub gravity: tuple::Vector,
    pub wind: tuple::Vector,
}

fn tick(env: Environment, proj: Projectile) -> Projectile {
    let pos = proj.position + proj.velocity;
    let vel = proj.velocity + env.gravity + env.wind;
    Projectile {
        position: pos,
        velocity: vel,
    }
}

fn main() {
    let mut p = Projectile {
        position: tuple::point(0., 1., 0.),
        velocity: tuple::vector(1., 1., 0.).normalize(),
    };
    let e = Environment {
        gravity: tuple::vector(0., -0.1, 0.),
        wind: tuple::vector(-0.01, 0., 0.),
    };
    while p.position.y > 0. && p.position.y < 5.0 {
        println!(
            "Position {:.3}, {:.3}, {:.3}",
            p.position.x, p.position.y, p.position.z
        );
        println!(
            "Velocity {:.3}, {:.3}, {:.3}",
            p.velocity.x, p.velocity.y, p.velocity.z
        );
        println!("------------------------------");
        p = tick(e, p);
    }
}
