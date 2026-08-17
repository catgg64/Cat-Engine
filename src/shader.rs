use std::fmt;
use std::sync::Arc;
use wgpu::RenderPipeline;

pub use wgpu::{BindGroupLayoutEntry, ShaderStages, BindingType, BufferBindingType};

#[derive(Debug, Clone)]
pub struct GpuValidation;

impl fmt::Display for GpuValidation {
    fn fmt(&self, f: &mut fmt::Formatter) -> std::result::Result<(), fmt::Error> {
        Ok(write!(f, "Gpu validating")?)
    }
}

#[derive(Debug, Clone)]
pub struct GpuOutdated;

impl fmt::Display for GpuOutdated {
    fn fmt(&self, f: &mut fmt::Formatter) -> std::result::Result<(), fmt::Error> {
        Ok(write!(f, "Gpu outdated")?)
    }
}

#[derive(Debug, Clone)]
pub struct LostDevice;

impl fmt::Display for LostDevice {
    fn fmt(&self, f: &mut fmt::Formatter) -> std::result::Result<(), fmt::Error> {
        Ok(write!(f, "Lost device")?)
    }
}

#[derive(Debug, Clone)]
pub enum ShaderError {
    GpuValidation,
    GpuOutdated,
    LostDevice,
}

pub struct Shader {
    render_pipeline: RenderPipeline,
}

pub use wgpu::Face;
pub use wgpu::FrontFace;
pub use wgpu::PrimitiveTopology;
pub use wgpu::VertexBufferLayout;

impl Shader {
    pub fn new(location: &'static str, catengine: &mut crate::CatEngine, vertex_buffer_layouts: Option<&[Option<VertexBufferLayout>]>, vertex_function_name: &str, framgment_function_name: &str, topology: wgpu::PrimitiveTopology, front_face: wgpu::FrontFace, cull_mode: Option<wgpu::Face>, bind_group_layouts: &[Option<&crate::bindgroup::BindGroupLayout>]) -> Result<Shader, ShaderError> {
        let shader = catengine.device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Shader"),
            source: wgpu::ShaderSource::Wgsl(std::fs::read_to_string(location).unwrap().into()),
        });

        let render_pipeline_layout =
            catengine.device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),

                bind_group_layouts: bind_group_layouts,
                immediate_size: 0,
            }
        );

        let render_pipeline = catengine.device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some(vertex_function_name),
                buffers: {
                    match vertex_buffer_layouts {
                        Some(v) => v,
                        None => &[]
                    }
                },
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some(framgment_function_name),
                targets: &[Some(wgpu::ColorTargetState {
                    format: catengine.config.format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            }),
            primitive: wgpu::PrimitiveState {
                topology,
                strip_index_format: None,
                front_face,
                cull_mode,
                // Setting this to anything other than Fill requires Features::NON_FILL_POLYGON_MODE
                polygon_mode: wgpu::PolygonMode::Fill,
                // Requires Features::DEPTH_CLIP_CONTROL
                unclipped_depth: false,
                // Requires Features::CONSERVATIVE_RASTERIZATION
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview_mask: None,
            cache: None,
        });

        Ok(Self{ render_pipeline })
    }

    pub fn get_pipeline(&self) -> &RenderPipeline {
        &self.render_pipeline
    }
}
