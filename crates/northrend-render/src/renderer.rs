use std::{error::Error, future::Future};

use northrend_backend::WindowHandle;

use crate::{MaterialDescriptor, MeshData, RenderFrame, ShaderDescriptor};

pub trait Renderer {
    type Error: Error;
    type Material: Copy;
    type Mesh;
    type Shader: Copy;
    type Surface;

    fn create_surface(
        &mut self,
        window: WindowHandle,
        width: u32,
        height: u32,
    ) -> impl Future<Output = Result<Self::Surface, Self::Error>>;

    fn create_shader(&mut self, shader: ShaderDescriptor<'_>) -> Result<Self::Shader, Self::Error>;
    fn create_material(
        &mut self,
        material: MaterialDescriptor<Self::Shader>,
    ) -> Result<Self::Material, Self::Error>;
    fn create_mesh(&mut self, mesh: MeshData<'_>) -> Result<Self::Mesh, Self::Error>;
    fn resize(&mut self, surface: &mut Self::Surface, width: u32, height: u32);
    fn render(
        &mut self,
        surface: &mut Self::Surface,
        frame: &RenderFrame<'_, Self::Mesh, Self::Material>,
    ) -> Result<(), Self::Error>;
}
