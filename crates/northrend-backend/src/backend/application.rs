use crate::{BackendContext, window::WindowEvent};

pub trait BackendApplication {
    fn started<C: BackendContext>(&mut self, context: &mut C);
    fn resumed<C: BackendContext>(&mut self, _context: &mut C) {}
    fn suspended<C: BackendContext>(&mut self, _context: &mut C) {}
    fn window_event<C: BackendContext>(&mut self, context: &mut C, event: WindowEvent);
}
