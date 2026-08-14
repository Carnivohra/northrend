pub trait Renderer {
    type Error;

    fn resize(&mut self, width: u32, height: u32);
    fn render(&mut self) -> Result<(), Self::Error>;
}
