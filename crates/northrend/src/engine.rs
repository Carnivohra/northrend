use northrend_backend::{
    BackendApplication, BackendContext, WindowDescriptor, WindowId,
    window::{WindowEvent, event::WindowEventKind},
};

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

    fn resumed<C: BackendContext>(&mut self, _context: &mut C) {}

    fn suspended<C: BackendContext>(&mut self, _context: &mut C) {}

    fn window_event<C: BackendContext>(&mut self, context: &mut C, event: WindowEvent) {
        if matches!(event.kind, WindowEventKind::CloseRequested) {
            context.destroy_window(event.window_id);

            if self.main_window == Some(event.window_id) {
                context.exit();
            }
        }
    }
}
