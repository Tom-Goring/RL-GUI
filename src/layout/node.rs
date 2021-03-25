use crate::core::bounds::Bounds;
use crate::core::point::Point;
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

    pub fn reposition(&mut self, point: Point) {
        self.bounds.x = point.x;
        self.bounds.y = point.y;

        for child in &mut self.children {
            child.reposition(point);
        }
    }

    pub fn size(&self) -> Size {
        Size::new(self.bounds.width, self.bounds.height)
    }
}
