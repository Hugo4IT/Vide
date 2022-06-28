use std::borrow::Cow;

use crate::render::Renderer;

#[derive(Debug)]
pub struct Shader {
    pub module: wgpu::ShaderModule,
}

impl Shader {
    pub fn new(renderer: &mut Renderer, source: Cow<str>) -> Self {
        let module = renderer.wgpu_device().create_shader_module(&wgpu::ShaderModuleDescriptor {
            label: Some("Shader Module"),
            source: wgpu::ShaderSource::Wgsl(source),
        });

        Self {
            module,
        }
    }
}