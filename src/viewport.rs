use crate::core::point::Point;
use crate::core::size::Size;

#[derive(Copy, Clone)]
pub struct Viewport {
    physical_size: Size<u32>,
    logical_size: Size<f32>,
}

impl Viewport {
    pub fn new(width: u32, height: u32) -> Self {
        Viewport {
            physical_size: Size { width, height },
            logical_size: Size {
                width: width as f32,
                height: height as f32,
            }, // TODO: add support for scaling factor
        }
    }

    pub fn normalise_window_coords(&self, point: Point) -> Point {
        let x = ((point.x as f32 / self.physical_size.width as f32) * 2.0) - 1.0;
        let y = ((point.y as f32 / self.physical_size.height as f32) * 2.0) - 1.0;
        Point::new(x, y)
    }

    pub fn logical_size(&self) -> Size<f32> {
        self.logical_size
    }
}
