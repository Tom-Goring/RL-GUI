use crate::core::point::Point;

#[derive(Copy, Clone)]
pub struct Viewport {
    height: u32,
    width: u32,
}

impl Viewport {
    pub fn new(width: u32, height: u32) -> Self {
        Viewport { width, height }
    }

    pub fn normalise_window_coords(&self, point: Point) -> Point {
        let x = ((point.x as f32 / self.width as f32) * 2.0) - 1.0;
        let y = ((point.y as f32 / self.height as f32) * 2.0) - 1.0;
        Point::new(x, y)
    }
}
