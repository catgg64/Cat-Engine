use crate::CatEngine;
use wgpu::util::DeviceExt;

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
}

//pub struct VertexBufferLayout {
//    layout: wgpu::VertexBufferLayout<'static>,
//}

//impl VertexBufferLayout {
//    pub fn new(array_stride: usize, step_mode: VertexStepMode, attributes: Vec<VertexAttribute>) -> Self {
//        let attributes: &'static [VertexAttribute] = Box::leak(attributes.into_boxed_slice());0
//
//        let layout = wgpu::VertexBufferLayout {
//            array_stride: array_stride as u64,
//            step_mode,
//            attributes,
//        };
//        Self { layout }
//    }    

//    pub fn get_layout(&self) -> &wgpu::VertexBufferLayout<'static> {
//        &self.layout
//    }
//}
