#[derive(Copy, Clone)]
pub enum Event {
    Resized { width: u32, height: u32 },
}
