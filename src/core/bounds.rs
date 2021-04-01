use super::point::Point;
use super::size::Size;

#[derive(Copy, Clone, Debug)]
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

    pub fn with_size(size: Size) -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            width: size.width,
            height: size.height,
        }
    }

    pub fn contains(&self, point: Point) -> bool {
        point.x > self.x
            && point.x < self.x + self.width
            && point.y > self.y
            && point.y < self.y + self.height
    }
}

impl std::ops::Mul<f32> for Bounds<f32> {
    type Output = Self;

    fn mul(self, scale: f32) -> Self {
        Self {
            x: self.x as f32 * scale,
            y: self.y as f32 * scale,
            width: self.width * scale,
            height: self.height * scale,
        }
    }
}
