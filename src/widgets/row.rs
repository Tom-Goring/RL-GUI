#![allow(dead_code)]
use crate::compositor::Compositor;
use crate::core::length::Length;
use crate::core::point::Point;
use crate::element::Element;
use crate::events::Event;
use crate::layout::limits::Limits;
use crate::layout::node::Node;
use crate::primitives::Primitive;
use crate::viewport::Viewport;
use crate::widgets::Widget;

pub struct Row<'a, Message> {
    width: Length,
    height: Length,
    max_width: u32,
    max_height: u32,
    children: Vec<Element<'a, Message>>,
}

impl<'a, Message> Row<'a, Message> {
    pub fn new() -> Self {
        Self::with_children(Vec::new())
    }

    pub fn with_children(children: Vec<Element<'a, Message>>) -> Self {
        Self {
            width: Length::Fill,
            height: Length::Fill,
            max_height: u32::MAX,
            max_width: u32::MAX,
            children,
        }
    }
}

impl<'a, Message> Default for Row<'a, Message> {
    fn default() -> Self {
        Self::with_children(Vec::new())
    }
}

impl<'a, Message> Widget<Message> for Row<'a, Message> {
    fn draw(&self, node: Node) -> Primitive {
        Primitive::Group {
            primitives: self
                .children
                .iter()
                .zip(node.children)
                .map(|(child, layout)| child.draw(layout))
                .collect(),
        }
    }

    fn on_event(
        &mut self,
        event: Event,
        cursor_position: Point,
        viewport: Viewport,
        messages: &mut Vec<Message>,
        layout: Node,
    ) {
        let children = layout.children;
        self.children
            .iter_mut()
            .zip(children)
            .for_each(|(child, layout)| {
                child.on_event(event.clone(), cursor_position, viewport, messages, layout)
            });
    }

    fn layout(&self, renderer: &mut Compositor, limits: Limits) -> Node {
        let limits = limits
            .max_width(self.max_width)
            .max_height(self.max_height)
            .width(self.width)
            .height(self.height);

        // First calculate the sizes of all children
        // Then we can know how wide and tall the row has to be, resolve that size with the min and max sizes given

        unimplemented!()
    }
}
