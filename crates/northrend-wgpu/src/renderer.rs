mod state;

use state::WgpuRendererState;

use northrend_backend::WindowHandle;
use northrend_render::Renderer;
use wgpu::{
    Color, CommandEncoderDescriptor, CurrentSurfaceTexture, DeviceDescriptor, Instance, InstanceDescriptor, LoadOp, Operations, RenderPassColorAttachment, RenderPassDescriptor, RequestAdapterOptions, StoreOp, TextureViewDescriptor,
};

use crate::{WgpuError, WgpuSurface};

pub struct WgpuRenderer {
    instance: Instance,
    state: Option<WgpuRendererState>,
}

impl WgpuRenderer {
    pub fn new() -> Self {
        Self {
            instance: Instance::new(InstanceDescriptor::new_without_display_handle()),
            state: None,
        }
    }
}

impl Renderer for WgpuRenderer {
    type Error = WgpuError;
    type Surface = WgpuSurface;

    async fn create_surface(
        &mut self,
        window: WindowHandle,
        width: u32,
        height: u32,
    ) -> Result<Self::Surface, Self::Error> {
        let surface = self.instance.create_surface(window)?;

        if self.state.is_none() {
            let adapter = self.instance
                .request_adapter(&RequestAdapterOptions {
                    compatible_surface: Some(&surface),
                    ..Default::default()
                })
                .await?;

            let (device, queue) = adapter
                .request_device(&DeviceDescriptor {
                    label: Some("northrend-wgpu device"),
                    ..Default::default()
                })
                .await?;

            self.state = Some(WgpuRendererState {
                adapter,
                device,
                queue,
            });
        }

        let state = self.state.as_ref().expect("renderer state is initialized");

        if !state.adapter.is_surface_supported(&surface) {
            return Err(WgpuError::UnsupportedSurface);
        }

        let active = width > 0 && height > 0;
        let configuration = surface
            .get_default_config(&state.adapter, width.max(1), height.max(1))
            .ok_or(WgpuError::UnsupportedSurface)?;

        let surface = WgpuSurface {
            surface,
            configuration,
            active,
        };

        if surface.active {
            surface.configure(&state.device);
        }

        Ok(surface)
    }

    fn resize(&mut self, surface: &mut Self::Surface, width: u32, height: u32) {
        let state = self.state.as_ref().expect("renderer state is initialized");

        surface.active = width > 0 && height > 0;

        if !surface.active {
            return;
        }

        surface.configuration.width = width;
        surface.configuration.height = height;
        surface.configure(&state.device);
    }

    fn render(&mut self, surface: &mut Self::Surface) -> Result<(), Self::Error> {
        if !surface.active {
            return Ok(());
        }

        let state = self.state.as_ref().expect("renderer state is initialized");
        let (frame, reconfigure) = match surface.surface.get_current_texture() {
            CurrentSurfaceTexture::Success(frame) => (frame, false),
            CurrentSurfaceTexture::Suboptimal(frame) => (frame, true),
            CurrentSurfaceTexture::Timeout | CurrentSurfaceTexture::Occluded => return Ok(()),
            CurrentSurfaceTexture::Outdated => {
                surface.configure(&state.device);
                return Ok(());
            }
            CurrentSurfaceTexture::Lost => return Err(WgpuError::SurfaceLost),
            CurrentSurfaceTexture::Validation => {
                return Err(WgpuError::SurfaceValidationFailed);
            }
        };

        let view = frame.texture.create_view(&TextureViewDescriptor::default());
        let mut encoder = state.device
            .create_command_encoder(&CommandEncoderDescriptor {
                label: Some("northrend-wgpu encoder"),
            });

        {
            encoder.begin_render_pass(&RenderPassDescriptor {
                label: Some("northrend-wgpu clear pass"),
                color_attachments: &[Some(RenderPassColorAttachment {
                    view: &view,
                    depth_slice: None,
                    resolve_target: None,
                    ops: Operations {
                        load: LoadOp::Clear(Color {
                            r: 0.0,
                            g: 0.0,
                            b: 0.0,
                            a: 1.0,
                        }),
                        store: StoreOp::Store,
                    },
                })],

                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
                multiview_mask: None,
            });
        }

        state.queue.submit([encoder.finish()]);
        state.queue.present(frame);

        if reconfigure {
            surface.configure(&state.device);
        }

        Ok(())
    }
}
