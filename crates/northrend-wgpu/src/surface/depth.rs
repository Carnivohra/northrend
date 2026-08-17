use wgpu::{
    Device, Extent3d, Texture, TextureDescriptor, TextureDimension, TextureUsages, TextureView,
    TextureViewDescriptor,
};

use crate::pipeline::WgpuRenderPipeline;

pub(crate) struct WgpuDepthTexture {
    _texture: Texture,
    pub(crate) view: TextureView,
}

impl WgpuDepthTexture {
    pub(crate) fn new(device: &Device, width: u32, height: u32) -> Self {
        let texture = device.create_texture(&TextureDescriptor {
            label: Some("northrend-wgpu depth texture"),
            size: Extent3d {
                width: width.max(1),
                height: height.max(1),
                depth_or_array_layers: 1,
            },

            mip_level_count: 1,
            sample_count: 1,
            dimension: TextureDimension::D2,
            format: WgpuRenderPipeline::DEPTH_FORMAT,
            usage: TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        });

        let view = texture.create_view(&TextureViewDescriptor::default());

        Self {
            _texture: texture,
            view,
        }
    }
}
