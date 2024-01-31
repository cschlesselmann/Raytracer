use auto_ops::impl_op_ex;
use crate::algebra::utils::equal_f32;

#[derive(Debug, PartialOrd, Copy, Clone)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

pub const BLACK: Color = Color{ r: 0.0, g: 0.0, b: 0.0 };

#[rustfmt::skip]
impl_op_ex!(+ |a: &Color, b: &Color| -> Color { Color::new(a.r + b.r, a.g + b.g, a.b + b.b) });
#[rustfmt::skip]
impl_op_ex!(- |a: &Color, b: &Color| -> Color { Color::new(a.r - b.r, a.g - b.g, a.b - b.b) });
#[rustfmt::skip]
impl_op_ex!(* |a: &Color, b: &Color| -> Color { Color::new(a.r * b.r, a.g * b.g, a.b * b.b) });
#[rustfmt::skip]
impl_op_ex!(* |a: &Color, b: &f32| -> Color { Color::new(a.r * b, a.g * b, a.b * b) });

impl PartialEq for Color {
    fn eq(&self, other: &Self) -> bool {
        equal_f32(self.r, other.r) && equal_f32(self.g, other.g) && equal_f32(self.b, other.b)
    }
}

impl Color {
    pub fn new(r: impl Into<f64>, g: impl Into<f64>, b: impl Into<f64>) -> Color {
        Color {
            r: r.into() as f32,
            g: g.into() as f32,
            b: b.into() as f32,
        }
    }
}

#[cfg(test)]
mod tests {
    use ctor::ctor;
    use log::LevelFilter;
    use simple_logger::SimpleLogger;
    use crate::rendering::Color;

    #[ctor]
    fn foo() {
        SimpleLogger::new()
            .with_level(LevelFilter::Trace)
            .init()
            .unwrap_or_else(|e| println!("[{}] {}", module_path!(), e));
    }

    #[test]
    fn color_is_tuple() {
        let c = Color::new(-0.5, 0.4, 1.7);
        assert_eq!(c.r, -0.5);
        assert_eq!(c.g, 0.4);
        assert_eq!(c.b, 1.7);
    }

    #[test]
    fn adding_colors() {
        let c1 = Color::new(0.9, 0.6, 0.75);
        let c2 = Color::new(0.7, 0.1, 0.25);

        assert_eq!(c1 + c2, Color::new(1.6, 0.7, 1.0));
    }

    #[test]
    fn subtracting_colors() {
        let c1 = Color::new(0.9, 0.6, 0.75);
        let c2 = Color::new(0.7, 0.1, 0.25);

        assert_eq!(c1 - c2, Color::new(0.2, 0.5, 0.5));
    }

    #[test]
    fn mul_color_scalar() {
        let c1 = Color::new(0.2, 0.3, 0.4);

        assert_eq!(c1 * 2.0, Color::new(0.4, 0.6, 0.8));
    }

    #[test]
    fn mul_colors() {
        let c1 = Color::new(1, 0.2, 0.4);
        let c2 = Color::new(0.9, 1, 0.1);

        assert_eq!(c1 * c2, Color::new(0.9, 0.2, 0.04));
    }
}