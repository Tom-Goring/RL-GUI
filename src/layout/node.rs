use crate::core::bounds::Bounds;
use crate::core::size::Size;

// Holds data on the bounds of an element, along with all its children recursively
#[derive(Clone, Debug)]
pub struct Node {
    pub bounds: Bounds,
    pub children: Vec<Node>,
}

impl Node {
    pub fn new(size: Size) -> Self {
        Self::with_children(size, Vec::new())
    }

    pub fn with_children(size: Size, children: Vec<Node>) -> Self {
        Self {
            bounds: Bounds {
                x: 0.0,
                y: 0.0,
                width: size.width,
                height: size.height,
            },
            children,
        }
    }

    pub fn reposition(&mut self, x: Option<f32>, y: Option<f32>) {
        if let Some(x) = x {
            self.bounds.x = x;
        }
        if let Some(y) = y {
            self.bounds.y = y;
        }

        for child in &mut self.children {
            child.reposition(x, y);
        }
    }

    pub fn size(&self) -> Size {
        Size::new(self.bounds.width, self.bounds.height)
    }
}
