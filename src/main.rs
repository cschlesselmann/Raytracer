use crate::algebra::vector::Tuple;
use crate::scene::environment::{tick, Environment};
use crate::scene::projectile::Projectile;
use log::info;
use simple_logger::SimpleLogger;

mod algebra;
mod scene;

fn main() {
    SimpleLogger::new().init().unwrap();

    info!("Initialising scene");
    let mut p = Projectile {
        position: Tuple::new_point(0, 1, 0),
        velocity: Tuple::new_vector(1, 1, 0).normalize(),
    };
    let e = Environment {
        gravity: Tuple::new_vector(0, -0.1, 0),
        wind: Tuple::new_vector(-0.01, 0, 0),
    };

    let mut ticks = 0;
    while p.position.y > 0.0 {
        info!("Position before tick {}: {:?}", ticks, p.position);
        p = tick(&p, &e);
        info!("Position after tick {}: {:?}", ticks, p.position);
        ticks += 1;
    }

    info!("Took {} ticks to reach the ground", ticks)
}
