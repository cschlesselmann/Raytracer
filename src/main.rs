use crate::algebra::vector::{Point, Vector3};
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
        position: Point::new(0, 1, 0),
        velocity: Vector3::new(1, 1, 0).normalize(),
    };
    let e = Environment {
        gravity: Vector3::new(0, -0.1, 0),
        wind: Vector3::new(-0.01, 0, 0),
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
