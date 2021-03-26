use crate::compositor::Compositor;
use crate::core::point::Point;
use crate::core::size::Size;
use crate::element::Element;
use crate::layout::align::Alignment;
use crate::layout::axis::Axis;
use crate::layout::limits::Limits;
use crate::layout::node::Node;

// TODO: use alignment to properly align nodes

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

    let total_required_width: f32;
    let total_required_height: f32;

    // if vertical, required width is the largest width of the children, height is all added
    // if horizontal, required width is all added, height is largest of children
    match axis {
        Axis::Vertical => {
            total_required_height = nodes.iter().map(|node| node.bounds.height).sum();
            total_required_width = nodes
                .iter()
                .map(|node| node.bounds.width as u32)
                .max()
                .unwrap() as f32; // TODO: fix this hack
        }
        Axis::Horizontal => {
            total_required_width = nodes.iter().map(|node| node.bounds.width).sum();
            total_required_height = nodes
                .iter()
                .map(|node| node.bounds.height as u32)
                .max()
                .unwrap() as f32; // TODO: fix this hack
        }
    }

    if nodes.is_empty() {
        panic!("Tried to flex-resolve an empty node tree");
    }

    let mut new_nodes = Vec::new();

    // only try to resolve nodes if we have multiple, else just use the single node
    if nodes.len() == 1 {
        new_nodes.push(nodes.first().unwrap().clone());
    } else {
        let windows = nodes.windows(2);

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
    }

    let required_size = Size::new(total_required_width, total_required_height);
    let size = limits.resolve(required_size);

    Node::with_children(size, new_nodes)
}
