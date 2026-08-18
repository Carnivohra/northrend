use std::sync::Arc;

use northrend_backend::{
    BackendApplication, DeviceEvent, WindowId,
    window::{WindowEvent, event::WindowEventKind},
};
use winit::{
    application::ApplicationHandler,
    event::{ElementState, MouseScrollDelta},
    event_loop::ActiveEventLoop,
    keyboard::PhysicalKey,
    window::{Window, WindowId as WinitWindowId},
};

use crate::input::{key_code, mouse_button};

use super::context::WinitBackendContext;

pub(super) struct WinitApplication<A> {
    application: A,
    windows: Vec<Option<Arc<Window>>>,
    started: bool,
}

impl<A> WinitApplication<A> {
    pub(super) fn new(application: A) -> Self {
        Self {
            application,
            windows: Vec::new(),
            started: false,
        }
    }
}

impl<A: BackendApplication> ApplicationHandler for WinitApplication<A> {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let mut context = WinitBackendContext::new(event_loop, &mut self.windows);

        if !self.started {
            self.application.started(&mut context);
            self.started = true;
        }

        self.application.resumed(&mut context);
    }

    fn suspended(&mut self, event_loop: &ActiveEventLoop) {
        let mut context = WinitBackendContext::new(event_loop, &mut self.windows);
        self.application.suspended(&mut context);
    }

    fn device_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _device_id: winit::event::DeviceId,
        event: winit::event::DeviceEvent,
    ) {
        let winit::event::DeviceEvent::MouseMotion { delta } = event else {
            return;
        };
        let mut context = WinitBackendContext::new(event_loop, &mut self.windows);
        let event = DeviceEvent::MouseMotion {
            horizontal: delta.0,
            vertical: delta.1,
        };

        self.application.device_event(&mut context, event);
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id: WinitWindowId,
        event: winit::event::WindowEvent,
    ) {
        let kind = match event {
            winit::event::WindowEvent::CloseRequested => WindowEventKind::CloseRequested,
            winit::event::WindowEvent::Resized(size) => WindowEventKind::Resized {
                width: size.width,
                height: size.height,
            },
            winit::event::WindowEvent::RedrawRequested => WindowEventKind::RedrawRequested,
            winit::event::WindowEvent::Focused(focused) => WindowEventKind::Focused(focused),
            winit::event::WindowEvent::KeyboardInput { event, .. } => {
                let PhysicalKey::Code(code) = event.physical_key else {
                    return;
                };
                let Some(key) = key_code(code) else {
                    return;
                };

                WindowEventKind::KeyboardInput {
                    key,
                    pressed: event.state == ElementState::Pressed,
                    repeat: event.repeat,
                }
            }
            winit::event::WindowEvent::MouseInput { state, button, .. } => {
                WindowEventKind::MouseInput {
                    button: mouse_button(button),
                    pressed: state == ElementState::Pressed,
                }
            }
            winit::event::WindowEvent::CursorMoved { position, .. } => {
                WindowEventKind::CursorMoved {
                    x: position.x,
                    y: position.y,
                }
            }
            winit::event::WindowEvent::CursorEntered { .. } => WindowEventKind::CursorEntered,
            winit::event::WindowEvent::CursorLeft { .. } => WindowEventKind::CursorLeft,
            winit::event::WindowEvent::MouseWheel { delta, .. } => match delta {
                MouseScrollDelta::LineDelta(horizontal, vertical) => {
                    WindowEventKind::MouseWheelLines {
                        horizontal,
                        vertical,
                    }
                }
                MouseScrollDelta::PixelDelta(delta) => WindowEventKind::MouseWheelPixels {
                    horizontal: delta.x,
                    vertical: delta.y,
                },
            },
            _ => return,
        };

        let Some(window_id) = self.windows.iter()
            .position(|window| {
                window.as_ref()
                    .is_some_and(|window| window.id() == window_id)
            })
            .and_then(|index| u64::try_from(index).ok())
            .and_then(|index| index.checked_add(1))
            .map(WindowId::new)
        else {
            return;
        };

        let mut context = WinitBackendContext::new(event_loop, &mut self.windows);
        let event = WindowEvent { window_id, kind };
        self.application.window_event(&mut context, event);
    }
}
