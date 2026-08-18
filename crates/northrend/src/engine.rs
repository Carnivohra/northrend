use northrend_backend::{
    BackendApplication, BackendContext, WindowDescriptor,
    window::{WindowEvent, event::WindowEventKind},
};
use northrend_game::Game;
use northrend_render::{Color, RenderFrame, Renderer};

use crate::window::WindowTarget;

pub(crate) struct Engine<R: Renderer, G: Game> {
    main_window: Option<WindowTarget<R::Surface>>,
    renderer: R,
    game: G,
}

impl<R: Renderer, G: Game> Engine<R, G> {
    pub(crate) fn new(renderer: R, game: G) -> Self {
        Self {
            main_window: None,
            renderer,
            game,
        }
    }

    fn tick<C: BackendContext>(&mut self, context: &C) {
        let Some(target) = self.main_window.as_mut() else {
            return;
        };

        let frame = RenderFrame::new(Color::BLACK, &[]);
        self.renderer.render(&mut target.surface, &frame)
            .expect("failed to render frame");

        context.request_redraw(&target.window);
    }
}

impl<R: Renderer, G: Game> BackendApplication for Engine<R, G> {
    fn started<C: BackendContext>(&mut self, context: &mut C) {
        let descriptor = WindowDescriptor {
            title: self.game.name().to_owned(),
            ..Default::default()
        };

        let width = descriptor.width;
        let height = descriptor.height;
        let window = context
            .create_window(descriptor)
            .expect("failed to create window");

        let surface = pollster::block_on(self.renderer.create_surface(window.handle(), width, height))
            .expect("failed to create render surface");

        self.main_window = Some(WindowTarget::new(window, surface));

        if let Some(target) = self.main_window.as_ref() {
            context.request_redraw(&target.window);
        }
    }

    fn window_event<C: BackendContext>(&mut self, context: &mut C, event: WindowEvent) {
        let Some(target) = self.main_window.as_ref() else {
            return;
        };

        if target.window.id() != event.window_id {
            return;
        }

        match event.kind {
            WindowEventKind::CloseRequested => {
                let target = self.main_window.take()
                    .expect("main window is initialized");

                let WindowTarget { window, surface } = target;
                drop(surface);
                context.destroy_window(window);
                context.exit();
            }
            WindowEventKind::Resized { width, height } => {
                if let Some(target) = self.main_window.as_mut() {
                    self.renderer.resize(&mut target.surface, width, height);
                }
            }
            WindowEventKind::RedrawRequested => self.tick(context),
        }
    }
}
