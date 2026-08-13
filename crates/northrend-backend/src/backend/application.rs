use crate::BackendContext;

pub trait BackendApplication {
    fn started<C: BackendContext>(&mut self, context: &mut C);
    fn resumed<C: BackendContext>(&mut self, context: &mut C);
    fn suspended<C: BackendContext>(&mut self, context: &mut C);
    fn event(&mut self);
}
