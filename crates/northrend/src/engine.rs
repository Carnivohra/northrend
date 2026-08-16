use northrend_backend::{
    BackendApplication, BackendContext, WindowDescriptor, WindowId,
    window::{WindowEvent, event::WindowEventKind},
};
use northrend_render::Renderer;

pub(crate) struct Engine<R: Renderer> {
    main_window: Option<WindowId>,
    main_surface: Option<R::Surface>,
    renderer: R,
}

impl<R: Renderer> Engine<R> {
    pub(crate) fn new(renderer: R) -> Self {
        Self {
            main_window: None,
            main_surface: None,
            renderer,
        }
    }
}

impl<R: Renderer> BackendApplication for Engine<R> {
    fn started<C: BackendContext>(&mut self, context: &mut C) {
        let descriptor = WindowDescriptor::default();
        let width = descriptor.width;
        let height = descriptor.height;
        let window_id = context
            .create_window(descriptor)
            .expect("failed to create window");

        let window = context
            .window_handle(window_id)
            .expect("failed to get window handle");

        let surface = pollster::block_on(self.renderer.create_surface(window, width, height))
            .expect("failed to create render surface");

        self.main_window = Some(window_id);
        self.main_surface = Some(surface);
        context.request_redraw(window_id);
    }

    fn window_event<C: BackendContext>(&mut self, context: &mut C, event: WindowEvent) {
        if self.main_window != Some(event.window_id) {
            return;
        }

        match event.kind {
            WindowEventKind::CloseRequested => {
                self.main_surface = None;
                self.main_window = None;
                context.destroy_window(event.window_id);
                context.exit();
            }
            WindowEventKind::Resized { width, height } => {
                if let Some(surface) = self.main_surface.as_mut() {
                    self.renderer.resize(surface, width, height);
                }
            }
            WindowEventKind::RedrawRequested => {
                if let Some(surface) = self.main_surface.as_mut() {
                    self.renderer
                        .render(surface)
                        .expect("failed to render frame");

                    context.request_redraw(event.window_id);
                }
            }
        }
    }
}
