use std::{borrow::Cow, collections::HashMap};

use crate::render::Renderer;

#[derive(Debug, Clone, Copy)]
pub enum ShaderValue {
    /// Two unsigned bytes (u8). `uvec2` in shaders.
    Uint8x2([u8; 2]),
    /// Four unsigned bytes (u8). `uvec4` in shaders.
    Uint8x4([u8; 4]),
    /// Two signed bytes (i8). `ivec2` in shaders.
    Sint8x2([i8; 2]),
    /// Four signed bytes (i8). `ivec4` in shaders.
    Sint8x4([i8; 4]),
    /// Two unsigned bytes (u8). [0, 255] converted to float [0, 1] `vec2` in shaders.
    Unorm8x2([u8; 2]),
    /// Four unsigned bytes (u8). [0, 255] converted to float [0, 1] `vec4` in shaders.
    Unorm8x4([u8; 4]),
    /// Two signed bytes (i8). [-127, 127] converted to float [-1, 1] `vec2` in shaders.
    Snorm8x2([i8; 2]),
    /// Four signed bytes (i8). [-127, 127] converted to float [-1, 1] `vec4` in shaders.
    Snorm8x4([i8; 4]),
    /// Two unsigned shorts (u16). `uvec2` in shaders.
    Uint16x2([u16; 2]),
    /// Four unsigned shorts (u16). `uvec4` in shaders.
    Uint16x4([u16; 4]),
    /// Two signed shorts (i16). `ivec2` in shaders.
    Sint16x2([i16; 2]),
    /// Four signed shorts (i16). `ivec4` in shaders.
    Sint16x4([i16; 4]),
    /// Two unsigned shorts (u16). [0, 65535] converted to float [0, 1] `vec2` in shaders.
    Unorm16x2([u16; 2]),
    /// Four unsigned shorts (u16). [0, 65535] converted to float [0, 1] `vec4` in shaders.
    Unorm16x4([u16; 4]),
    /// Two signed shorts (i16). [-32767, 32767] converted to float [-1, 1] `vec2` in shaders.
    Snorm16x2([i16; 2]),
    /// Four signed shorts (i16). [-32767, 32767] converted to float [-1, 1] `vec4` in shaders.
    Snorm16x4([i16; 4]),
    /// One single-precision float (f32). `float` in shaders.
    Float32(f32),
    /// Two single-precision floats (f32). `vec2` in shaders.
    Float32x2([f32; 2]),
    /// Three single-precision floats (f32). `vec3` in shaders.
    Float32x3([f32; 3]),
    /// Four single-precision floats (f32). `vec4` in shaders.
    Float32x4([f32; 4]),
    /// One unsigned int (u32). `uint` in shaders.
    Uint32(u32),
    /// Two unsigned ints (u32). `uvec2` in shaders.
    Uint32x2([u32; 2]),
    /// Three unsigned ints (u32). `uvec3` in shaders.
    Uint32x3([u32; 3]),
    /// Four unsigned ints (u32). `uvec4` in shaders.
    Uint32x4([u32; 4]),
    /// One signed int (i32). `int` in shaders.
    Sint32(i32),
    /// Two signed ints (i32). `ivec2` in shaders.
    Sint32x2([i32; 2]),
    /// Three signed ints (i32). `ivec3` in shaders.
    Sint32x3([i32; 3]),
    /// Four signed ints (i32). `ivec4` in shaders.
    Sint32x4([i32; 4]),
    // /// One double-precision float (f64). `double` in shaders. Requires VERTEX_ATTRIBUTE_64BIT features.
    // Float64(f64),
    // /// Two double-precision floats (f64). `dvec2` in shaders. Requires VERTEX_ATTRIBUTE_64BIT features.
    // Float64x2([f64; 2]),
    // /// Three double-precision floats (f64). `dvec3` in shaders. Requires VERTEX_ATTRIBUTE_64BIT features.
    // Float64x3([f64; 3]),
    // /// Four double-precision floats (f64). `dvec4` in shaders. Requires VERTEX_ATTRIBUTE_64BIT features.
    // Float64x4([f64; 4]),
}

