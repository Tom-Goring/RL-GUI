#[derive(Copy, Clone, Debug)]
pub enum Event {
    KeyPressed {
        key_code: KeyCode,
        shift: bool,
        control: bool,
        alt: bool,
    },
    KeyReleased {
        key_code: KeyCode,
        shift: bool,
        control: bool,
        alt: bool,
    },
    ReceivedCharacter(char),
}

#[derive(Copy, Clone, Debug)]
pub enum KeyCode {
    Left,
    Right,
    Escape,
    Backspace,
    /// The Enter key.
    Enter,
    /// The space bar.
    Space,
    Other,
}
