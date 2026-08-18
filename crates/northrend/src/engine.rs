use std::time::Instant;

use northrend_assets::AssetLibrary;
use northrend_backend::{
    BackendApplication, BackendContext, DeviceEvent, WindowDescriptor,
    window::{WindowEvent, event::WindowEventKind},
};
use northrend_game::Game;
use northrend_input::InputState;
use northrend_render::{Camera, Color, RenderFrame, RenderScene, RenderView, Renderer};
use northrend_world::{TerrainTile, TerrainTileCoordinate, World};

use crate::window::WindowTarget;

pub(crate) struct Engine<R: Renderer, G: Game> {
    main_window: Option<WindowTarget<R::Surface>>,
    render_scene: RenderScene,
    camera: Option<Camera>,
    input: InputState,
    last_tick: Instant,
    aspect_ratio: f32,
    _assets: AssetLibrary,
    world: World,
    renderer: R,
    game: G,
}

impl<R: Renderer, G: Game> Engine<R, G> {
    pub(crate) fn new(renderer: R, game: G) -> Self {
        let assets = AssetLibrary::open("assets", game.base_archive_order())
            .expect("failed to open asset library");
        let map = game.terrain_map();
        let wdt_path = format!("World\\Maps\\{map}\\{map}.wdt");
        let wdt = assets.read_wdt(&wdt_path)
            .expect("failed to read terrain map");
        let mut world = World::new();

        for tile in wdt.existing_tiles() {
            let coordinate = tile.coordinate();
            let path = format!(
                "World\\Maps\\{map}\\{map}_{}_{}.adt",
                coordinate.x,
                coordinate.y,
            );
            let adt = assets.read_adt(&path)
                .expect("failed to read terrain tile");
            let coordinate = TerrainTileCoordinate::new(
                i32::from(coordinate.x),
                i32::from(coordinate.y),
            );
            let tile = TerrainTile::from_adt(&adt)
                .expect("failed to build terrain tile");

            world.terrain_mut().insert(coordinate, tile);
        }

        assert!(!world.terrain().is_empty(), "terrain map contains no tiles");

        Self {
            main_window: None,
            render_scene: RenderScene::new(),
            camera: None,
            input: InputState::default(),
            last_tick: Instant::now(),
            aspect_ratio: 16.0 / 9.0,
            _assets: assets,
            world,
            renderer,
            game,
        }
    }

    fn tick<C: BackendContext>(&mut self, context: &C) {
        if self.main_window.is_none() {
            return;
        }

        let now = Instant::now();
        let delta_time = now.duration_since(self.last_tick).as_secs_f32().min(0.1);
        self.last_tick = now;

        self.game.tick(&self.input, delta_time);
        self.camera = Some(self.game.camera(self.aspect_ratio));

        let Some(camera) = &self.camera else {
            return;
        };
        let target = self.main_window.as_mut()
            .expect("main window is initialized");

        let views = [RenderView::new(camera, self.render_scene.draws())];
        let frame = RenderFrame::new(Color::new(0.38, 0.62, 0.82, 1.0), &views);
        self.renderer.render(&mut target.surface, &frame)
            .expect("failed to render frame");

        self.input.end_tick();
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
        self.aspect_ratio = width as f32 / height as f32;
        let window = context
            .create_window(descriptor)
            .expect("failed to create window");

        let surface = pollster::block_on(self.renderer.create_surface(window.handle(), width, height))
            .expect("failed to create render surface");
        for (_, tile) in self.world.terrain().tiles() {
            self.render_scene.load_terrain(&mut self.renderer, tile)
                .expect("failed to create terrain render resources");
        }
        self.camera = Some(self.game.camera(self.aspect_ratio));
        self.last_tick = Instant::now();

        self.main_window = Some(WindowTarget::new(window, surface));

        if let Some(target) = self.main_window.as_ref() {
            context.request_redraw(&target.window);
        }
    }

    fn device_event<C: BackendContext>(&mut self, _context: &mut C, event: DeviceEvent) {
        event.update_input(&mut self.input);
    }

    fn window_event<C: BackendContext>(&mut self, context: &mut C, event: WindowEvent) {
        let Some(target) = self.main_window.as_ref() else {
            return;
        };

        if target.window.id() != event.window_id {
            return;
        }

        event.kind.update_input(&mut self.input);

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

                if width > 0 && height > 0 {
                    self.aspect_ratio = width as f32 / height as f32;
                    self.camera = Some(self.game.camera(self.aspect_ratio));
                }
            }
            WindowEventKind::RedrawRequested => self.tick(context),
            _ => {}
        }
    }
}
