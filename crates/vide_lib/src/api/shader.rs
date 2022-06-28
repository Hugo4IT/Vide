use std::borrow::Cow;

#[derive(Debug)]
pub struct Shader {
    pub module: wgpu::ShaderModule,
}

impl Shader {
    pub fn new(device: &wgpu::Device, source: Cow<str>) -> Self {
        let module = device.create_shader_module(&wgpu::ShaderModuleDescriptor {
            label: Some("Shader Module"),
            source: wgpu::ShaderSource::Wgsl(source),
        });

        Self {
            module,
        }
    }
}