impl ShaderValue {
    fn wgpu_equivalent(&self) -> wgpu::VertexFormat {
        match self {
              ShaderValue::Uint8x2(_) => wgpu::VertexFormat::Uint8x2,
              ShaderValue::Uint8x4(_) => wgpu::VertexFormat::Uint8x4,
              ShaderValue::Sint8x2(_) => wgpu::VertexFormat::Sint8x2,
              ShaderValue::Sint8x4(_) => wgpu::VertexFormat::Sint8x4,
             ShaderValue::Unorm8x2(_) => wgpu::VertexFormat::Unorm8x2,
             ShaderValue::Unorm8x4(_) => wgpu::VertexFormat::Unorm8x4,
             ShaderValue::Snorm8x2(_) => wgpu::VertexFormat::Snorm8x2,
             ShaderValue::Snorm8x4(_) => wgpu::VertexFormat::Snorm8x4,
             ShaderValue::Uint16x2(_) => wgpu::VertexFormat::Uint16x2,
             ShaderValue::Uint16x4(_) => wgpu::VertexFormat::Uint16x4,
             ShaderValue::Sint16x2(_) => wgpu::VertexFormat::Sint16x2,
             ShaderValue::Sint16x4(_) => wgpu::VertexFormat::Sint16x4,
            ShaderValue::Unorm16x2(_) => wgpu::VertexFormat::Unorm16x2,
            ShaderValue::Unorm16x4(_) => wgpu::VertexFormat::Unorm16x4,
            ShaderValue::Snorm16x2(_) => wgpu::VertexFormat::Snorm16x2,
            ShaderValue::Snorm16x4(_) => wgpu::VertexFormat::Snorm16x4,
              ShaderValue::Float32(_) => wgpu::VertexFormat::Float32,
            ShaderValue::Float32x2(_) => wgpu::VertexFormat::Float32x2,
            ShaderValue::Float32x3(_) => wgpu::VertexFormat::Float32x3,
            ShaderValue::Float32x4(_) => wgpu::VertexFormat::Float32x4,
               ShaderValue::Uint32(_) => wgpu::VertexFormat::Uint32,
             ShaderValue::Uint32x2(_) => wgpu::VertexFormat::Uint32x2,
             ShaderValue::Uint32x3(_) => wgpu::VertexFormat::Uint32x3,
             ShaderValue::Uint32x4(_) => wgpu::VertexFormat::Uint32x4,
               ShaderValue::Sint32(_) => wgpu::VertexFormat::Sint32,
             ShaderValue::Sint32x2(_) => wgpu::VertexFormat::Sint32x2,
             ShaderValue::Sint32x3(_) => wgpu::VertexFormat::Sint32x3,
             ShaderValue::Sint32x4(_) => wgpu::VertexFormat::Sint32x4,
        }
    }

    fn size(&self) -> u64 {
        self.wgpu_equivalent().size()
    }

    #[rustfmt::skip]
    fn is_same_type(&self, other: &ShaderValue) -> bool {
        match (self, other) {
              (  ShaderValue::Uint8x2(_), ShaderValue::Uint8x2(_))
            | (  ShaderValue::Uint8x4(_), ShaderValue::Uint8x4(_))
            | (  ShaderValue::Sint8x2(_), ShaderValue::Sint8x2(_))
            | (  ShaderValue::Sint8x4(_), ShaderValue::Sint8x4(_))
            | ( ShaderValue::Unorm8x2(_), ShaderValue::Unorm8x2(_))
            | ( ShaderValue::Unorm8x4(_), ShaderValue::Unorm8x4(_))
            | ( ShaderValue::Snorm8x2(_), ShaderValue::Snorm8x2(_))
            | ( ShaderValue::Snorm8x4(_), ShaderValue::Snorm8x4(_))
            | ( ShaderValue::Uint16x2(_), ShaderValue::Uint16x2(_))
            | ( ShaderValue::Uint16x4(_), ShaderValue::Uint16x4(_))
            | ( ShaderValue::Sint16x2(_), ShaderValue::Sint16x2(_))
            | ( ShaderValue::Sint16x4(_), ShaderValue::Sint16x4(_))
            | (ShaderValue::Unorm16x2(_), ShaderValue::Unorm16x2(_))
            | (ShaderValue::Unorm16x4(_), ShaderValue::Unorm16x4(_))
            | (ShaderValue::Snorm16x2(_), ShaderValue::Snorm16x2(_))
            | (ShaderValue::Snorm16x4(_), ShaderValue::Snorm16x4(_))
            | (  ShaderValue::Float32(_), ShaderValue::Float32(_))
            | (ShaderValue::Float32x2(_), ShaderValue::Float32x2(_))
            | (ShaderValue::Float32x3(_), ShaderValue::Float32x3(_))
            | (ShaderValue::Float32x4(_), ShaderValue::Float32x4(_))
            | (   ShaderValue::Uint32(_), ShaderValue::Uint32(_))
            | ( ShaderValue::Uint32x2(_), ShaderValue::Uint32x2(_))
            | ( ShaderValue::Uint32x3(_), ShaderValue::Uint32x3(_))
            | ( ShaderValue::Uint32x4(_), ShaderValue::Uint32x4(_))
            | (   ShaderValue::Sint32(_), ShaderValue::Sint32(_))
            | ( ShaderValue::Sint32x2(_), ShaderValue::Sint32x2(_))
            | ( ShaderValue::Sint32x3(_), ShaderValue::Sint32x3(_))
            | ( ShaderValue::Sint32x4(_), ShaderValue::Sint32x4(_)) => true,
            _ => false,
        }
    }
}

macro_rules! impl_from {
    ($type:ty, $input:ident, $($tt:tt)*) => {
        impl From<$type> for ShaderValue {
            fn from($input: $type) -> ShaderValue {
                $($tt)*
            }
        }
    };
    ($type:ty, $outputtype:ident) => {
        impl From<$type> for ShaderValue {
            fn from(input: $type) -> ShaderValue {
                ShaderValue::$outputtype(input)
            }
        }
    }
}

