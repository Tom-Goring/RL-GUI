#![allow(dead_code)]

use crate::core::size::Size;
use crate::pipelines;
use crate::primitives::layer::Layer;
use crate::primitives::Primitive;
use crate::viewport::Viewport;
use futures::task::SpawnExt;
use glyph_brush::Section;
use raw_window_handle::HasRawWindowHandle;
use unicode_segmentation::UnicodeSegmentation;

/// Data structure to combine elements together and draw them.
/// Will possibly take a renderer argument in the future for modularity
/// A compositor takes multiple drawable types and 'squishes' them together into a single image to be rendered by the
/// gpu
pub struct Compositor {
    instance: wgpu::Instance,
    device: wgpu::Device,
    queue: wgpu::Queue,
    staging_belt: wgpu::util::StagingBelt,
    local_pool: futures::executor::LocalPool,

    triangle_pipeline: pipelines::triangle::Pipeline,
    quad_pipeline: pipelines::quad::Pipeline,
    text_pipeline: pipelines::text::Pipeline,
}

impl Compositor {
    pub async fn new() -> Self {
        let instance = wgpu::Instance::new(wgpu::BackendBit::PRIMARY);

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: { wgpu::PowerPreference::default() },
                compatible_surface: None,
            })
            .await
            .unwrap();

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    features: wgpu::Features::empty(),
                    limits: wgpu::Limits {
                        max_bind_groups: 2,
                        ..wgpu::Limits::default()
                    },
                    shader_validation: false,
                },
                None,
            )
            .await
            .unwrap();

        let staging_belt = wgpu::util::StagingBelt::new(10 * 1024);
        let local_pool = futures::executor::LocalPool::new();

        let triangle_pipeline =
            pipelines::triangle::Pipeline::new(&device, wgpu::TextureFormat::Bgra8UnormSrgb);

        let quad_pipeline =
            pipelines::quad::Pipeline::new(&device, wgpu::TextureFormat::Bgra8UnormSrgb);

        let text_pipeline =
            pipelines::text::Pipeline::new(&device, wgpu::TextureFormat::Bgra8UnormSrgb);

        Self {
            instance,
            device,
            queue,
            staging_belt,
            local_pool,
            triangle_pipeline,
            quad_pipeline,
            text_pipeline,
        }
    }

    pub fn create_surface<W: HasRawWindowHandle>(&mut self, window: &W) -> wgpu::Surface {
        unsafe { self.instance.create_surface(window) }
    }

    pub fn create_swap_chain(
        &mut self,
        surface: &wgpu::Surface,
        width: u32,
        height: u32,
    ) -> wgpu::SwapChain {
        self.device.create_swap_chain(
            surface,
            &wgpu::SwapChainDescriptor {
                usage: wgpu::TextureUsage::OUTPUT_ATTACHMENT,
                format: wgpu::TextureFormat::Bgra8UnormSrgb,
                width,
                height,
                present_mode: wgpu::PresentMode::Fifo,
            },
        )
    }

    pub fn draw(
        &mut self,
        swap_chain: &mut wgpu::SwapChain,
        primitives: Primitive,
        viewport: &Viewport,
    ) {
        let scale_factor = viewport.scale_factor() as f32;

        let frame = swap_chain.get_current_frame().expect("Next frame");

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        let _ = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            color_attachments: &[wgpu::RenderPassColorAttachmentDescriptor {
                attachment: &frame.output.view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear({
                        wgpu::Color {
                            r: 1.0,
                            g: 1.0,
                            b: 1.0,
                            a: 1.0,
                        }
                    }),
                    store: true,
                },
            }],
            depth_stencil_attachment: None,
        });

        let coord_translator = viewport.projection();
        let layer = Layer::generate(&primitives, viewport);

        if !layer.quads.is_empty() {
            self.quad_pipeline.draw(
                &self.device,
                &mut encoder,
                &mut self.staging_belt,
                &frame.output.view,
                &layer.quads,
                layer.bounds() * scale_factor,
                coord_translator,
                scale_factor,
            )
        }

        if !layer.text.is_empty() {
            for text in layer.text.iter() {
                let section = Section {
                    screen_position: (text.bounds.x * scale_factor, text.bounds.y * scale_factor),
                    bounds: (
                        text.bounds.width * scale_factor,
                        text.bounds.height * scale_factor,
                    ),
                    layout: Default::default(),
                    text: vec![wgpu_glyph::Text {
                        text: &text.content,
                        scale: wgpu_glyph::ab_glyph::PxScale {
                            x: text.size * scale_factor,
                            y: text.size * scale_factor,
                        },
                        font_id: Default::default(),
                        ..Default::default()
                    }],
                };

                self.text_pipeline.queue(section);
            }

            self.text_pipeline
                .draw_brush
                .draw_queued(
                    &self.device,
                    &mut self.staging_belt,
                    &mut encoder,
                    &frame.output.view,
                    viewport.physical_size().width,
                    viewport.physical_size().height,
                )
                .expect("Text draw queued");
        }

        self.staging_belt.finish();
        self.queue.submit(Some(encoder.finish()));

        self.local_pool
            .spawner()
            .spawn(self.staging_belt.recall())
            .expect("Recall staging belt");

        self.local_pool.run_until_stalled();
    }

    pub fn measure_text(&mut self, contents: &str, size: f32, bounds: Size) -> (f32, f32) {
        self.text_pipeline.measure(contents, size, bounds)
    }

    pub fn find_cursor_position(&mut self, value: &str, size: u16, target: f32) -> usize {
        let graphemes: Vec<String> = UnicodeSegmentation::graphemes(value, true)
            .map(String::from)
            .collect();

        if graphemes.is_empty() {
            return 0;
        }

        let mut total: f32 = 0.0;
        for (idx, grapheme) in graphemes.into_iter().enumerate() {
            let width = self.measure_text(&grapheme, size as f32, Size::INFINITY).0;
            total += width / 2.0;
            if total > target {
                return idx;
            }
            total += width / 2.0;
            if total > target {
                return idx + 1;
            }
        }

        value.len()
    }

    pub fn measure_cursor_position(&mut self, value: &str, index: usize, size: u16) -> f32 {
        self.measure_text(&value[..index], size as f32, Size::INFINITY)
            .0
    }
}
