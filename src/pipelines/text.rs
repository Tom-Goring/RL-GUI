#![allow(dead_code)]

use wgpu_glyph::ab_glyph;

pub struct Pipeline {
    pub draw_brush: wgpu_glyph::GlyphBrush<()>,
}

impl Pipeline {
    pub fn new(device: &wgpu::Device, format: wgpu::TextureFormat) -> Self {
        let default_font = include_bytes!("../fonts/Lato-Regular.ttf");
        let font = ab_glyph::FontArc::try_from_slice(default_font).expect("Failed to load font");
        let draw_brush = wgpu_glyph::GlyphBrushBuilder::using_font(font.clone())
            .initial_cache_size((2048, 2048))
            .draw_cache_multithread(false)
            .build(device, format);

        Pipeline { draw_brush }
    }

    pub fn queue(&mut self, section: wgpu_glyph::Section<'_>) {
        self.draw_brush.queue(section);
    }

    // TODO: make draw_brush private and expose draw_queued with a wrapper
}
