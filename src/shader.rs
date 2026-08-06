use std::fmt;
use std::ops::Range;

use anyhow::anyhow;
use wgpu::{CommandEncoder, RenderPass};
use wgpu::{RenderPipeline, ShaderModule};

pub enum ShaderError {
    GpuValidation,
    GpuOutdated,
    LostDevice,
}

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

pub struct Shader {
    render_pipeline: RenderPipeline,
}

pub use wgpu::Face;
pub use wgpu::FrontFace;
pub use wgpu::PrimitiveTopology;

use crate::CatEngine;

impl Shader {
    pub fn new(location: &'static str, engine: &crate::CatEngine, vertex_function_name: &str, framgment_function_name: &str, topology: wgpu::PrimitiveTopology, front_face: wgpu::FrontFace, cull_mode: Option<wgpu::Face>) -> Self {
        
        let shader = engine.device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Shader"),
            source: wgpu::ShaderSource::Wgsl(std::fs::read_to_string(location).unwrap().into()),
        });

        let render_pipeline_layout =
            engine.device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: &[],
                immediate_size: 0,
            }
        );

        let render_pipeline = engine.device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some(vertex_function_name),
                buffers: &[],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some(framgment_function_name),
                targets: &[Some(wgpu::ColorTargetState {
                    format: engine.config.format,
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



        Self { render_pipeline }
    }

    pub fn draw(&mut self, engine: &CatEngine, vertices: Range<u32>, instances: Range<u32>) -> Result<(), ShaderError> {
        let output = match engine.surface.get_current_texture() {
                wgpu::CurrentSurfaceTexture::Success(surface_texture) => surface_texture,
                wgpu::CurrentSurfaceTexture::Suboptimal(surface_texture) => {
                    surface_texture
                }
                wgpu::CurrentSurfaceTexture::Timeout
                | wgpu::CurrentSurfaceTexture::Occluded
                | wgpu::CurrentSurfaceTexture::Validation => {
                    // Skip this frame
                    return Err(ShaderError::GpuValidation);
                }
                wgpu::CurrentSurfaceTexture::Outdated => {
                    engine.surface.configure(&engine.device, &engine.config);
                    return Err(ShaderError::GpuValidation);
                }
                wgpu::CurrentSurfaceTexture::Lost => {
                    // You could recreate the devices and all resources
                    // created with it here, but we'll just bail
                    return Err(ShaderError::LostDevice);
                }
            };

        let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = engine.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        });  


        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Render Pass"),
            color_attachments: &[
                // This is what @location(0) in the fragment shader targets
                Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    depth_slice: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(
                            wgpu::Color {
                                r: 0.1,
                                g: 0.2,
                                b: 0.3,
                                a: 1.0,
                            }
                        ),
                        store: wgpu::StoreOp::Store,
                    }
                })
            ],
            depth_stencil_attachment: None,
            occlusion_query_set: None,
            timestamp_writes: None,
            multiview_mask: None,
        });

        render_pass.set_pipeline(&self.render_pipeline);
        render_pass.draw(vertices, instances);

        Ok(())
    }


    pub fn get_pipeline(&self) -> &RenderPipeline {
        &self.render_pipeline
    }
}
