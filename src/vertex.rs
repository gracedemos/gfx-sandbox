use std::mem;

pub const TRIANGLE_COLOR: &[VertexColor] = &[
    VertexColor {position: [0.0, 0.5, 0.0], color: [1.0, 0.0, 0.0]},
    VertexColor {position: [-0.5, -0.5, 0.0], color: [0.0, 1.0, 0.0]},
    VertexColor {position: [0.5, -0.5, 0.0], color: [0.0, 0.0, 1.0]}
];

pub const TRIANGLE_COLOR_INDICES: &[u16] = &[
    0, 1, 2
];

pub const SQUARE_UV: &[VertexUV] = &[
    VertexUV {position: [-0.5, 0.5, 0.0], uv: [0.0, 0.0]},
    VertexUV {position: [0.5, 0.5, 0.0], uv: [1.0, 0.0]},
    VertexUV {position: [-0.5, -0.5, 0.0], uv: [0.0, 1.0]},
    VertexUV {position: [0.5, -0.5, 0.0], uv: [1.0, 1.0]}
];

pub const SQUARE_UV_INDICES: &[u16] = &[
    1, 0, 2,
    1, 2, 3
];

pub enum VertexType {
    VertexColor,
    VertexUV
}

#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct VertexColor {
    position: [f32; 3],
    color: [f32; 3]
}

#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct VertexUV {
    position: [f32; 3],
    uv: [f32; 2]
}

impl VertexType {
    pub fn desc(&self) -> wgpu::VertexBufferLayout<'static> {
        match *self {
            VertexType::VertexColor => vertex_color_desc(),
            VertexType::VertexUV => vertex_uv_desc()
        }
    }

}

fn vertex_color_desc() -> wgpu::VertexBufferLayout<'static> {
    wgpu::VertexBufferLayout {
        array_stride: mem::size_of::<VertexColor>() as u64,
        step_mode: wgpu::VertexStepMode::Vertex,
        attributes: &[
            wgpu::VertexAttribute {
                offset: 0,
                shader_location: 0,
                format: wgpu::VertexFormat::Float32x3
            },
            wgpu::VertexAttribute {
                offset: mem::size_of::<[f32; 3]>() as u64,
                shader_location: 1,
                format: wgpu::VertexFormat::Float32x3
            }
        ]
    }
}

fn vertex_uv_desc() -> wgpu::VertexBufferLayout<'static> {
    wgpu::VertexBufferLayout {
        array_stride: mem::size_of::<VertexUV>() as u64,
        step_mode: wgpu::VertexStepMode::Vertex,
        attributes: &[
            wgpu::VertexAttribute {
                offset: 0,
                shader_location: 0,
                format: wgpu::VertexFormat::Float32x3
            },
            wgpu::VertexAttribute {
                offset: mem::size_of::<[f32; 3]>() as u64,
                shader_location: 1,
                format: wgpu::VertexFormat::Float32x2
            }
        ]
    }
}
