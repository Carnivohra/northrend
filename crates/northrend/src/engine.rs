use northrend_backend::{BackendApplication, BackendContext, WindowDescriptor, WindowId};

pub struct Engine {
    main_window: Option<WindowId>,
}

impl Engine {
    pub fn new() -> Self {
        Self { main_window: None }
    }
}

impl BackendApplication for Engine {
    fn started<C: BackendContext>(&mut self, context: &mut C) {
        let window = context
            .create_window(WindowDescriptor::default())
            .expect("failed to create window");

        self.main_window = Some(window);
    }

    fn resumed<C: BackendContext>(&mut self, _context: &mut C) {
        todo!()
    }

    fn suspended<C: BackendContext>(&mut self, _context: &mut C) {
        todo!()
    }

    fn event(&mut self) {
        todo!()
    }
}
