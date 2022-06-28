use std::sync::MutexGuard;

use crate::{register_effect, effect::Effect};

use super::{mesh::{Mesh, Vertex}, shader::Shader, color::Color, animation::AnimatableProperty};

register_effect!(RectBackend, Rect);

pub struct Rect {
    pub position: AnimatableProperty<(f32, f32)>,
    pub size: AnimatableProperty<(f32, f32)>,
    pub color: AnimatableProperty<Color>,
}

pub struct RectBackend {
    mesh: Mesh,
}

impl RectBackend {
    fn render<'a>(&'a self, params: &Rect, pass: MutexGuard<wgpu::RenderPass<'a>>, frame: u64) {
        self.mesh.render(pass);
    }
}

impl Effect for RectBackend {
    fn new(renderer: &mut crate::render::Renderer) -> Self {
        let mesh = Mesh::new(
            renderer.wgpu_device(),
            renderer.wgpu_config(),
            vec![
                Vertex { position: [0.0, 0.0], uv: [0.0, 1.0] },
                Vertex { position: [1.0, 0.0], uv: [1.0, 1.0] },
                Vertex { position: [0.0, 1.0], uv: [0.0, 0.0] },
                Vertex { position: [1.0, 1.0], uv: [1.0, 0.0] },
            ],
            Some(vec![
                0, 1, 2,
                2, 1, 3,
            ]),
            Shader::new(renderer.wgpu_device(), include_str!("rect.wgsl").into()),
        );

        Self {
            mesh,
        }
    }
}