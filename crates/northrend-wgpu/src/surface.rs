mod depth;

use depth::WgpuDepthTexture;

use wgpu::{Device, Surface, SurfaceConfiguration};

pub struct WgpuSurface {
    pub(crate) surface: Surface<'static>,
    pub(crate) configuration: SurfaceConfiguration,
    pub(crate) active: bool,
    pub(crate) depth: WgpuDepthTexture,
}

impl WgpuSurface {
    pub(crate) fn new(
        surface: Surface<'static>,
        configuration: SurfaceConfiguration,
        active: bool,
        device: &Device,
    ) -> Self {
        let depth = WgpuDepthTexture::new(
            device,
            configuration.width,
            configuration.height,
        );

        if active {
            surface.configure(device, &configuration);
        }

        Self {
            surface,
            configuration,
            active,
            depth,
        }
    }

    pub(crate) fn configure(&mut self, device: &Device) {
        self.surface.configure(device, &self.configuration);
        self.depth = WgpuDepthTexture::new(
            device,
            self.configuration.width,
            self.configuration.height,
        );
    }
}
