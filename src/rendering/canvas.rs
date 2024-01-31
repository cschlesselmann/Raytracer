use crate::rendering::{BLACK, Color};

#[derive(Debug, PartialEq)]
pub struct Canvas {
    pub width: usize,
    pub height: usize,
    buffer: Vec<Color>
}

#[derive(Debug, PartialEq)]
pub enum CanvasError {
    PixelPosOutOfBounds(String)
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Canvas {
        Canvas {
            width,
            height,
            buffer: vec![BLACK; width * height]
        }
    }

    fn assert_pixel_bounds(&self, x: usize, y: usize) -> Result<(), CanvasError> {
        if x >= self.width {
            return Err(CanvasError::PixelPosOutOfBounds(format!("x: {} >= {}", x, self.width)))
        } else if y >= self.height {
            return Err(CanvasError::PixelPosOutOfBounds(format!("y: {} >= {}", y, self.height)))
        }

        Ok(())
    }


    pub fn set_pixel(&mut self, x: usize, y: usize, color: &Color) -> Result<(), CanvasError> {
        self.assert_pixel_bounds(x, y)?;
        self.buffer.insert(x * self.width + y * self.height, *color);
        Ok(())
    }

    pub fn get_pixel(&self, x: usize, y: usize) -> Result<&Color, CanvasError> {
        self.assert_pixel_bounds(x, y)?;
        match self.buffer.get(x * self.width + y * self.height) {
            None => unreachable!(),
            Some(c) => Ok(c)
        }
    }
}

#[cfg(test)]
mod test {
    use ctor::ctor;
    use log::LevelFilter;
    use simple_logger::SimpleLogger;
    use crate::rendering::{BLACK, Canvas, Color};

    #[ctor]
    fn foo() {
        SimpleLogger::new()
            .with_level(LevelFilter::Trace)
            .init()
            .unwrap_or_else(|e| println!("[{}] {}", module_path!(), e));
    }

    #[test]
    fn create_canvas() {
        let c = Canvas::new(10, 20);
        assert_eq!(c.width, 10);
        assert_eq!(c.height, 20);

        for color in &c.buffer {
            assert_eq!(*color, BLACK);
        }
    }

    #[test]
    fn write_to_canvas() {
        let mut c = Canvas::new(10, 20);
        let red = Color{
            r: 1.0,
            g: 0.0,
            b: 0.0,
        };

        c.set_pixel(2, 3, &red).unwrap();
        assert_eq!(c.get_pixel(2, 3).unwrap(), &red);
    }
}