impl_from! { (u8, u8), v, ShaderValue::Uint8x2([v.0, v.1]) }
impl_from! { (u8, u8, u8, u8), v, ShaderValue::Uint8x4([v.0, v.1, v.2, v.3]) }
impl_from! { (i8, i8), v, ShaderValue::Sint8x2([v.0, v.1]) }
impl_from! { (i8, i8, i8, i8), v, ShaderValue::Sint8x4([v.0, v.1, v.2, v.3]) }
impl_from! { (u16, u16), v, ShaderValue::Uint16x2([v.0, v.1]) }
impl_from! { (u16, u16, u16, u16), v, ShaderValue::Uint16x4([v.0, v.1, v.2, v.3]) }
impl_from! { (i16, i16), v, ShaderValue::Sint16x2([v.0, v.1]) }
impl_from! { (i16, i16, i16, i16), v, ShaderValue::Sint16x4([v.0, v.1, v.2, v.3]) }
impl_from! { (f32, f32), v, ShaderValue::Float32x2([v.0, v.1]) }
impl_from! { (f32, f32, f32), v, ShaderValue::Float32x3([v.0, v.1, v.2]) }
impl_from! { (f32, f32, f32, f32), v, ShaderValue::Float32x4([v.0, v.1, v.2, v.3]) }
impl_from! { (u32, u32), v, ShaderValue::Uint32x2([v.0, v.1]) }
impl_from! { (u32, u32, u32), v, ShaderValue::Uint32x3([v.0, v.1, v.2]) }
impl_from! { (u32, u32, u32, u32), v, ShaderValue::Uint32x4([v.0, v.1, v.2, v.3]) }
impl_from! { (i32, i32), v, ShaderValue::Sint32x2([v.0, v.1]) }
impl_from! { (i32, i32, i32), v, ShaderValue::Sint32x3([v.0, v.1, v.2]) }
impl_from! { (i32, i32, i32, i32), v, ShaderValue::Sint32x4([v.0, v.1, v.2, v.3]) }
impl_from! { [u8; 2], Uint8x2 }
impl_from! { [u8; 4], Uint8x4 }
impl_from! { [i8; 2], Sint8x2 }
impl_from! { [i8; 4], Sint8x4 }
impl_from! { [u16; 2], Uint16x2 }
impl_from! { [u16; 4], Uint16x4 }
impl_from! { [i16; 2], Sint16x2 }
impl_from! { [i16; 4], Sint16x4 }
impl_from! { [f32; 2], Float32x2 }
impl_from! { [f32; 3], Float32x3 }
impl_from! { [f32; 4], Float32x4 }
impl_from! { [u32; 2], Uint32x2 }
impl_from! { [u32; 3], Uint32x3 }
impl_from! { [u32; 4], Uint32x4 }
impl_from! { [i32; 2], Sint32x2 }
impl_from! { [i32; 3], Sint32x3 }
impl_from! { [i32; 4], Sint32x4 }
impl_from! { f32, Float32 }
impl_from! { u32, Uint32 }
impl_from! { i32, Sint32 }

pub enum ShaderStage {
    Vertex,
    Fragment,
    Mesh,
}

pub struct VertexPass {}

pub struct FragmentPass {}

pub struct MeshPass {}

pub struct ShaderPass {
    pub vertex_pass: Option<VertexPass>,
    pub fragment_pass: Option<FragmentPass>,
    pub mesh_pass: Option<MeshPass>,
}

pub struct ShaderParameter {
    stages: Vec<ShaderStage>,
}

pub struct ShaderGenerator {
    pub parameters: HashMap<String, ShaderParameter>,
    pub vertex_passes: Vec<VertexPass>,
    pub fragment_passes: Vec<FragmentPass>,
    pub mesh_passes: Vec<MeshPass>,
}

impl ShaderGenerator {
    pub fn new() -> Self {
        Self {
            parameters: HashMap::new(),
            vertex_passes: Vec::new(),
            fragment_passes: Vec::new(),
            mesh_passes: Vec::new(),
        }
    }

    pub fn push<'a>(&mut self, pass: impl Into<ShaderPass>) -> &mut Self {
        let pass = pass.into();

        if let Some(vertex_pass) = pass.vertex_pass {
            self.vertex_passes.push(vertex_pass);
        }

        if let Some(fragment_pass) = pass.fragment_pass {
            self.fragment_passes.push(fragment_pass);
        }

        if let Some(mesh_pass) = pass.mesh_pass {
            self.mesh_passes.push(mesh_pass);
        }

        self
    }

    pub fn generate(mut self, renderer: &mut Renderer) -> Shader {
        todo!()
    }
}

#[derive(Debug)]
pub struct Shader {
    pub module: wgpu::ShaderModule,
}

impl Shader {
    pub fn new(renderer: &mut Renderer, source: Cow<str>) -> Self {
        let module = renderer
            .wgpu_device()
            .create_shader_module(wgpu::ShaderModuleDescriptor {
                label: Some("Shader Module"),
                source: wgpu::ShaderSource::Wgsl(source),
            });

        Self { module }
    }
}
