#[derive(Copy, Clone, Debug)]
pub enum Event {
    Resized { width: u32, height: u32 },
}
