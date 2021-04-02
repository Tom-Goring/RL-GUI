use crate::compositor::Compositor;
use crate::core::length::Length;
use crate::core::point::Point;
use crate::element::Element;
use crate::events::Event;
use crate::layout;
use crate::layout::align::Alignment;
use crate::layout::axis::Axis;
use crate::layout::limits::Limits;
use crate::layout::node::Node;
use crate::primitives::Primitive;
use crate::viewport::Viewport;
use crate::widgets::Widget;

pub struct Column<'a, Message> {
    width: Length,
    height: Length,
    max_width: u32,
    max_height: u32,
    alignment: Alignment,
    children: Vec<Element<'a, Message>>,
    padding: f32,
}

impl<'a, Message> Column<'a, Message> {
    pub fn new() -> Self {
        Self::with_children(Vec::new())
    }

    pub fn with_children(children: Vec<Element<'a, Message>>) -> Self {
        Self {
            width: Length::Shrink,
            height: Length::Shrink,
            max_height: u32::MAX,
            max_width: u32::MAX,
            alignment: Alignment::Left,
            children,
            padding: 0.0,
        }
    }

    pub fn alignment(mut self, alignment: Alignment) -> Self {
        self.alignment = alignment;
        self
    }

    pub fn padding(mut self, padding: f32) -> Self {
        self.padding += padding;
        self
    }

    pub fn max_width(mut self, max_width: u32) -> Self {
        self.max_width = max_width;
        self
    }

    pub fn max_height(mut self, max_height: u32) -> Self {
        self.max_height = max_height;
        self
    }
}

impl<'a, Message> Default for Column<'a, Message> {
    fn default() -> Self {
        Self::with_children(Vec::new())
    }
}

impl<'a, Message> Widget<Message> for Column<'a, Message> {
    fn draw(&self, node: Node, compositor: &mut Compositor) -> Primitive {
        Primitive::Group {
            primitives: self
                .children
                .iter()
                .zip(node.children)
                .map(|(child, layout)| child.draw(layout, compositor))
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
        compositor: &mut Compositor,
    ) {
        let children = layout.children;
        self.children
            .iter_mut()
            .zip(children)
            .for_each(|(child, layout)| {
                child.on_event(
                    event,
                    cursor_position,
                    viewport,
                    messages,
                    layout,
                    compositor,
                )
            });
    }

    fn layout(&self, mut renderer: &mut Compositor, limits: Limits) -> Node {
        let limits = limits
            .max_width(self.max_width)
            .max_height(self.max_height)
            .width(self.width)
            .height(self.height);

        layout::flex::resolve(
            &self.children,
            Alignment::Center,
            Axis::Vertical,
            &limits,
            &mut renderer,
            self.padding,
        )
    }
}

impl<'a, Message> From<Column<'a, Message>> for Element<'a, Message>
where
    Message: 'a + Clone,
{
    fn from(row: Column<'a, Message>) -> Element<'a, Message> {
        Element::new(row)
    }
}
