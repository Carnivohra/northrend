use wgpu::{Adapter, Device, Queue};

pub(super) struct WgpuRendererState {
    pub(super) adapter: Adapter,
    pub(super) device: Device,
    pub(super) queue: Queue,
}
