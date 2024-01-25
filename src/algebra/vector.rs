use auto_ops::*;
use log::error;

use crate::algebra::utils::equal_f32;
use crate::algebra::vector::TupleType::{Invalid, Point, Vector};

#[derive(Debug, PartialOrd)]
pub struct Tuple {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

#[derive(Debug, PartialOrd, PartialEq)]
pub enum TupleType {
    Vector,
    Point,
    Invalid,
}

impl_op_ex!(+ |a: &Tuple, b: &Tuple| -> Tuple {Tuple::new(a.x + b.x, a.y + b.y, a.z + b.z, a.w + b.w)});
impl_op_ex!(-|a: &Tuple, b: &Tuple| -> Tuple { Tuple::new(a.x - b.x, a.y - b.y, a.z - b.z, a.w - b.w) });
impl_op_ex!(*|a: &Tuple, b: &f32| -> Tuple { Tuple::new(a.x * b, a.y * b, a.z * b, a.w * b) });
impl_op_ex!(/ |a: &Tuple, b: &f32| -> Tuple {Tuple::new(a.x / b, a.y / b, a.z / b, a.w / b)});
impl_op_ex!(-|a: &Tuple| -> Tuple { Tuple::new(-a.x, -a.y, -a.z, -a.w) });

impl PartialEq for Tuple {
    fn eq(&self, other: &Self) -> bool {
        equal_f32(self.x, other.x)
            && equal_f32(self.y, other.y)
            && equal_f32(self.z, other.z)
            && self.kind() == other.kind()
    }
}

impl Tuple {
    pub fn new(x: impl Into<f64>, y: impl Into<f64>, z: impl Into<f64>, w: impl Into<f64>) -> Tuple {
        Tuple {
            x: x.into() as f32,
            y: y.into() as f32,
            z: z.into() as f32,
            w: w.into() as f32,
        }
    }

    pub fn zero() -> Tuple {
        Tuple::new(0, 0, 0, 0)
    }

    pub fn new_vector(x: impl Into<f64>, y: impl Into<f64>, z: impl Into<f64>) -> Tuple {
        Tuple {
            x: x.into() as f32,
            y: y.into() as f32,
            z: z.into() as f32,
            w: 0f32,
        }
    }

    pub fn new_point(x: impl Into<f64>, y: impl Into<f64>, z: impl Into<f64>) -> Tuple {
        Tuple {
            x: x.into() as f32,
            y: y.into() as f32,
            z: z.into() as f32,
            w: 1f32,
        }
    }

    pub fn kind(&self) -> TupleType {
        if equal_f32(self.w, 0.0) {
            Vector
        } else if equal_f32(self.w, 1.0) {
            Point
        } else {
            Invalid
        }
    }

    pub fn magnitude(&self) -> f32 {
        f32::sqrt(self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w)
    }

    pub fn normalize(&self) -> Tuple {
        self / self.magnitude()
    }

    pub fn dot(&self, rhs: &Tuple) -> f32 {
        if cfg!(debug_assertions) {
            if self.kind() != Vector || rhs.kind() != Vector {
                error!(
                    "Called dot product on vector of kind {:?}/{:?}",
                    self.kind(),
                    rhs.kind()
                )
            }
        }
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z + self.w * rhs.w
    }

    pub fn cross(&self, rhs: &Tuple) -> Tuple {
        Tuple::new_vector(
            self.y * rhs.z - self.z * rhs.y,
            self.z * rhs.x - self.x * rhs.z,
            self.x * rhs.y - self.y * rhs.x,
        )
    }
}

#[cfg(test)]
mod tests {
    use ctor::ctor;
    use log::LevelFilter;
    use simple_logger::SimpleLogger;

    use crate::algebra::utils::equal_f32;
    use crate::algebra::vector::{Tuple, TupleType};

    #[ctor]
    fn foo() {
        SimpleLogger::new().with_level(LevelFilter::Trace).init().unwrap();
    }

    #[test]
    fn tuple_with_w_1_is_a_point() {
        let tuple = Tuple::new(4.3, -4.2, 3.1, 1.0);
        assert_eq!(tuple.kind(), TupleType::Point);
    }

