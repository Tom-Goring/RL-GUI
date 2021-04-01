use crate::core::bounds::Bounds;

pub mod layer;
pub mod quad;
pub mod triangle;
pub mod vertex;

#[derive(Debug, Clone)]
/// Holds rendering primitives
pub enum Primitive {
    None,
    Quad {
        bounds: Bounds,
        color: [f32; 3],
        border_colour: [f32; 3],
        border_width: f32,
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
