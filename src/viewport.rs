use crate::core::size::Size;

use glam::Mat4;

#[derive(Copy, Clone)]
pub struct Viewport {
    physical_size: Size<u32>,
    logical_size: Size<f32>,
    projection: Mat4,
}

impl Viewport {
    pub fn new(width: u32, height: u32) -> Self {
        Viewport {
            physical_size: Size { width, height },
            // TODO: add support for scaling factor (so logical size isn't just the same as physical size)
            logical_size: Size {
                width: width as f32,
                height: height as f32,
            },
            projection: Mat4::orthographic_rh_gl(0.0, width as f32, height as f32, 0.0, -1.0, 1.0),
        }
    }

    pub fn logical_size(&self) -> Size<f32> {
        self.logical_size
    }

    pub fn physical_size(&self) -> Size<u32> {
        self.physical_size
    }

    pub fn projection(&self) -> Mat4 {
        self.projection
    }
}

#[cfg(test)]
mod tests {}
