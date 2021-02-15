pub mod quad;
pub mod text;
pub mod triangle;
pub mod vertex;

/// Holds rendering primitives
pub enum Primitive {
    None,
    Quad {
        position: [f32; 2],
        color: [f32; 3],
        size: [f32; 2],
    },
    Group(Vec<Primitive>),
}
