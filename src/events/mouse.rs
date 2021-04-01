use crate::core::point::Point;

#[derive(Copy, Clone, Debug)]
pub enum Event {
    CursorEnteredWindow,
    CursorLeftWindow,
    CursorMoved(Point),
    Pressed(Button),
    Released(Button),
}

#[derive(Copy, Clone, Debug)]
pub enum Button {
    Left,
    Right,
    Other,
}
