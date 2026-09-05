use std::sync::Arc;
use wgpu::BindGroupLayout;
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

pub struct ShaderAttributesBuilder {
    shader_module_label: Option<String>,
    shader_module_source: ShaderSource<'static>,

    render_pipeline_label: Option<String>,
    render_pipeline_bind_group_layouts: &[Option<BindGroupLayout>]
}

impl ShaderAttributesBuilder {
    pub fn default_3d(source: ShaderSource<'static>) -> Self {
        Self { 
            shader_module_label: Some("Shader Module".to_string()), 
            shader_module_source: source,

            render_pipeline_label: Some("Render Pipeline Layout".to_string()),
            render_pipeline_bind_group_layouts: vec![],
        }
    }

    pub fn set_shader_module_label(&mut self, shader_module_label: Option<String>) { self.shader_module_label = shader_module_label; } 
    pub fn set_shader_module_source(&mut self, shader_module_source: ShaderSource<'static>) { self.shader_module_source = shader_module_source; } 
    
    pub fn set_render_pipeline_label(&mut self, render_pipeline_label: Option<String>) { self.render_pipeline_label = render_pipeline_label; } 
    pub fn set_render_pipeline_bind_group_layouts(&mut self, render_pipeline_bind_group_layouts: Vec<Option<BindGroupLayout>>) { self.render_pipeline_bind_group_layouts = render_pipeline_bind_group_layouts; } 
}

impl Into<ShaderAttributes> for ShaderAttributesBuilder {
    fn into(self) -> ShaderAttributes {
        let shader_module_descriptor = ShaderModuleDescriptor {
            label: self.shader_module_label.as_deref(),
            source: self.shader_module_source,
        };

        let pipeline_layout_descriptor = &PipelineLayoutDescriptor {
            label: self.render_pipeline_label.as_deref(),
            bind_group_layouts: &self.render_pipeline_bind_group_layouts.into_boxed_slice(),
        };

        ShaderAttributes { shader_module_descriptor, pipeline_layout_descriptor: (), render_pipeline_descriptor: () }
    }
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
