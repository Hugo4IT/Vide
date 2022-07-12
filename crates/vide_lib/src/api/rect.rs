use std::sync::MutexGuard;

use crate::{register_effect, effect::{Effect, EffectBackend}};

use super::{mesh::{Vertex, VertexAttributeDescriptor}, shader::Shader, color::Color, animation::AnimatedProperty, transform::OPENGL_TO_WGPU_MATRIX, instanced_mesh::InstancedMesh};

register_effect!(RectBackend, Rect);

pub struct Rect {
    pub position: AnimatedProperty<(f32, f32)>,
    pub size: AnimatedProperty<(f32, f32)>,
    pub color: AnimatedProperty<Color>,
}

#[repr(C)]
#[derive(Debug, Default, Clone, Copy)]
struct RectInstance {
    matrix: [[f32; 4]; 4],
    color: [f32; 4],
}

impl RectInstance {
    fn from_rect(rect: &Rect, frame: u64) -> Self {
        let position = rect.position.evaluate(frame);
        let size = rect.size.evaluate(frame);
        let color = rect.color.evaluate(frame);

        Self {
            matrix: (cgmath::Matrix4::from_translation(cgmath::Vector3::new(position.0, position.1, 0.0)) * cgmath::Matrix4::from_nonuniform_scale(size.0, size.1, 1.0) * OPENGL_TO_WGPU_MATRIX).into(),
            color: color.into(),
        }
    }
}

impl VertexAttributeDescriptor for RectInstance {
    fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<RectInstance>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: &[
                wgpu::VertexAttribute {
                    format: wgpu::VertexFormat::Float32x4,
                    offset: 0,
                    shader_location: 5,
                },
                wgpu::VertexAttribute {
                    format: wgpu::VertexFormat::Float32x4,
                    offset: std::mem::size_of::<[f32; 4]>() as wgpu::BufferAddress,
                    shader_location: 6,
                },
                wgpu::VertexAttribute {
                    format: wgpu::VertexFormat::Float32x4,
                    offset: std::mem::size_of::<[f32; 8]>() as wgpu::BufferAddress,
                    shader_location: 7,
                },
                wgpu::VertexAttribute {
                    format: wgpu::VertexFormat::Float32x4,
                    offset: std::mem::size_of::<[f32; 12]>() as wgpu::BufferAddress,
                    shader_location: 8,
                },
                wgpu::VertexAttribute {
                    format: wgpu::VertexFormat::Float32x4,
                    offset: std::mem::size_of::<[f32; 16]>() as wgpu::BufferAddress,
                    shader_location: 9,
                },
            ],
        }
    }
}

unsafe impl bytemuck::Pod for RectInstance {}
unsafe impl bytemuck::Zeroable for RectInstance {}

pub struct RectBackend {
    mesh: InstancedMesh<RectInstance>,
    instances: Vec<RectInstance>,
}

impl EffectBackend for RectBackend {
    type Instance = Rect;

    fn push(&mut self, instance: &Self::Instance, frame: u64) {
        self.instances.push(RectInstance::from_rect(instance, frame));
    }

    fn render<'a>(&'a mut self, pass: MutexGuard<wgpu::RenderPass<'a>>, device: &wgpu::Device, queue: &wgpu::Queue) {
        self.mesh.render(pass, device, queue, self.instances.drain(..).collect());
    }
}

impl Effect for RectBackend {
    fn new(renderer: &mut crate::render::Renderer) -> Self {
        let shader = Shader::new(renderer, include_str!("rect.wgsl").into());
        let mesh = InstancedMesh::new(
            renderer,
            vec![
                Vertex { position: [-0.5, -0.5], uv: [0.0, 1.0] },
                Vertex { position: [ 0.5, -0.5], uv: [1.0, 1.0] },
                Vertex { position: [-0.5,  0.5], uv: [0.0, 0.0] },
                Vertex { position: [ 0.5,  0.5], uv: [1.0, 0.0] },
            ],
            Some(vec![
                0, 1, 2,
                2, 1, 3,
            ]),
            shader,
        );

        Self {
            mesh,

            instances: Vec::new(),
        }
    }
}