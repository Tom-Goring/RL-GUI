use crate::compositor::Compositor;
use crate::core::point::Point;
use crate::core::size::Size;
use crate::element::Element;
use crate::layout::align::Alignment;
use crate::layout::axis::Axis;
use crate::layout::limits::Limits;
use crate::layout::node::Node;

// Lays out a bunch of widgets vertically or horizontally. Currently only supports left and top alignment and is
// is probably broken when using a column of rows or vice versa but I'm unsure why.
pub fn resolve<Message>(
    elements: &[Element<Message>],
    _alignment: Alignment,
    axis: Axis,
    limits: &Limits,
    mut renderer: &mut Compositor,
    padding: f32,
) -> Node {
    let nodes: Vec<Node> = elements
        .iter()
        .map(|element| element.layout(&mut renderer, *limits))
        .collect();

    let total_required_width: f32 = nodes.iter().map(|node| node.bounds.width).sum();
    let total_required_height: f32 = nodes.iter().map(|node| node.bounds.height).sum();

    let windows = nodes.windows(2);

    let mut new_nodes = Vec::new();

    for window in windows {
        let first_node = window[0].clone();
        let mut second_node = window[1].clone();

        match axis {
            Axis::Vertical => second_node.reposition(Point {
                x: first_node.bounds.x,
                y: first_node.bounds.y + first_node.bounds.height + padding,
            }),
            Axis::Horizontal => second_node.reposition(Point {
                x: first_node.bounds.x + first_node.bounds.width + padding,
                y: first_node.bounds.y,
            }),
        }

        new_nodes.push(first_node);
        new_nodes.push(second_node);
    }

    let required_size = Size::new(total_required_width, total_required_height);
    let size = limits.resolve(required_size);

    Node::with_children(size, new_nodes)
}
