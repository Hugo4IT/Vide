use std::{marker::PhantomData, sync::MutexGuard};

use wgpu::util::DeviceExt;

use crate::render::Renderer;

use super::{
    mesh::{Vertex, VertexAttributeDescriptor},
    shader::Shader,
};

#[derive(Debug)]
pub struct InstancedMesh<T: VertexAttributeDescriptor> {
    vertices: Vec<Vertex>,
    len_vertices: u32,
    indices: Option<Vec<u16>>,
    len_indices: u32,
    shader: Shader,

    vertex_buffer: wgpu::Buffer,
    index_buffer: Option<wgpu::Buffer>,
    instance_buffer: wgpu::Buffer,
    instance_buffer_len: usize,
    pipeline: wgpu::RenderPipeline,

    _phantom: PhantomData<T>,
}

impl<T: VertexAttributeDescriptor + bytemuck::Pod + bytemuck::Zeroable> InstancedMesh<T> {
    pub fn new(
        renderer: &mut Renderer,
        vertices: Vec<Vertex>,
        indices: Option<Vec<u16>>,
        shader: Shader,
    ) -> Self {
        let device = renderer.wgpu_device();
        let config = renderer.wgpu_config();

        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(&vertices[..]),
            usage: wgpu::BufferUsages::VERTEX,
        });
        let len_vertices = vertices.len() as u32;

        let (index_buffer, len_indices) = if let Some(indices) = indices.as_ref() {
            (
                Some(
                    device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                        label: Some("Index Buffer"),
                        contents: bytemuck::cast_slice(&indices[..]),
                        usage: wgpu::BufferUsages::INDEX,
                    }),
                ),
                indices.len() as u32,
            )
        } else {
            (None, 0)
        };

        let instance_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Mesh Instance Buffer"),
            size: std::mem::size_of::<T>() as wgpu::BufferAddress,
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Render Pipeline Layout"),
            bind_group_layouts: &[renderer.wgpu_transform_bind_group_layout()],
            push_constant_ranges: &[],
        });

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader.module,
                entry_point: "vs_main",
                buffers: &[Vertex::desc(), T::desc()],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader.module,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format: config.format,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil: Some(wgpu::DepthStencilState {
                format: wgpu::TextureFormat::Depth32Float,
                depth_write_enabled: false,
                depth_compare: wgpu::CompareFunction::Less,
                stencil: wgpu::StencilState::default(),
                bias: wgpu::DepthBiasState::default(),
            }),
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
        });

        Self {
            vertices,
            len_vertices,
            indices,
            len_indices,
            shader,
            vertex_buffer,
            index_buffer,
            instance_buffer,
            instance_buffer_len: 0,
            pipeline,
            _phantom: Default::default(),
        }
    }

    pub fn render<'a>(
        &'a mut self,
        mut render_pass: MutexGuard<wgpu::RenderPass<'a>>,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        instances: Vec<T>,
    ) {
        if self.instance_buffer_len != instances.len() {
            self.instance_buffer_len = instances.len();
            self.instance_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Instance Buffer"),
                contents: bytemuck::cast_slice(&instances[..]),
                usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            });
        }

        queue.write_buffer(
            &self.instance_buffer,
            0,
            bytemuck::cast_slice(&instances[..]),
        );

        if let Some(index_buffer) = self.index_buffer.as_ref() {
            render_pass.set_pipeline(&self.pipeline);
            render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
            render_pass.set_vertex_buffer(1, self.instance_buffer.slice(..));
            render_pass.set_index_buffer(index_buffer.slice(..), wgpu::IndexFormat::Uint16);
            render_pass.draw_indexed(0..self.len_indices, 0, 0..(self.instance_buffer_len as u32));
        } else {
            render_pass.set_pipeline(&self.pipeline);
            render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
            render_pass.set_vertex_buffer(1, self.instance_buffer.slice(..));
            render_pass.draw(0..self.len_vertices, 0..(self.instance_buffer_len as u32));
        }
    }
}
