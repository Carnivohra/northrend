use wgpu::{Device, Surface, SurfaceConfiguration};

pub struct WgpuSurface {
    pub(crate) surface: Surface<'static>,
    pub(crate) configuration: SurfaceConfiguration,
    pub(crate) active: bool,
}

impl WgpuSurface {
    pub(crate) fn configure(&self, device: &Device) {
        self.surface.configure(device, &self.configuration);
    }
}
