/*
    Stores the information about a single quad - the actual vertices
    are hard coded in the quad pipeline, but here we specify the per
    instance qualities of an individual quad, and then pass them into
    the vertex shader to enable instancing.
*/
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Quad {
    pub position: [f32; 2],
    pub color: [f32; 3],
    pub size: [f32; 2],
    pub border_colour: [f32; 3],
    pub border_width: f32,
}

impl Quad {
    pub fn desc<'a>() -> wgpu::VertexBufferDescriptor<'a> {
        wgpu::VertexBufferDescriptor {
            stride: std::mem::size_of::<Self>() as wgpu::BufferAddress,
            step_mode: wgpu::InputStepMode::Instance,
            attributes: &[
                wgpu::VertexAttributeDescriptor {
                    // Pos
                    offset: 0,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float2,
                },
                wgpu::VertexAttributeDescriptor {
                    // Color
                    offset: 2 * 4,
                    shader_location: 2,
                    format: wgpu::VertexFormat::Float3,
                },
                wgpu::VertexAttributeDescriptor {
                    // Scale
                    offset: 4 * (3 + 2),
                    shader_location: 3,
                    format: wgpu::VertexFormat::Float2,
                },
                // Border Color
                wgpu::VertexAttributeDescriptor {
                    offset: 4 * (3 + 2 + 2),
                    shader_location: 4,
                    format: wgpu::VertexFormat::Float3,
                },
                // Border width
                wgpu::VertexAttributeDescriptor {
                    offset: 4 * (3 + 2 + 2 + 3),
                    shader_location: 5,
                    format: wgpu::VertexFormat::Float,
                },
            ],
        }
    }
}

unsafe impl bytemuck::Pod for Quad {}
unsafe impl bytemuck::Zeroable for Quad {}
