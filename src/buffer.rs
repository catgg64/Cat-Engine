use crate::CatEngine;
use wgpu::util::DeviceExt;

pub use wgpu::BufferUsages;

pub struct Buffer {
    buffer: wgpu::Buffer,
}

impl Buffer {
    pub fn new(engine: &CatEngine, contents: &[u8], label: Option<&str>, usage: BufferUsages) -> Self {
        let buffer = engine.device.create_buffer_init(
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
}

pub struct VertexBufferLayout {
    layout: wgpu::VertexBufferLayout<'static>,
}

//impl VertexBufferLayout {
//    
//}
