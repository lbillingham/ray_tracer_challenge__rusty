#[macro_use]
extern crate approx;

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

#[cfg(test)]
mod tests_for_main {

    use super::*;

    #[test]
    fn it_can_simulate_a_dumb_gravity_drop_1_tic() {
        let pos_initial = tuple::point(0.,0.,0.);
        let vel_initial = tuple::vector(0., 0., 0.);


        let mut p = Projectile {
            position: tuple::point(pos_initial.x, pos_initial.y, pos_initial.z),
            velocity: tuple::vector(vel_initial.x, vel_initial.y, vel_initial.z).normalize(),
        };
        let e = Environment {
            gravity: tuple::vector(0., -0.1, 0.),
            wind: tuple::vector(0., 0., 0.),
        };
        assert!(p.position.is_point());
        println!(
            "p.velocity.magnitude {:.3}", p.velocity.magnitude()
        );
        assert!(p.velocity.magnitude() == 0.);
         
        assert!(e.gravity.is_vector());

        p = tick(e, p);
        assert!(p.velocity.magnitude() > 0.);
        assert!(p.position == pos_initial); 
        assert!(p.velocity != vel_initial); 
        assert!(p.velocity.y == -0.1);
        assert!(p.velocity.x == vel_initial.x);
        assert!(p.velocity.x == vel_initial.x);
    }

    #[test]
    fn it_can_simulate_a_dumb_gravity_drop_2_tics() {
        let pos_initial = tuple::point(0.,0.,0.);
        let vel_initial = tuple::vector(0., 0., 0.);


        let mut p = Projectile {
            position: tuple::point(pos_initial.x, pos_initial.y, pos_initial.z),
            velocity: tuple::vector(vel_initial.x, vel_initial.y, vel_initial.z).normalize(),
        };
        let e = Environment {
            gravity: tuple::vector(0., -0.1, 0.),
            wind: tuple::vector(0., 0., 0.),
        };
        assert!(p.position.is_point());
        println!(
            "p.velocity.magnitude {:.3}", p.velocity.magnitude()
        );
        p = tick(e, p);
        p = tick(e, p);
        assert!(p.position != pos_initial);
        assert!(p.position.y == -0.1);
        assert!(p.velocity.y == -0.2);

        assert!(p.velocity.x == vel_initial.x);
        assert!(p.velocity.z == vel_initial.z);
        assert!(p.position.x == pos_initial.x);
        assert!(p.position.z == pos_initial.z);
    }

    #[test]
    fn it_can_simulate_wind_with_no_gravity_1_tic() {
        let pos_initial = tuple::point(0., 0., 0.);
        let vel_initial = tuple::vector(0., 0., 0.);

        let mut p = Projectile {
            position: tuple::point(pos_initial.x, pos_initial.y, pos_initial.z),
            velocity: tuple::vector(vel_initial.x, vel_initial.y, vel_initial.z).normalize(),
        };
        let e = Environment {
            gravity: tuple::vector(0., 0., 0.),
            wind: tuple::vector(0.1, 0., 0.2),
        };
 
        assert!(p.velocity.magnitude() == 0.);

        p = tick(e, p);
        assert!(p.velocity.magnitude() > 0.);
        assert!(p.position == pos_initial);
        assert!(p.velocity != vel_initial);
        assert!(p.velocity.x == 0.1);
        assert!(p.velocity.y == vel_initial.y);
        assert!(p.velocity.z == 0.2);
    }

    #[test]
    fn it_can_simulate_wind_with_no_gravity_2_tics() {
        let pos_initial = tuple::point(0., 0., 0.);
        let vel_initial = tuple::vector(0., 0., 0.);

        let mut p = Projectile {
            position: tuple::point(pos_initial.x, pos_initial.y, pos_initial.z),
            velocity: tuple::vector(vel_initial.x, vel_initial.y, vel_initial.z).normalize(),
        };
        let e = Environment {
            gravity: tuple::vector(0., 0., 0.),
            wind: tuple::vector(0.1, 0., 0.2),
        };

        p = tick(e, p);
        p = tick(e, p);
        assert!(p.position != pos_initial);
        assert!(p.position.x == 0.1);
        assert!(p.position.y == pos_initial.y);
        assert!(p.position.z == 0.2);
        assert!(p.velocity.x == 0.2);
        assert!(p.velocity.y == vel_initial.y);
        assert!(p.velocity.z == 0.4);
    }

    #[test] 
    fn it_can_simulate_stasis() {
        let pos_initial = tuple::point(0., 0., 0.);
        let vel_initial = tuple::vector(0., 0., 0.);

        let mut p = Projectile {
            position: tuple::point(pos_initial.x, pos_initial.y, pos_initial.z),
            velocity: tuple::vector(vel_initial.x, vel_initial.y, vel_initial.z).normalize(),
        };
        let e = Environment {
            gravity: tuple::vector(0., 0., 0.),
            wind: tuple::vector(0., 0., 0.),
        };

        p = tick(e, p);
        p = tick(e, p);
        p = tick(e, p);
        p = tick(e, p);
        p = tick(e, p);
        p = tick(e, p);
        p = tick(e, p);

        assert!(p.position == pos_initial);
        assert!(p.velocity == vel_initial);

    }

    #[test] 
    fn it_can_simulate_newtons_1st_law() {
        let pos_initial = tuple::point(0., 0., 0.);
        let vel_initial = tuple::vector(1., 10., -100.);

        let mut p = Projectile {
            position: tuple::point(pos_initial.x, pos_initial.y, pos_initial.z),
            velocity: tuple::vector(vel_initial.x, vel_initial.y, vel_initial.z),
        };
        let e = Environment {
            gravity: tuple::vector(0., 0., 0.),
            wind: tuple::vector(0., 0., 0.),
        };

        p = tick(e, p);
        assert!(p.velocity == vel_initial);
        assert!(p.position != pos_initial);
        assert!(p.position == tuple::point(1., 10., -100.));

        p = tick(e, p);
        p = tick(e, p);
        assert!(p.velocity == vel_initial);
        assert!(p.position == tuple::point(3., 30., -300.));

    }
}


fn main() {
    let mut p = Projectile {
        position: tuple::point(0., 1., 0.),
        velocity: tuple::vector(0., 0., 0.).normalize(),
    };
    let e = Environment {
        gravity: tuple::vector(0., -0.1, 0.),
        wind: tuple::vector(0., 0., 0.),
    };
    let mut tic_count = 0;
    while p.position.y > 0. && p.position.y < 5.0 {
        println!(
            "tic number {:}", tic_count
        );
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
        tic_count += 1;
    }
}
