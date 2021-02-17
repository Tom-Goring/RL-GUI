#[derive(Copy, Clone, Debug)]
pub struct Size<T = f32> {
    pub width: T,
    pub height: T,
}

impl Size {
    pub const ZERO: Size = Size::new(0., 0.);
    pub const INFINITY: Size = Size::new(f32::INFINITY, f32::INFINITY);
}

impl<T> Size<T> {
    pub const fn new(width: T, height: T) -> Self {
        Size { width, height }
    }
}
