use northrend_render::{ShaderDescriptor, ShaderSource};
use wgpu::{Device, ShaderModule, ShaderModuleDescriptor, ShaderSource as WgpuShaderSource};

pub(crate) struct WgpuShaderResource {
    pub(crate) module: ShaderModule,
}

impl WgpuShaderResource {
    pub(crate) fn new(device: &Device, descriptor: ShaderDescriptor<'_>) -> Self {
        let source = match descriptor.source {
            ShaderSource::Wgsl(source) => WgpuShaderSource::Wgsl(source.into()),
        };

        let module = device.create_shader_module(ShaderModuleDescriptor {
            label: descriptor.label,
            source,
        });

        Self { module }
    }
}
