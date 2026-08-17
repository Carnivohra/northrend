use wgpu::{
    BindGroup, BindGroupLayout, BlendState, ColorTargetState, ColorWrites, CompareFunction,
    DepthStencilState, Device, Face, FragmentState, IndexFormat, PipelineLayoutDescriptor,
    PrimitiveState, RenderPass, RenderPipeline, RenderPipelineDescriptor, ShaderModule,
    StencilState, TextureFormat, VertexState,
};

use crate::WgpuMesh;

pub(crate) struct WgpuRenderPipeline {
    pipeline: RenderPipeline,
}

impl WgpuRenderPipeline {
    pub(crate) const DEPTH_FORMAT: TextureFormat = TextureFormat::Depth32Float;

    pub(crate) fn new(
        device: &Device,
        color_format: TextureFormat,
        camera_bind_group_layout: &BindGroupLayout,
        shader: &ShaderModule,
    ) -> Self {
        let layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label: Some("northrend-wgpu render pipeline layout"),
            bind_group_layouts: &[Some(camera_bind_group_layout)],
            immediate_size: 0,
        });

        let pipeline = device.create_render_pipeline(&RenderPipelineDescriptor {
            label: Some("northrend-wgpu render pipeline"),
            layout: Some(&layout),
            vertex: VertexState {
                module: shader,
                entry_point: Some("vertex_main"),
                compilation_options: Default::default(),
                buffers: &[Some(WgpuMesh::layout())],
            },
            primitive: PrimitiveState {
                cull_mode: Some(Face::Back),
                ..Default::default()
            },
            depth_stencil: Some(DepthStencilState {
                format: Self::DEPTH_FORMAT,
                depth_write_enabled: Some(true),
                depth_compare: Some(CompareFunction::GreaterEqual),
                stencil: StencilState::default(),
                bias: Default::default(),
            }),
            multisample: Default::default(),
            fragment: Some(FragmentState {
                module: shader,
                entry_point: Some("fragment_main"),
                compilation_options: Default::default(),
                targets: &[Some(ColorTargetState {
                    format: color_format,
                    blend: Some(BlendState::REPLACE),
                    write_mask: ColorWrites::ALL,
                })],
            }),
            multiview_mask: None,
            cache: None,
        });

        Self { pipeline }
    }

    pub(crate) fn bind(
        &self,
        render_pass: &mut RenderPass<'_>,
        camera: &BindGroup,
        camera_offset: u32,
    ) {
        render_pass.set_pipeline(&self.pipeline);
        render_pass.set_bind_group(0, camera, &[camera_offset]);
    }

    pub(crate) fn draw(&self, render_pass: &mut RenderPass<'_>, mesh: &WgpuMesh) {
        render_pass.set_vertex_buffer(0, mesh.vertex_buffer.slice(..));
        render_pass.set_index_buffer(mesh.index_buffer.slice(..), IndexFormat::Uint32);
        render_pass.draw_indexed(0..mesh.index_count, 0, 0..1);
    }
}