    #[test]
    fn tuple_with_w_0_is_a_vector() {
        let tuple = Tuple::new(4.3, -4.2, 3.1, 0.0);
        assert_eq!(tuple.kind(), TupleType::Vector);
    }

    #[test]
    fn point_creates_tuple_with_w_1() {
        let tuple = Tuple::new_point(4, -4, 3);
        assert_eq!(tuple.w, 1f32)
    }

    #[test]
    fn vector_creates_tuple_with_w_0() {
        let tuple = Tuple::new_vector(4, -4, 3);
        assert_eq!(tuple.w, 0f32)
    }

    #[test]
    fn tuple_equal() {
        let tuple = Tuple::new_vector(4, -4, 3);
        let other = Tuple::new_vector(4, -4, 3);
        assert_eq!(tuple, other)
    }

    #[test]
    fn tuple_not_equal() {
        let tuple = Tuple::new_vector(4, -4, 3);
        let other = Tuple::new_vector(4, -5, 3);
        assert_ne!(tuple, other)
    }

    #[test]
    fn tuple_add() {
        let tuple = Tuple::new(3, -2, 5, 1);
        let other = Tuple::new(-2, 3, 1, 0);
        assert_eq!(tuple + other, Tuple::new(1, 1, 6, 1))
    }

    #[test]
    fn tuple_sub_points() {
        let tuple = Tuple::new_point(3, 2, 1);
        let other = Tuple::new_point(5, 6, 7);

        assert_eq!(tuple - other, Tuple::new_vector(-2, -4, -6))
    }

    #[test]
    fn tuple_sub_vec() {
        let tuple = Tuple::new_vector(3, 2, 1);
        let other = Tuple::new_vector(5, 6, 7);

        assert_eq!(tuple - other, Tuple::new_vector(-2, -4, -6))
    }

    #[test]
    fn sub_vector_from_zero_vector() {
        assert_eq!(
            Tuple::zero() - Tuple::new_vector(1, -2, 3),
            Tuple::new_vector(-1, 2, -3)
        )
    }

    #[test]
    fn negate_tuple() {
        assert_eq!(-Tuple::new(1, -2, 3, -4), Tuple::new(-1, 2, -3, 4))
    }

    #[test]
    fn mul_by_scalar() {
        let tuple = Tuple::new(1, -2, 3, -4);
        assert_eq!(tuple * 3.5, Tuple::new(3.5, -7, 10.5, -14))
    }

    #[test]
    fn mul_by_fraction() {
        let tuple = Tuple::new(1, -2, 3, -4);
        assert_eq!(tuple * 0.5, Tuple::new(0.5, -1, 1.5, -2))
    }

    #[test]
    fn compute_magnitude() {
        assert_eq!(Tuple::new_vector(0, 1, 0).magnitude(), 1f32);
        assert_eq!(Tuple::new_vector(0, 0, 1).magnitude(), 1f32);
        assert_eq!(Tuple::new_vector(1, 2, 3).magnitude(), f32::sqrt(14f32));
        assert_eq!(Tuple::new_vector(-1, -2, -3).magnitude(), f32::sqrt(14f32));
    }

    #[test]
    fn normalize() {
        assert_eq!(Tuple::new_vector(4, 0, 0).normalize(), Tuple::new_vector(1, 0, 0));
        assert_eq!(
            Tuple::new_vector(1, 2, 3).normalize(),
            Tuple::new_vector(
                1f32 / f32::sqrt(14f32),
                2f32 / f32::sqrt(14f32),
                3f32 / f32::sqrt(14f32)
            )
        );
    }

    #[test]
    fn magnitude_of_normalized_vec() {
        assert!(equal_f32(Tuple::new_vector(1, 2, 3).normalize().magnitude(), 1.0))
    }

    #[test]
    fn dot_product() {
        let a = Tuple::new_vector(1, 2, 3);
        let b = Tuple::new_vector(2, 3, 4);
        assert_eq!(a.dot(&b), 20f32)
    }

    #[test]
    fn cross_product() {
        let a = Tuple::new_vector(1, 2, 3);
        let b = Tuple::new_vector(2, 3, 4);
        assert_eq!(a.cross(&b), Tuple::new_vector(-1, 2, -1));
        assert_eq!(b.cross(&a), Tuple::new_vector(1, -2, 1));
    }
}
