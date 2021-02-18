#![allow(dead_code)]

use crate::core::size::Size;
use crate::pipelines;
use crate::primitives::layer::Layer;
use crate::primitives::Primitive;
use crate::viewport::Viewport;
use futures::task::SpawnExt;
use glyph_brush::Section;
use raw_window_handle::HasRawWindowHandle;
use wgpu_glyph::Text;

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
                layer.bounds(),
                coord_translator,
            )
        }

        if !layer.text.is_empty() {
            for text in layer.text.iter() {
                let section = Section {
                    screen_position: (text.bounds.x, text.bounds.y),
                    bounds: (text.bounds.width, text.bounds.height),
                    layout: Default::default(),
                    text: vec![Text::new(&text.content).with_scale(
                        wgpu_glyph::ab_glyph::PxScale {
                            x: text.size,
                            y: text.size,
                        },
                    )],
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
}
