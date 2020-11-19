/*
    Stores the information about a single quad - the actual vertices
    are hard coded in the quad pipeline, but here we specify the per
    instance qualities of an individual quad, and then pass them into
    the vertex shader to enable instancing.
*/
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct TriangleInstance {
    pub position: [f32; 2],
    pub color: [f32; 3],
}

impl TriangleInstance {
    pub fn desc<'a>() -> wgpu::VertexBufferDescriptor<'a> {
        wgpu::VertexBufferDescriptor {
            stride: std::mem::size_of::<Self>() as wgpu::BufferAddress,
            step_mode: wgpu::InputStepMode::Instance,
            attributes: &[
                wgpu::VertexAttributeDescriptor {
                    offset: 0,
                    shader_location: 2,
                    format: wgpu::VertexFormat::Float2,
                },
                wgpu::VertexAttributeDescriptor {
                    offset: 2 * 4,
                    shader_location: 3,
                    format: wgpu::VertexFormat::Float3,
                },
            ],
        }
    }
}

unsafe impl bytemuck::Pod for TriangleInstance {}
unsafe impl bytemuck::Zeroable for TriangleInstance {}
