use std::{error::Error, future::Future};

use northrend_backend::WindowHandle;

use crate::{
    MaterialDescriptor, MaterialHandle, MeshData, MeshHandle, RenderFrame, ShaderDescriptor,
    ShaderHandle,
};

pub trait Renderer {
    type Error: Error;
    type Surface;

    fn create_surface(
        &mut self,
        window: WindowHandle,
        width: u32,
        height: u32,
    ) -> impl Future<Output = Result<Self::Surface, Self::Error>>;

    fn create_shader(&mut self, shader: ShaderDescriptor<'_>) -> Result<ShaderHandle, Self::Error>;
    fn create_material(
        &mut self,
        material: MaterialDescriptor,
    ) -> Result<MaterialHandle, Self::Error>;
    fn create_mesh(&mut self, mesh: MeshData<'_>) -> Result<MeshHandle, Self::Error>;
    fn resize(&mut self, surface: &mut Self::Surface, width: u32, height: u32);
    fn render(
        &mut self,
        surface: &mut Self::Surface,
        frame: &RenderFrame<'_>,
    ) -> Result<(), Self::Error>;
}
