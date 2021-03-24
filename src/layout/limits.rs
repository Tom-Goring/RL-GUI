use crate::core::length::Length;
use crate::core::size::Size;

// TODO: Add support for padding?

/// Holds data on the minimum w/h, maximum w/h, and a size to fill up to
#[derive(Copy, Clone, Debug)]
pub struct Limits {
    pub min: Size,
    pub max: Size,
    pub fill: Size,
}

impl Limits {
    pub fn new(min: Size, max: Size) -> Limits {
        Limits {
            min,
            max,
            fill: Size::INFINITY,
        }
    }

    pub fn width(mut self, width: Length) -> Limits {
        match width {
            Length::Shrink => self.fill.width = self.min.width,
            Length::Fill => self.fill.width = self.fill.width.min(self.max.width), // Make the width the minimum of the current vs
        }

        self
    }

    pub fn height(mut self, height: Length) -> Limits {
        match height {
            Length::Shrink => self.fill.height = self.min.height,
            Length::Fill => self.fill.height = self.fill.height.min(self.max.height), // Make the width the minimum of the current vs
        }

        self
    }

    /// Returns a new set of limits where the min_width is the minimum of the max width and the maximum of the new min
    /// width and the current min width. IF the new minimum width is greater than the current maximum width, we set the
    /// new min width to be the current max width, and no greater.
    pub fn min_width(mut self, min_width: u32) -> Limits {
        self.min.width = self.min.width.max(min_width as f32).min(self.max.width);
        self
    }

    /// See min_width()
    pub fn min_height(mut self, min_height: u32) -> Limits {
        self.min.height = self.min.height.max(min_height as f32).min(self.max.height);
        self
    }

    // Sets a new max width if it is smaller than the current max width but larger than the current min width
    pub fn max_width(mut self, max_width: u32) -> Limits {
        self.max.width = self.max.width.min(max_width as f32).max(self.min.width);
        self
    }

    // Sets a new max height if it is smaller than the current max width but larger than the current min width
    pub fn max_height(mut self, max_height: u32) -> Limits {
        self.max.height = self.max.height.min(max_height as f32).max(self.min.height);
        self
    }

    // Generates a size that fits the given size. The lengths are reduced to the maximum, and increased to the fill
    pub fn resolve(&self, intrinsic_size: Size) -> Size {
        Size::new(
            intrinsic_size
                .width
                .min(self.max.width)
                .max(self.fill.width),
            intrinsic_size
                .height
                .min(self.max.height)
                .max(self.fill.height),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_width() {
        let mut limits = Limits::new(Size::new(0.0, 0.0), Size::new(1000.0, 1000.0))
            .width(Length::Shrink)
            .height(Length::Shrink);
        println!("{:?}", limits.resolve(Size::new(100.0, 100.0)));
    }
}
