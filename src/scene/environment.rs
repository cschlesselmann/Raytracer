use crate::algebra::Vector3;
use crate::scene::projectile::Projectile;

pub struct Environment {
    pub gravity: Vector3,
    pub wind: Vector3,
}

pub fn tick(projectile: &Projectile, environment: &Environment) -> Projectile {
    Projectile {
        position: &projectile.position + &projectile.velocity,
        velocity: &projectile.velocity + &environment.gravity + &environment.wind,
    }
}
