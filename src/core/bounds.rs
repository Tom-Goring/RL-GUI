use super::point::Point;
use super::size::Size;

/// Holds data on an arbitrary set of bounds. Tracks a set of xy coordinates alongside a width and height value.
pub struct Bounds<T = f32> {
    pub x: T,
    pub y: T,
    pub width: T,
    pub height: T,
}

impl Bounds<f32> {
    pub fn new(point: Point, size: Size) -> Self {
        Bounds {
            x: point.x,
            y: point.y,
            width: size.width,
            height: size.height,
        }
    }

    pub fn contains(&self, point: Point) -> bool {
        point.x > self.x
            && point.x < self.x + self.width
            && point.y < self.y
            && point.y > self.y - self.height
    }
}
