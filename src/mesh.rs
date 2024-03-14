use wgpu::util::DeviceExt;
use crate::vertex;
use crate::app;

pub struct Mesh<'a> {
    vertices: &'a [u8],
    indices: &'a [u16],
    vertex_type: vertex::VertexType,
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer
}

impl<'a> Mesh<'a> {
    pub fn new(
        app: &app::App,
        vertices: &'a [u8],
        vertex_type: vertex::VertexType,
        indices: &'a [u16]
    ) -> Self {
        let vertex_buffer = app.device().create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: None,
                contents: vertices,
                usage: wgpu::BufferUsages::VERTEX
            }
        );

        let index_buffer = app.device().create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: None,
                contents: bytemuck::cast_slice(indices),
                usage: wgpu::BufferUsages::INDEX
            }
        );

        Mesh {
            vertices,
            indices,
            vertex_type,
            vertex_buffer,
            index_buffer
        }
    }

    pub fn vertex_buffer(&self) -> &wgpu::Buffer {
        &self.vertex_buffer
    }

    pub fn index_buffer(&self) -> &wgpu::Buffer {
        &self.index_buffer
    }

    pub fn indices(&self) -> &[u16] {
        self.indices
    }
}
