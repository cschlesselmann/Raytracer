use auto_ops::*;

use crate::algebra::utils::equal_f32;

#[derive(Debug, PartialOrd)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl_op_ex!(+ |a: &Vector3, b: &Vector3| -> Vector3 { Vector3::new(a.x + b.x, a.y + b.y, a.z + b.z) });
impl_op_ex!(-|a: &Vector3, b: &Vector3| -> Vector3 { Vector3::new(a.x - b.x, a.y - b.y, a.z - b.z) });
impl_op_ex!(*|a: &Vector3, b: &f32| -> Vector3 { Vector3::new(a.x * b, a.y * b, a.z * b) });
impl_op_ex!(/ |a: &Vector3, b: &f32| -> Vector3 { Vector3::new(a.x / b, a.y / b, a.z / b) });
impl_op_ex!(-|a: &Vector3| -> Vector3 { Vector3::new(-a.x, -a.y, -a.z) });

#[derive(Debug, PartialOrd)]
pub struct Point {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl_op_ex!(+ |a: &Point, b: &Vector3| -> Point { Point::new(a.x + b.x, a.y + b.y, a.z + b.z) });
impl_op_ex!(-|a: &Point, b: &Vector3| -> Point { Point::new(a.x - b.x, a.y - b.y, a.z - b.z) });
impl_op_ex!(-|a: &Point, b: &Point| -> Vector3 { Vector3::new(a.x - b.x, a.y - b.y, a.z - b.z) });

impl PartialEq for Vector3 {
    fn eq(&self, other: &Self) -> bool {
        equal_f32(self.x, other.x) && equal_f32(self.y, other.y) && equal_f32(self.z, other.z)
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        equal_f32(self.x, other.x) && equal_f32(self.y, other.y) && equal_f32(self.z, other.z)
    }
}

impl Vector3 {
    pub fn new(x: impl Into<f64>, y: impl Into<f64>, z: impl Into<f64>) -> Vector3 {
        Vector3 {
            x: x.into() as f32,
            y: y.into() as f32,
            z: z.into() as f32,
        }
    }

    pub fn zero() -> Vector3 {
        Vector3::new(0, 0, 0)
    }

    pub fn magnitude(&self) -> f32 {
        f32::sqrt(self.x * self.x + self.y * self.y + self.z * self.z)
    }

    pub fn normalize(&self) -> Vector3 {
        self / self.magnitude()
    }

    pub fn dot(&self, rhs: &Vector3) -> f32 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    pub fn cross(&self, rhs: &Vector3) -> Vector3 {
        Vector3::new(
            self.y * rhs.z - self.z * rhs.y,
            self.z * rhs.x - self.x * rhs.z,
            self.x * rhs.y - self.y * rhs.x,
        )
    }
}

impl Point {
    pub fn new(x: impl Into<f64>, y: impl Into<f64>, z: impl Into<f64>) -> Point {
        Point {
            x: x.into() as f32,
            y: y.into() as f32,
            z: z.into() as f32,
        }
    }
}

#[cfg(test)]
mod tests {
    use ctor::ctor;
    use log::LevelFilter;
    use simple_logger::SimpleLogger;

    use crate::algebra::utils::equal_f32;
    use crate::algebra::vector::{Point, Vector3};

    #[ctor]
    fn foo() {
        SimpleLogger::new().with_level(LevelFilter::Trace).init().unwrap();
    }

    #[test]
    fn tuple_equal() {
        let tuple = Vector3::new(4, -4, 3);
        let other = Vector3::new(4, -4, 3);
        assert_eq!(tuple, other)
    }

    #[test]
    fn tuple_not_equal() {
        let tuple = Vector3::new(4, -4, 3);
        let other = Vector3::new(4, -5, 3);
        assert_ne!(tuple, other)
    }

    #[test]
    fn tuple_add() {
        let point = Point::new(3, -2, 5);
        let vec = Vector3::new(-2, 3, 1);
        assert_eq!(point + vec, Point::new(1, 1, 6))
    }

    #[test]
    fn tuple_sub_points() {
        let tuple = Point::new(3, 2, 1);
        let other = Point::new(5, 6, 7);

        assert_eq!(tuple - other, Vector3::new(-2, -4, -6))
    }

    #[test]
    fn tuple_sub_vec() {
        let tuple = Vector3::new(3, 2, 1);
        let other = Vector3::new(5, 6, 7);

        assert_eq!(tuple - other, Vector3::new(-2, -4, -6))
    }

    #[test]
    fn sub_vector_from_zero_vector() {
        assert_eq!(Vector3::zero() - Vector3::new(1, -2, 3), Vector3::new(-1, 2, -3))
    }

    #[test]
    fn negate_tuple() {
        assert_eq!(-Vector3::new(1, -2, 3), Vector3::new(-1, 2, -3))
    }

    #[test]
    fn mul_by_scalar() {
        let tuple = Vector3::new(1, -2, 3);
        assert_eq!(tuple * 3.5, Vector3::new(3.5, -7, 10.5))
    }

    #[test]
    fn mul_by_fraction() {
        let tuple = Vector3::new(1, -2, 3);
        assert_eq!(tuple * 0.5, Vector3::new(0.5, -1, 1.5))
    }

    #[test]
    fn compute_magnitude() {
        assert_eq!(Vector3::new(0, 1, 0).magnitude(), 1f32);
        assert_eq!(Vector3::new(0, 0, 1).magnitude(), 1f32);
        assert_eq!(Vector3::new(1, 2, 3).magnitude(), f32::sqrt(14f32));
        assert_eq!(Vector3::new(-1, -2, -3).magnitude(), f32::sqrt(14f32));
    }

    #[test]
    fn normalize() {
        assert_eq!(Vector3::new(4, 0, 0).normalize(), Vector3::new(1, 0, 0));
        assert_eq!(
            Vector3::new(1, 2, 3).normalize(),
            Vector3::new(
                1f32 / f32::sqrt(14f32),
                2f32 / f32::sqrt(14f32),
                3f32 / f32::sqrt(14f32)
            )
        );
    }

    #[test]
    fn magnitude_of_normalized_vec() {
        assert!(equal_f32(Vector3::new(1, 2, 3).normalize().magnitude(), 1.0))
    }

    #[test]
    fn dot_product() {
        let a = Vector3::new(1, 2, 3);
        let b = Vector3::new(2, 3, 4);
        assert_eq!(a.dot(&b), 20f32)
    }

    #[test]
    fn cross_product() {
        let a = Vector3::new(1, 2, 3);
        let b = Vector3::new(2, 3, 4);
        assert_eq!(a.cross(&b), Vector3::new(-1, 2, -1));
        assert_eq!(b.cross(&a), Vector3::new(1, -2, 1));
    }
}
