use crate::CatEngine;
use wgpu::{BindingResource, util::DeviceExt};

pub use wgpu::{BufferUsages, VertexStepMode, VertexFormat, VertexAttribute, BufferAddress, VertexBufferLayout};

pub struct Buffer {
    buffer: wgpu::Buffer,
}

impl Buffer {
    pub fn new(catengine: &CatEngine, contents: &[u8], label: Option<&str>, usage: BufferUsages) -> Self {
        let buffer = catengine.device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor{
                label,
                contents,
                usage,
            }
        );

        Self { buffer }
    }

    pub fn get_buffer(&self) -> &wgpu::Buffer {
        &self.buffer
    }

    pub fn as_entire_binding(&self) -> BindingResource<'_> {
        self.buffer.as_entire_binding()
    }
}
