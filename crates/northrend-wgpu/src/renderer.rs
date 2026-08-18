mod state;

use std::collections::HashMap;

use state::WgpuRendererState;

use northrend_backend::WindowHandle;
use northrend_render::{
    MaterialDescriptor, MaterialHandle, MeshData, MeshHandle, RenderFrame, Renderer,
    ShaderDescriptor, ShaderHandle,
};
use wgpu::{
    CommandEncoderDescriptor, CurrentSurfaceTexture, DeviceDescriptor, Instance, InstanceDescriptor, LoadOp, Operations, RenderPassColorAttachment, RenderPassDepthStencilAttachment, RenderPassDescriptor, RequestAdapterOptions, StoreOp, TextureViewDescriptor,
};

use crate::{
    WgpuError, WgpuSurface,
    camera::WgpuCamera,
    material::WgpuMaterialResource,
    mesh::WgpuMesh,
    shader::WgpuShaderResource,
};

pub struct WgpuRenderer {
    instance: Instance,
    state: Option<WgpuRendererState>,
}

impl WgpuRenderer {
    #[allow(clippy::new_without_default)]
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

            let camera = WgpuCamera::new(&device);

            self.state = Some(WgpuRendererState {
                adapter,
                device,
                queue,
                camera,
                meshes: Vec::new(),
                shaders: Vec::new(),
                materials: Vec::new(),
                surface_formats: Vec::new(),
                pipelines: HashMap::new(),
            });
        }

        let state = self.state.as_mut().expect("renderer state is initialized");

        if !state.adapter.is_surface_supported(&surface) {
            return Err(WgpuError::UnsupportedSurface);
        }

        let active = width > 0 && height > 0;
        let configuration = surface
            .get_default_config(&state.adapter, width.max(1), height.max(1))
            .ok_or(WgpuError::UnsupportedSurface)?;

        state.register_surface_format(configuration.format)?;
        let surface = WgpuSurface::new(surface, configuration, active, &state.device);

        Ok(surface)
    }

    fn create_shader(&mut self, shader: ShaderDescriptor<'_>) -> Result<ShaderHandle, Self::Error> {
        let state = self.state.as_mut().ok_or(WgpuError::RendererNotInitialized)?;
        let handle = ShaderHandle::from_index(state.shaders.len())
            .ok_or(WgpuError::ResourceCapacityExceeded)?;
        let shader = WgpuShaderResource::new(&state.device, shader);

        state.shaders.push(shader);
        Ok(handle)
    }

    fn create_material(
        &mut self,
        material: MaterialDescriptor,
    ) -> Result<MaterialHandle, Self::Error> {
        let state = self.state.as_mut().ok_or(WgpuError::RendererNotInitialized)?;

        if state.shaders.get(material.shader.index()).is_none() {
            return Err(WgpuError::InvalidShader);
        }

        let handle = MaterialHandle::from_index(state.materials.len())
            .ok_or(WgpuError::ResourceCapacityExceeded)?;
        state.materials.push(WgpuMaterialResource {
            shader: material.shader,
        });

        for index in 0..state.surface_formats.len() {
            let color_format = state.surface_formats[index];
            state.ensure_pipeline(handle, color_format)?;
        }

        Ok(handle)
    }

    fn create_mesh(&mut self, mesh: MeshData<'_>) -> Result<MeshHandle, Self::Error> {
        let state = self.state.as_mut().ok_or(WgpuError::RendererNotInitialized)?;
        let handle = MeshHandle::from_index(state.meshes.len())
            .ok_or(WgpuError::ResourceCapacityExceeded)?;
        let mesh = WgpuMesh::new(&state.device, mesh)?;

        state.meshes.push(mesh);
        Ok(handle)
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

    fn render(
        &mut self,
        surface: &mut Self::Surface,
        frame: &RenderFrame<'_>,
    ) -> Result<(), Self::Error> {
        if !surface.active {
            return Ok(());
        }

        let state = self.state.as_mut().ok_or(WgpuError::RendererNotInitialized)?;
        let color_format = surface.configuration.format;

        state.camera.prepare(
            &state.device,
            &state.queue,
            frame.views.iter()
                .map(|view| &view.camera.view_projection.columns),
        )?;

        let (surface_texture, reconfigure) = match surface.surface.get_current_texture() {
            CurrentSurfaceTexture::Success(surface_texture) => (surface_texture, false),
            CurrentSurfaceTexture::Suboptimal(surface_texture) => (surface_texture, true),
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

        let surface_view = surface_texture.texture.create_view(&TextureViewDescriptor::default());

        let mut encoder = state.device
            .create_command_encoder(&CommandEncoderDescriptor {
                label: Some("northrend-wgpu encoder"),
            });

        {
            let mut render_pass = encoder.begin_render_pass(&RenderPassDescriptor {
                label: Some("northrend-wgpu render pass"),
                color_attachments: &[Some(RenderPassColorAttachment {
                    view: &surface_view,
                    depth_slice: None,
                    resolve_target: None,
                    ops: Operations {
                        load: LoadOp::Clear(wgpu::Color {
                            r: f64::from(frame.clear_color.red),
                            g: f64::from(frame.clear_color.green),
                            b: f64::from(frame.clear_color.blue),
                            a: f64::from(frame.clear_color.alpha),
                        }),
                        store: StoreOp::Store,
                    },
                })],

                depth_stencil_attachment: Some(RenderPassDepthStencilAttachment {
                    view: &surface.depth.view,
                    depth_ops: Some(Operations {
                        load: LoadOp::Clear(0.0),
                        store: StoreOp::Discard,
                    }),
                    stencil_ops: None,
                }),
                timestamp_writes: None,
                occlusion_query_set: None,
                multiview_mask: None,
            });

            for (view_index, view) in frame.views.iter().enumerate() {
                let camera_offset = state.camera.offset(view_index)?;
                let mut bound_shader = None;

                for draw in view.draws {
                    let shader = state.material_shader(draw.material)?;
                    let pipeline = state.pipelines.get(&(shader, color_format))
                        .ok_or(WgpuError::InvalidMaterial)?;
                    let mesh = state.meshes.get(draw.mesh.index())
                        .ok_or(WgpuError::InvalidMesh)?;

                    if bound_shader != Some(shader) {
                        pipeline.bind(
                            &mut render_pass,
                            &state.camera.bind_group,
                            camera_offset,
                        );
                        bound_shader = Some(shader);
                    }

                    mesh.draw(&mut render_pass);
                }
            }
        }

        state.queue.submit([encoder.finish()]);
        state.queue.present(surface_texture);

        if reconfigure {
            surface.configure(&state.device);
        }

        Ok(())
    }
}
