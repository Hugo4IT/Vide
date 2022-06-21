use crate::render::Render;

use super::{mesh::{Mesh, Vertex}, shader::Shader};

#[derive(Debug)]
pub struct Rect {
    mesh: Mesh,
}

impl Rect {
    pub(crate) fn new(device: &wgpu::Device, config: wgpu::SurfaceConfiguration) -> Self {
        let mesh = Mesh::new(
            device,
            config,
            vec![
                Vertex { position: [0.0, 0.0, 0.0, 0.0], uv1_uv2: [0.0, 1.0, 0.0, 0.0], ..Default::default() },
                Vertex { position: [1.0, 0.0, 0.0, 0.0], uv1_uv2: [1.0, 1.0, 0.0, 0.0], ..Default::default() },
                Vertex { position: [0.0, 1.0, 0.0, 0.0], uv1_uv2: [0.0, 0.0, 0.0, 0.0], ..Default::default() },
                Vertex { position: [0.0, 1.0, 0.0, 0.0], uv1_uv2: [0.0, 0.0, 0.0, 0.0], ..Default::default() },
                Vertex { position: [1.0, 0.0, 0.0, 0.0], uv1_uv2: [1.0, 1.0, 0.0, 0.0], ..Default::default() },
                Vertex { position: [1.0, 1.0, 0.0, 0.0], uv1_uv2: [1.0, 0.0, 0.0, 0.0], ..Default::default() },
            ],
            None,
            Shader::new(device, include_str!("rect.wgsl").into()),
        );

        Self {
            mesh,
        }
    }
}

impl Render for Rect {
    fn render(&self, renderer: &mut crate::render::Renderer) {
        
    }
}