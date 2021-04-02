use crate::widgets::text_input::value::TextValue;

#[derive(Default, Copy, Clone)]
pub struct Cursor {
    pub index: usize,
}

impl Cursor {
    pub fn move_left(&mut self) {
        self.index = (self.index).saturating_sub(1).max(0);
    }

    pub fn move_right(&mut self, value: &TextValue) {
        self.index = (self.index + 1).min(value.len());
    }
}
