use std::sync::Arc;
pub use wgpu::{BindGroupLayoutEntry, ShaderStages, BindingType, BufferBindingType, ShaderModuleDescriptor, PipelineLayoutDescriptor, RenderPipelineDescriptor, TextureViewDimension, TextureSampleType, SamplerBindingType, ShaderSource, PipelineCompilationOptions, FragmentState, ColorTargetState, BlendState, ColorWrites, PolygonMode, MultisampleState, PrimitiveState, StencilState, DepthStencilState, DepthBiasState};

pub struct ShaderModule {
    shader_module: Arc<wgpu::ShaderModule>,
}

impl ShaderModule {
    pub fn new(catengine: &CatEngine, shader_module_descriptor: ShaderModuleDescriptor) -> Self  {
        Self{
            shader_module: Arc::new(
                catengine.device.create_shader_module(shader_module_descriptor)
            )
        }
    }

    pub fn get_inner(&self) -> Arc<wgpu::ShaderModule> {
        self.shader_module.clone()
    }
}

pub struct PipelineLayout {
    pipeline_layout: Arc<wgpu::PipelineLayout>,
}

impl PipelineLayout {
    pub fn new(catengine: &CatEngine, pipeline_layout_descriptor: &PipelineLayoutDescriptor) -> Self  {
        Self {
            pipeline_layout: Arc::new(
                catengine.device.create_pipeline_layout(pipeline_layout_descriptor)
            )
        }
    }
    
    pub fn get_inner(&self) -> Arc<wgpu::PipelineLayout> { self.pipeline_layout.clone() }
}

pub struct RenderPipeline {
    render_pipeline: Arc<wgpu::RenderPipeline>,
}

impl RenderPipeline {
    pub fn new(catengine: &CatEngine, render_pipeline_descriptor: &RenderPipelineDescriptor) -> Self  {
        Self {
            render_pipeline: Arc::new(
                catengine.device.create_render_pipeline(render_pipeline_descriptor)
            )
        }
    }

    pub fn get_inner(&self) -> Arc<wgpu::RenderPipeline> { self.render_pipeline.clone() }
}

pub struct ShaderAttributes {
    pub shader_module_descriptor: ShaderModuleDescriptor<'static>,
    pub pipeline_layout_descriptor: PipelineLayoutDescriptor<'static>, 
    pub render_pipeline_descriptor: RenderPipelineDescriptor<'static>,
}

pub struct Shader {
    render_pipeline: RenderPipeline,
}

pub use wgpu::Face;
pub use wgpu::FrontFace;
pub use wgpu::PrimitiveTopology;
pub use wgpu::VertexBufferLayout;

use crate::CatEngine;

impl Shader {
    pub fn new(catengine: &CatEngine, render_pipeline_descriptor: &RenderPipelineDescriptor) -> Self {
        let render_pipeline = RenderPipeline::new(catengine, render_pipeline_descriptor);

        Self{ render_pipeline }
    }

    pub fn get_pipeline(&self) -> &RenderPipeline {
        &self.render_pipeline
    }
}
