use crate::algebra::vector::Tuple;
use crate::scene::projectile::Projectile;

pub struct Environment {
    pub gravity: Tuple,
    pub wind: Tuple,
}

pub fn tick(projectile: &Projectile, environment: &Environment) -> Projectile {
    Projectile {
        position: &projectile.position + &projectile.velocity,
        velocity: &projectile.velocity + &environment.gravity + &environment.wind,
    }
}
