#[derive(Debug, Clone, Copy)]
/// Holds information on a single arbitrary xy pair of coordinates.
pub struct Point {
    pub x: f32,
    pub y: f32,
}

impl Point {
    pub const fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}
