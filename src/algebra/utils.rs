pub const EPSILON: f32 = 0.000001;

pub fn equal_f32(a: f32, b: f32) -> bool {
    f32::abs(a - b) < EPSILON
}
