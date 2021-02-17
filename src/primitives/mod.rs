use crate::core::bounds::Bounds;

pub mod layer;
pub mod quad;
pub mod triangle;
pub mod vertex;

/// Holds rendering primitives
pub enum Primitive {
    None,
    Quad {
        bounds: Bounds,
        color: [f32; 3],
    },
    Text {
        content: String,
        bounds: Bounds,
        size: f32,
    },
    Group {
        primitives: Vec<Primitive>,
    },
}
