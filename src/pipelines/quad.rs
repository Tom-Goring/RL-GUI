use crate::primitives::quad::QuadInstance;
use crate::primitives::vertex::Vertex;
use wgpu::util::DeviceExt;

const VERTICES: [Vertex; 4] = [
    Vertex {
        position: [0.0, 0.0],
    },
    Vertex {
        position: [1.0, 0.0],
    },
    Vertex {
        position: [1.0, 1.0],
    },
    Vertex {
        position: [0.0, 1.0],
    },
];

const INDICES: [u16; 6] = [0, 1, 2, 0, 2, 3];

pub struct Pipeline {
    render_pipeline: wgpu::RenderPipeline,
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    instance_buffer: wgpu::Buffer,
}

impl Pipeline {
    pub fn new(device: &wgpu::Device, format: wgpu::TextureFormat) -> Self {
        let vs_src = include_str!("../shaders/quad.vert");
        let fs_src = include_str!("../shaders/quad.frag");
        let mut compiler = shaderc::Compiler::new().unwrap();

        let (vs_spirv, fs_spirv) = {
            let vs_spirv = compiler
                .compile_into_spirv(
                    vs_src,
                    shaderc::ShaderKind::Vertex,
                    "quad.vert",
                    "main",
                    None,
                )
                .unwrap();
            let fs_spirv = compiler
                .compile_into_spirv(
                    fs_src,
                    shaderc::ShaderKind::Fragment,
                    "quad.frag",
                    "main",
                    None,
                )
                .unwrap();

            (vs_spirv, fs_spirv)
        };

        let (vs_module, fs_module) = {
            let vs_module =
                device.create_shader_module(wgpu::util::make_spirv(&vs_spirv.as_binary_u8()));
            let fs_module =
                device.create_shader_module(wgpu::util::make_spirv(&fs_spirv.as_binary_u8()));

            (vs_module, fs_module)
        };

        let (vertex_buffer, index_buffer, instance_buffer) = {
            let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Quad Vertex Buffer"),
                contents: bytemuck::cast_slice(&VERTICES[..]),
                usage: wgpu::BufferUsage::VERTEX,
            });

            let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Quad Index Buffer"),
                contents: bytemuck::cast_slice(&INDICES[..]),
                usage: wgpu::BufferUsage::INDEX,
            });

            let instance_buffer = device.create_buffer(&wgpu::BufferDescriptor {
                label: Some("Quad Instance Buffer"),
                size: std::mem::size_of::<QuadInstance>() as u64 * 10000 as u64,
                usage: wgpu::BufferUsage::VERTEX | wgpu::BufferUsage::COPY_DST,
                mapped_at_creation: false,
            });

            (vertex_buffer, index_buffer, instance_buffer)
        };

        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Quad Render Pipeline Layout"),
                bind_group_layouts: &[],
                push_constant_ranges: &[],
            });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Quad Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex_stage: wgpu::ProgrammableStageDescriptor {
                module: &vs_module,
                entry_point: "main",
            },
            fragment_stage: Some(wgpu::ProgrammableStageDescriptor {
                module: &fs_module,
                entry_point: "main",
            }),
            rasterization_state: Some(wgpu::RasterizationStateDescriptor {
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: wgpu::CullMode::Back,
                ..Default::default()
            }),
            primitive_topology: wgpu::PrimitiveTopology::TriangleList,
            color_states: &[wgpu::ColorStateDescriptor {
                format,
                color_blend: wgpu::BlendDescriptor {
                    src_factor: wgpu::BlendFactor::SrcAlpha,
                    dst_factor: wgpu::BlendFactor::OneMinusSrcAlpha,
                    operation: wgpu::BlendOperation::Add,
                },
                alpha_blend: wgpu::BlendDescriptor {
                    src_factor: wgpu::BlendFactor::One,
                    dst_factor: wgpu::BlendFactor::OneMinusSrcAlpha,
                    operation: wgpu::BlendOperation::Add,
                },
                write_mask: wgpu::ColorWrite::ALL,
            }],
            depth_stencil_state: None,
            vertex_state: wgpu::VertexStateDescriptor {
                index_format: wgpu::IndexFormat::Uint16,
                vertex_buffers: &[Vertex::desc(), QuadInstance::desc()],
            },
            sample_count: 1,
            sample_mask: !0,
            alpha_to_coverage_enabled: false,
        });

        Self {
            render_pipeline,
            vertex_buffer,
            index_buffer,
            instance_buffer,
        }
    }

    pub fn draw(
        &self,
        encoder: &mut wgpu::CommandEncoder,
        target: &wgpu::SwapChainFrame,
        queue: &wgpu::Queue,
        instances: &[QuadInstance],
    ) {
        let mut render_pass = super::begin_load_render_pass(encoder, &target);

        queue.write_buffer(&self.instance_buffer, 0, bytemuck::cast_slice(instances));

        render_pass.set_pipeline(&self.render_pipeline);
        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        render_pass.set_vertex_buffer(1, self.instance_buffer.slice(..));
        render_pass.set_index_buffer(self.index_buffer.slice(..));
        render_pass.draw_indexed(0..INDICES.len() as u32, 0, 0..instances.len() as u32);
    }
}
