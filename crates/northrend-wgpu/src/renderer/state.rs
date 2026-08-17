use std::collections::HashMap;

use northrend_render::{MaterialHandle, ShaderHandle};
use wgpu::{Adapter, Device, Queue, TextureFormat};

use crate::{
    WgpuError,
    camera::WgpuCamera,
    material::WgpuMaterialResource,
    mesh::WgpuMesh,
    pipeline::WgpuRenderPipeline,
    shader::WgpuShaderResource,
};

pub(super) struct WgpuRendererState {
    pub(super) adapter: Adapter,
    pub(super) device: Device,
    pub(super) queue: Queue,
    pub(super) camera: WgpuCamera,
    pub(super) meshes: Vec<WgpuMesh>,
    pub(super) shaders: Vec<WgpuShaderResource>,
    pub(super) materials: Vec<WgpuMaterialResource>,
    pub(super) pipelines: HashMap<(ShaderHandle, TextureFormat), WgpuRenderPipeline>,
}

impl WgpuRendererState {
    pub(super) fn material_shader(
        &self,
        material: MaterialHandle,
    ) -> Result<ShaderHandle, WgpuError> {
        self.materials.get(material.index())
            .map(|material| material.shader)
            .ok_or(WgpuError::InvalidMaterial)
    }

    pub(super) fn ensure_pipeline(
        &mut self,
        material: MaterialHandle,
        color_format: TextureFormat,
    ) -> Result<(), WgpuError> {
        let shader = self.material_shader(material)?;
        let key = (shader, color_format);

        if self.pipelines.contains_key(&key) {
            return Ok(());
        }

        let shader = self.shaders.get(shader.index())
            .ok_or(WgpuError::InvalidShader)?;
        let pipeline = WgpuRenderPipeline::new(
            &self.device,
            color_format,
            &self.camera.bind_group_layout,
            &shader.module,
        );

        self.pipelines.insert(key, pipeline);
        Ok(())
    }
}
