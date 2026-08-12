pub trait BackendApplication {
    fn resumed(&mut self);
    fn suspended(&mut self);
    fn event(&mut self);
}
