use northrend_render::{ShaderDescriptor, ShaderSource};
use wgpu::{Device, ShaderModule, ShaderModuleDescriptor, ShaderSource as WgpuShaderSource};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct WgpuShader(usize);

impl WgpuShader {
    pub(crate) const fn new(index: usize) -> Self {
        Self(index)
    }

    pub(crate) const fn index(self) -> usize {
        self.0
    }
}

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
