use northrend_render::MeshData;
use wgpu::{
    Buffer, BufferAddress, Device, IndexFormat, RenderPass, VertexAttribute,
    VertexBufferLayout, VertexFormat, VertexStepMode,
    util::{BufferInitDescriptor, DeviceExt},
};

use crate::WgpuError;

pub(crate) struct WgpuMesh {
    vertex_buffer: Buffer,
    index_buffer: Buffer,
    index_count: u32,
}

impl WgpuMesh {
    const ATTRIBUTES: [VertexAttribute; 2] = [
        VertexAttribute {
            format: VertexFormat::Float32x3,
            offset: 0,
            shader_location: 0,
        },
        VertexAttribute {
            format: VertexFormat::Float32x4,
            offset: 12,
            shader_location: 1,
        },
    ];

    pub(crate) fn new(device: &Device, mesh: MeshData<'_>) -> Result<Self, WgpuError> {
        if mesh.vertices.is_empty()
            || mesh.indices.is_empty()
            || mesh.indices.iter().any(|index| *index as usize >= mesh.vertices.len())
        {
            return Err(WgpuError::InvalidMeshData);
        }

        let index_count = u32::try_from(mesh.indices.len())
            .map_err(|_| WgpuError::InvalidMeshData)?;

        let vertices: Vec<[f32; 7]> = mesh.vertices.iter()
            .map(|vertex| {
                [
                    vertex.position.x,
                    vertex.position.y,
                    vertex.position.z,
                    vertex.color.red,
                    vertex.color.green,
                    vertex.color.blue,
                    vertex.color.alpha,
                ]
            })
            .collect();

        let vertex_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: Some("northrend-wgpu vertex buffer"),
            contents: bytemuck::cast_slice(&vertices),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let index_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: Some("northrend-wgpu index buffer"),
            contents: bytemuck::cast_slice(mesh.indices),
            usage: wgpu::BufferUsages::INDEX,
        });

        Ok(Self {
            vertex_buffer,
            index_buffer,
            index_count,
        })
    }

    pub(crate) const fn layout() -> VertexBufferLayout<'static> {
        VertexBufferLayout {
            array_stride: std::mem::size_of::<[f32; 7]>() as BufferAddress,
            step_mode: VertexStepMode::Vertex,
            attributes: &Self::ATTRIBUTES,
        }
    }

    pub(crate) fn draw(&self, render_pass: &mut RenderPass<'_>) {
        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        render_pass.set_index_buffer(self.index_buffer.slice(..), IndexFormat::Uint32);
        render_pass.draw_indexed(0..self.index_count, 0, 0..1);
    }
}
