use crate::core::bounds::Bounds;
use crate::primitives::quad::Quad;
use crate::primitives::Primitive;
use crate::viewport::Viewport;

#[derive(Debug)]
pub struct Layer {
    pub quads: Vec<Quad>,
    pub text: Vec<Text>,
    bounds: Bounds,
}

impl Layer {
    pub fn generate(primitive: &Primitive, viewport: &Viewport) -> Self {
        let bounds = Bounds::with_size(viewport.logical_size());
        let mut quads = Vec::new();
        let mut text = Vec::new();
        process_primitive(primitive.clone(), bounds, &mut quads, &mut text);
        Layer {
            quads,
            text,
            bounds,
        }
    }

    pub fn bounds(&self) -> Bounds {
        self.bounds
    }
}

fn process_primitive(
    primitive: Primitive,
    bounds: Bounds,
    mut quads: &mut Vec<Quad>,
    mut text: &mut Vec<Text>,
) {
    match primitive {
        Primitive::None => {}
        Primitive::Quad {
            bounds,
            color,
            border_colour,
            border_width,
        } => {
            quads.push(Quad {
                position: [bounds.x, bounds.y],
                color,
                size: [bounds.width, bounds.height],
                border_colour,
                border_width,
            });
        }
        Primitive::Text {
            content,
            bounds,
            size,
        } => {
            text.push(Text {
                content,
                bounds,
                size,
            });
        }
        Primitive::Group { primitives } => {
            for primitive in primitives {
                process_primitive(primitive, bounds, &mut quads, &mut text);
            }
        }
    };
}

fn _old(primitive: Primitive, bounds: Bounds) -> Layer {
    match primitive {
        Primitive::Group { primitives } => {
            let mut quads = Vec::new();
            let mut text = Vec::new();
            for primitive in primitives {
                match primitive {
                    Primitive::Quad {
                        bounds,
                        color,
                        border_colour,
                        border_width,
                    } => {
                        quads.push(Quad {
                            position: [bounds.x, bounds.y],
                            color,
                            size: [bounds.width, bounds.height],
                            border_colour,
                            border_width,
                        });
                    }
                    Primitive::Text {
                        content,
                        bounds,
                        size,
                    } => text.push(Text {
                        content: content.clone(),
                        bounds,
                        size,
                    }),
                    Primitive::Group { primitives } => {
                        for primitive in primitives {
                            process_primitive(primitive, bounds, &mut quads, &mut text);
                        }
                    }
                    Primitive::None => {}
                }
            }
            Layer {
                quads,
                text,
                bounds,
            }
        }
        _ => Layer {
            quads: Vec::new(),
            text: Vec::new(),
            bounds,
        },
    }
}

#[derive(Debug)]
pub struct Text {
    pub content: String,
    pub bounds: Bounds,
    pub size: f32,
}
