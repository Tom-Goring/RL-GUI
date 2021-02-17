#![allow(dead_code)]

use crate::core::size::Size;
use wgpu_glyph::ab_glyph;

pub struct Pipeline {
    pub draw_brush: wgpu_glyph::GlyphBrush<()>,
    pub measure_brush: glyph_brush::GlyphBrush<()>,
}

impl Pipeline {
    pub fn new(device: &wgpu::Device, format: wgpu::TextureFormat) -> Self {
        let default_font = include_bytes!("../fonts/Lato-Regular.ttf");
        let font = ab_glyph::FontArc::try_from_slice(default_font).expect("Failed to load font");
        let draw_brush = wgpu_glyph::GlyphBrushBuilder::using_font(font.clone())
            .initial_cache_size((2048, 2048))
            .draw_cache_multithread(false)
            .build(device, format);

        let measure_brush = glyph_brush::GlyphBrushBuilder::using_font(font).build();

        Pipeline {
            draw_brush,
            measure_brush,
        }
    }

    pub fn queue(&mut self, section: wgpu_glyph::Section<'_>) {
        self.draw_brush.queue(section);
    }

    pub fn measure(&mut self, content: &str, size: f32, bounds: Size) -> (f32, f32) {
        use wgpu_glyph::GlyphCruncher;

        let section = wgpu_glyph::Section {
            bounds: (bounds.width, bounds.height),
            layout: Default::default(),
            text: vec![wgpu_glyph::Text::new(content).with_scale(size)],
            ..Default::default()
        };

        if let Some(bounds) = self.measure_brush.glyph_bounds(section) {
            (bounds.width().ceil(), bounds.height().ceil())
        } else {
            (0.0, 0.0)
        }
    }

    // TODO: make draw_brush private and expose draw_queued with a wrapper
}
