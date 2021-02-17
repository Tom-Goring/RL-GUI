use crate::core::bounds::Bounds;
use crate::primitives::quad::Quad;
use crate::primitives::Primitive;

pub struct Layer {
    pub quads: Vec<Quad>,
    pub text: Vec<Text>,
}

impl Layer {
    pub fn generate(primitive: &Primitive) -> Self {
        match primitive {
            Primitive::Group { primitives } => {
                let mut quads = Vec::new();
                let mut text = Vec::new();
                for primitive in primitives {
                    match primitive {
                        Primitive::Quad { bounds, color } => quads.push(Quad {
                            position: [bounds.x, bounds.y],
                            color: *color,
                            size: [bounds.width, bounds.height],
                        }),
                        Primitive::Text {
                            content,
                            bounds,
                            size,
                        } => text.push(Text {
                            content: content.clone(),
                            bounds: *bounds,
                            size: *size,
                        }),
                        _ => {}
                    }
                }
                Self { quads, text }
            }
            _ => Self {
                quads: Vec::new(),
                text: Vec::new(),
            },
        }
    }
}

pub struct Text {
    pub content: String,
    pub bounds: Bounds,
    pub size: f32,
}
