use std::{error::Error, future::Future};

use northrend_backend::WindowHandle;

pub trait Renderer {
    type Error: Error;
    type Surface;

    fn create_surface(
        &mut self,
        window: WindowHandle,
        width: u32,
        height: u32,
    ) -> impl Future<Output = Result<Self::Surface, Self::Error>>;

    fn resize(&mut self, surface: &mut Self::Surface, width: u32, height: u32);
    fn render(&mut self, surface: &mut Self::Surface) -> Result<(), Self::Error>;
}
