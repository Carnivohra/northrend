use northrend_backend::BackendApplication;

pub struct Engine;

impl Engine {
    pub fn new() -> Self {
        Self
    }
}

impl BackendApplication for Engine {
    fn resumed(&mut self) {
        todo!()
    }

    fn suspended(&mut self) {
        todo!()
    }

    fn event(&mut self) {
        todo!()
    }
}
