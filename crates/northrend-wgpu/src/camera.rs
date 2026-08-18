use wgpu::{
    BindGroup, BindGroupDescriptor, BindGroupEntry, BindGroupLayout, BindGroupLayoutDescriptor,
    BindGroupLayoutEntry, BindingResource, BindingType, Buffer, BufferBinding, BufferBindingType,
    BufferDescriptor, BufferSize, BufferUsages, Device, Queue, ShaderStages,
};

use crate::WgpuError;

pub(crate) struct WgpuCamera {
    buffer: Buffer,
    pub(crate) bind_group: BindGroup,
    pub(crate) bind_group_layout: BindGroupLayout,
    stride: u64,
    capacity: usize,
    data: Vec<u8>,
}

impl WgpuCamera {
    const UNIFORM_SIZE: u64 = 64;

    pub(crate) fn new(device: &Device) -> Self {
        let bind_group_layout = device.create_bind_group_layout(
            &BindGroupLayoutDescriptor {
                label: Some("northrend-wgpu camera bind group layout"),
                entries: &[BindGroupLayoutEntry {
                    binding: 0,
                    visibility: ShaderStages::VERTEX,
                    ty: BindingType::Buffer {
                        ty: BufferBindingType::Uniform,
                        has_dynamic_offset: true,
                        min_binding_size: BufferSize::new(Self::UNIFORM_SIZE),
                    },
                    count: None,
                }],
            },
        );

        let alignment = u64::from(device.limits().min_uniform_buffer_offset_alignment).max(1);
        let stride = Self::UNIFORM_SIZE.div_ceil(alignment) * alignment;
        let buffer = Self::create_buffer(device, stride);
        let bind_group = Self::create_bind_group(device, &bind_group_layout, &buffer);

        Self {
            buffer,
            bind_group,
            bind_group_layout,
            stride,
            capacity: 1,
            data: Vec::new(),
        }
    }

    pub(crate) fn prepare<'a>(
        &mut self,
        device: &Device,
        queue: &Queue,
        matrices: impl ExactSizeIterator<Item = &'a [[f32; 4]; 4]>,
    ) -> Result<(), WgpuError> {
        let count = matrices.len();

        if count == 0 {
            return Ok(());
        }

        if count > self.capacity {
            let capacity = count.checked_next_power_of_two()
                .ok_or(WgpuError::TooManyViews)?;
            let size = self.stride
                .checked_mul(u64::try_from(capacity).map_err(|_| WgpuError::TooManyViews)?)
                .ok_or(WgpuError::TooManyViews)?;

            if size > device.limits().max_buffer_size || size > u64::from(u32::MAX) {
                return Err(WgpuError::TooManyViews);
            }

            self.buffer = Self::create_buffer(device, size);
            self.bind_group = Self::create_bind_group(
                device,
                &self.bind_group_layout,
                &self.buffer,
            );
            self.capacity = capacity;
        }

        let size = self.stride
            .checked_mul(u64::try_from(count).map_err(|_| WgpuError::TooManyViews)?)
            .ok_or(WgpuError::TooManyViews)?;
        let size = usize::try_from(size).map_err(|_| WgpuError::TooManyViews)?;
        let stride = usize::try_from(self.stride).map_err(|_| WgpuError::TooManyViews)?;
        let uniform_size = usize::try_from(Self::UNIFORM_SIZE)
            .map_err(|_| WgpuError::TooManyViews)?;

        self.data.resize(size, 0);

        for (index, columns) in matrices.enumerate() {
            let offset = index * stride;
            self.data[offset..offset + uniform_size]
                .copy_from_slice(bytemuck::cast_slice(columns));
        }

        queue.write_buffer(&self.buffer, 0, &self.data);
        Ok(())
    }

    pub(crate) fn offset(&self, index: usize) -> Result<u32, WgpuError> {
        let index = u64::try_from(index).map_err(|_| WgpuError::TooManyViews)?;
        let offset = self.stride.checked_mul(index)
            .ok_or(WgpuError::TooManyViews)?;
        u32::try_from(offset).map_err(|_| WgpuError::TooManyViews)
    }

    fn create_buffer(device: &Device, size: u64) -> Buffer {
        device.create_buffer(&BufferDescriptor {
            label: Some("northrend-wgpu camera buffer"),
            size,
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
            mapped_at_creation: false,
        })
    }

    fn create_bind_group(
        device: &Device,
        layout: &BindGroupLayout,
        buffer: &Buffer,
    ) -> BindGroup {
        device.create_bind_group(&BindGroupDescriptor {
            label: Some("northrend-wgpu camera bind group"),
            layout,
            entries: &[BindGroupEntry {
                binding: 0,
                resource: BindingResource::Buffer(BufferBinding {
                    buffer,
                    offset: 0,
                    size: BufferSize::new(Self::UNIFORM_SIZE),
                }),
            }],
        })
    }
}
