use crate::core::bounds::Bounds;
use crate::primitives::quad::Quad;
use crate::primitives::vertex::Vertex;
use glam::Mat4;
use std::mem;
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
    constants: wgpu::BindGroup,
    constants_buffer: wgpu::Buffer,
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

        let constant_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Quad uniforms layout"),
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStage::VERTEX,
                ty: wgpu::BindingType::UniformBuffer {
                    dynamic: false,
                    min_binding_size: wgpu::BufferSize::new(mem::size_of::<Uniforms>() as u64),
                },
                count: None,
            }],
        });

        let constants_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Quad uniforms buffer"),
            size: mem::size_of::<Uniforms>() as u64,
            usage: wgpu::BufferUsage::UNIFORM | wgpu::BufferUsage::COPY_DST,
            mapped_at_creation: false,
        });

        let constants = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Quad uniforms bind group"),
            layout: &constant_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::Buffer(constants_buffer.slice(..)),
            }],
        });

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
                size: std::mem::size_of::<Quad>() as u64 * 10000 as u64,
                usage: wgpu::BufferUsage::VERTEX | wgpu::BufferUsage::COPY_DST,
                mapped_at_creation: false,
            });

            (vertex_buffer, index_buffer, instance_buffer)
        };

        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Quad Render Pipeline Layout"),
                bind_group_layouts: &[&constant_layout],
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
                cull_mode: wgpu::CullMode::None,
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
                vertex_buffers: &[Vertex::desc(), Quad::desc()],
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
            constants,
            constants_buffer,
        }
    }

    pub fn draw(
        &self,
        device: &wgpu::Device,
        encoder: &mut wgpu::CommandEncoder,
        staging_belt: &mut wgpu::util::StagingBelt,
        target: &wgpu::TextureView,
        instances: &[Quad],
        bounds: Bounds,
        translator: Mat4,
        scale: f32,
    ) {
        let uniforms = Uniforms::new(translator, scale);

        // println!("{:?}", uniforms);

        {
            let mut constants_buffer = staging_belt.write_buffer(
                encoder,
                &self.constants_buffer,
                0,
                wgpu::BufferSize::new(mem::size_of::<Uniforms>() as u64).unwrap(),
                device,
            );

            constants_buffer.copy_from_slice(bytemuck::bytes_of(&uniforms));
        }

        let instance_bytes = bytemuck::cast_slice(instances);

        let mut instance_buffer = staging_belt.write_buffer(
            encoder,
            &self.instance_buffer,
            0,
            wgpu::BufferSize::new(instance_bytes.len() as u64).unwrap(),
            device,
        );

        instance_buffer.copy_from_slice(instance_bytes);

        {
            let mut render_pass = super::begin_load_render_pass(encoder, &target);
            render_pass.set_pipeline(&self.render_pipeline);
            render_pass.set_bind_group(0, &self.constants, &[]);
            render_pass.set_index_buffer(self.index_buffer.slice(..));
            render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
            render_pass.set_vertex_buffer(1, self.instance_buffer.slice(..));
            render_pass.set_scissor_rect(
                bounds.x as u32,
                bounds.y as u32,
                bounds.width as u32,
                bounds.height as u32 + 1,
            );
            render_pass.draw_indexed(0..INDICES.len() as u32, 0, 0..instances.len() as u32);
        }
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
struct Uniforms {
    transform: [f32; 16],
    scale: f32,
}

impl Uniforms {
    fn new(translator: Mat4, scale: f32) -> Uniforms {
        Self {
            transform: *translator.as_ref(),
            scale,
        }
    }
}

unsafe impl bytemuck::Pod for Uniforms {}
unsafe impl bytemuck::Zeroable for Uniforms {}
