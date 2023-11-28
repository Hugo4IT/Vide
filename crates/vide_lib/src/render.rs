use std::{
    any::Any,
    sync::{Mutex, MutexGuard, OnceLock},
    time::Duration,
};

use log::info;
use wgpu::util::DeviceExt;

use crate::{api::video::VideoSettings, clip::IntoFrame, effect::EffectRegistrationPacket};

pub(crate) type PushFunction = fn(&mut Box<dyn Any>, &Box<dyn Any>, u64);
pub(crate) type RenderFunction =
    for<'a> fn(&'a mut Box<dyn Any>, MutexGuard<wgpu::RenderPass<'a>>, &wgpu::Device, &wgpu::Queue);

/// Timing information needed for rendering
#[derive(Default, Debug, Clone, Copy)]
pub struct Time {
    /// Current frame
    pub video_frame: u64,
    /// Current frame relative to the first frame of the parent sequence (video_frame - sequence_start_frame)
    pub sequence_frame: u64,
    /// Current frame relative to the first frame of this clip (video_frame - clip_start_frame)
    pub clip_frame: u64,

    /// Current time (in seconds) relative the start of the parent video
    pub video_time: f64,
    /// Current time (in seconds) relative the start of the parent sequence
    pub sequence_time: f64,
    /// Current time (in seconds) relative the start of this clip
    pub clip_time: f64,

    /// Current video progress ranging from `0.0` to `1.0`. This can be used as `time` input for interpolation functions
    pub video_progress: f64,
    /// Current sequence progress ranging from `0.0` to `1.0`. This can be used as `time` input for interpolation functions
    pub sequence_progress: f64,
    /// Current clip progress ranging from `0.0` to `1.0`. This can be used as `time` input for interpolation functions
    pub clip_progress: f64,
}

impl Time {
    pub fn derive_clip(mut self, clip_frame: u64, clip_time: f64, clip_progress: f64) -> Self {
        self.sequence_frame = self.clip_frame;
        self.sequence_time = self.clip_time;
        self.sequence_progress = self.clip_progress;
        self.clip_frame = clip_frame;
        self.clip_time = clip_time;
        self.clip_progress = clip_progress;

        self
    }
}

pub enum RenderEvent<'a> {
    WriteBuffer {
        buffer: &'a wgpu::Buffer,
        offset: wgpu::BufferAddress,
        data: &'a [u8],
    },
    SetTransform(cgmath::Matrix4<f32>),
    Effect {
        // Render effect
        id: usize,
        params: &'a Box<dyn Any>,
        frame: u64,
    },
}

pub trait Render {
    fn render(&self, renderer: &mut Renderer);
}

pub struct Renderer {
    pub settings: VideoSettings,
    pub screen_matrix: cgmath::Matrix4<f32>,

    // WGPU Special
    queue: wgpu::Queue,
    device: wgpu::Device,
    config: wgpu::SurfaceConfiguration,

    // VRAM -> RAM transfer for exporting to file
    #[cfg(not(feature = "preview"))]
    out_texture: wgpu::Texture,
    #[cfg(not(feature = "preview"))]
    out_texture_view: wgpu::TextureView,
    #[cfg(not(feature = "preview"))]
    unpadded_bytes_per_row: u32,
    #[cfg(not(feature = "preview"))]
    padded_bytes_per_row: u32,
    #[cfg(not(feature = "preview"))]
    out_buffer: wgpu::Buffer,

    // Window surface for preview
    #[cfg(feature = "preview")]
    surface: wgpu::Surface,

    /// Holds function pointers to all `push()` functions of registered effects
    effect_push_functions: Vec<Option<PushFunction>>,
    /// Holds function pointers to all `render()` functions of registered effects
    effect_render_functions: Vec<Option<RenderFunction>>,
    /// Holds `self` for the `render()` functions described in [`Renderer::effect_functions`]
    effects: Vec<Option<Box<dyn Any>>>,

    transform_buffer: wgpu::Buffer,
    transform_bind_group_layout: wgpu::BindGroupLayout,
    transform_bind_group: wgpu::BindGroup,

    depth_texture_view: wgpu::TextureView,
}

impl Renderer {
    pub fn new(
        settings: VideoSettings,
        #[cfg(feature = "preview")] window: &winit::window::Window,
    ) -> Self {
        let instance = wgpu::Instance::new(wgpu::Backends::all());

        #[cfg(feature = "preview")]
        let surface = unsafe { instance.create_surface(window) };

        let adapter = pollster::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::HighPerformance,
            force_fallback_adapter: false,
            #[cfg(feature = "preview")]
            compatible_surface: Some(&surface),
            #[cfg(not(feature = "preview"))]
            compatible_surface: None,
        }))
        .unwrap();

        let (device, queue) = pollster::block_on(adapter.request_device(
            &wgpu::DeviceDescriptor {
                label: Some("Device"),
                features: wgpu::Features::empty(),
                limits: if cfg!(target_arch = "wasm32") {
                    wgpu::Limits::downlevel_webgl2_defaults()
                } else {
                    wgpu::Limits::default()
                },
            },
            None,
        ))
        .unwrap();

        #[cfg(not(feature = "preview"))]
        let (
            out_texture,
            out_texture_view,
            unpadded_bytes_per_row,
            padded_bytes_per_row,
            out_buffer,
        ) = {
            let out_texture = device.create_texture(&wgpu::TextureDescriptor {
                label: Some("Output Texture"),
                size: wgpu::Extent3d {
                    width: settings.resolution.0,
                    height: settings.resolution.1,
                    depth_or_array_layers: 1,
                },
                mip_level_count: 1,
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                format: wgpu::TextureFormat::Rgba8UnormSrgb,
                usage: wgpu::TextureUsages::COPY_SRC | wgpu::TextureUsages::RENDER_ATTACHMENT,
            });
            let out_texture_view = out_texture.create_view(&wgpu::TextureViewDescriptor::default());

            let pixel_size = core::mem::size_of::<[u8; 4]>() as u32;
            let align = wgpu::COPY_BYTES_PER_ROW_ALIGNMENT;
            let unpadded_bytes_per_row = pixel_size * settings.resolution.0;
            let padding = (align - unpadded_bytes_per_row % align) % align;
            let padded_bytes_per_row = unpadded_bytes_per_row + padding;

            let buffer_size = (padded_bytes_per_row * settings.resolution.1) as wgpu::BufferAddress;
            let out_buffer = device.create_buffer(&wgpu::BufferDescriptor {
                label: Some("Output Buffer"),
                size: buffer_size,
                usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
                mapped_at_creation: false,
            });

            (
                out_texture,
                out_texture_view,
                unpadded_bytes_per_row,
                padded_bytes_per_row,
                out_buffer,
            )
        };

        let config = {
            let config = wgpu::SurfaceConfiguration {
                usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
                #[cfg(feature = "preview")]
                format: surface.get_supported_formats(&adapter)[0],
                #[cfg(not(feature = "preview"))]
                format: wgpu::TextureFormat::Rgba8UnormSrgb,
                width: settings.resolution.0,
                height: settings.resolution.1,
                #[cfg(feature = "preview")]
                present_mode: wgpu::PresentMode::Fifo,
                #[cfg(not(feature = "preview"))]
                present_mode: wgpu::PresentMode::Immediate,
            };

            #[cfg(feature = "preview")]
            surface.configure(&device, &config);

            config
        };

        #[rustfmt::skip]
        let screen_matrix = cgmath::Matrix4::new(
            2.0 / settings.resolution.0 as f32, 0.0,                                0.0, 0.0,
            0.0,                                2.0 / settings.resolution.1 as f32, 0.0, 0.0,
            0.0,                                0.0,                                1.0, 0.0,
            0.0,                                0.0,                                0.0, 1.0,
        );

        let transform_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Transform Buffer"),
            contents: bytemuck::cast_slice(&[Into::<[[f32; 4]; 4]>::into(screen_matrix)]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let transform_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("Transform Bind Group Layout"),
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }],
            });

        let transform_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Transform Bind Group"),
            layout: &transform_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: transform_buffer.as_entire_binding(),
            }],
        });

        let depth_texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("Depth Texture"),
            size: wgpu::Extent3d {
                width: settings.resolution.0,
                height: settings.resolution.1,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Depth32Float,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING,
        });

        let depth_texture_view = depth_texture.create_view(&wgpu::TextureViewDescriptor::default());

        Self {
            settings,
            screen_matrix,

            queue,
            device,
            config,

            #[cfg(not(feature = "preview"))]
            out_texture,
            #[cfg(not(feature = "preview"))]
            out_texture_view,
            #[cfg(not(feature = "preview"))]
            unpadded_bytes_per_row,
            #[cfg(not(feature = "preview"))]
            padded_bytes_per_row,
            #[cfg(not(feature = "preview"))]
            out_buffer,

            #[cfg(feature = "preview")]
            surface,

            effect_push_functions: Vec::new(),
            effect_render_functions: Vec::new(),
            effects: Vec::new(),

            transform_buffer,
            transform_bind_group_layout,
            transform_bind_group,

            depth_texture_view,
        }
    }

    #[inline]
    pub fn fps(&self) -> f64 {
        self.settings.fps
    }

    #[inline]
    pub fn duration(&self) -> Duration {
        self.settings.duration
    }

    #[inline]
    pub fn last_frame(&self) -> u64 {
        self.settings.duration.into_frame(self.settings.fps)
    }

    #[inline]
    pub fn wgpu_device(&self) -> &wgpu::Device {
        &self.device
    }

    #[inline]
    pub fn wgpu_config(&self) -> wgpu::SurfaceConfiguration {
        self.config.clone()
    }

    #[inline]
    pub fn wgpu_transform_bind_group_layout(&self) -> &wgpu::BindGroupLayout {
        &self.transform_bind_group_layout
    }

    pub(crate) fn register_effects(&mut self, packets: Vec<EffectRegistrationPacket>) {
        info!(
            "Renderer received {} effect registration packets",
            packets.len()
        );

        for packet in packets {
            info!("Registering effect on id {}", packet.id);

            if self.effect_render_functions.len() < packet.id + 1 {
                self.effect_push_functions
                    .extend((self.effect_push_functions.len()..=packet.id).map(|_| None));
                self.effect_render_functions
                    .extend((self.effect_render_functions.len()..=packet.id).map(|_| None));
                self.effects
                    .extend((self.effects.len()..=packet.id).map(|_| None));
            }

            if self.effect_render_functions[packet.id].is_none() {
                self.effect_push_functions[packet.id] = Some(packet.push_function);
                self.effect_render_functions[packet.id] = Some(packet.render_function);
                self.effects[packet.id] = Some((packet.init_function)(self));
            }
        }
    }

    pub(crate) fn render(&mut self, events: Vec<RenderEvent>) -> Option<Vec<u8>> {
        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Main Command Encoder"),
            });

        #[cfg(feature = "preview")]
        let (output, surface_view) = {
            let output = self.surface.get_current_texture().unwrap();
            let view = output
                .texture
                .create_view(&wgpu::TextureViewDescriptor::default());
            (output, view)
        };

        {
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    #[cfg(not(feature = "preview"))]
                    view: &self.out_texture_view,
                    #[cfg(feature = "preview")]
                    view: &surface_view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: self.settings.background_color.r,
                            g: self.settings.background_color.g,
                            b: self.settings.background_color.b,
                            a: 1.0,
                        }),
                        store: true,
                    },
                })],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: &self.depth_texture_view,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(1.0),
                        store: true,
                    }),
                    stencil_ops: None,
                }),
            });

            pass.set_bind_group(0, &self.transform_bind_group, &[]);

            let pass_ref = Mutex::new(pass);
            for event in events {
                match event {
                    RenderEvent::WriteBuffer {
                        buffer,
                        offset,
                        data,
                    } => {
                        self.queue.write_buffer(buffer, offset, data);
                    }
                    RenderEvent::SetTransform(transform) => {
                        self.queue.write_buffer(
                            &self.transform_buffer,
                            0,
                            bytemuck::cast_slice(&[Into::<[[f32; 4]; 4]>::into(transform)]),
                        );
                    }
                    RenderEvent::Effect { id, params, frame } => {
                        self.effect_push_functions[id].unwrap()(
                            self.effects.get_mut(id).unwrap().as_mut().unwrap(),
                            params,
                            frame,
                        );
                    }
                }
            }

            let render_functions = self.effect_render_functions.clone();
            for (id, effect) in self.effects.iter_mut().enumerate() {
                if let Some(effect) = effect {
                    (render_functions[id].unwrap())(
                        effect,
                        pass_ref.lock().unwrap(),
                        &self.device,
                        &self.queue,
                    );
                }
            }
        }

        #[cfg(not(feature = "preview"))]
        encoder.copy_texture_to_buffer(
            wgpu::ImageCopyTexture {
                aspect: wgpu::TextureAspect::All,
                texture: &self.out_texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
            },
            wgpu::ImageCopyBuffer {
                buffer: &self.out_buffer,
                layout: wgpu::ImageDataLayout {
                    offset: 0,
                    bytes_per_row: std::num::NonZeroU32::new(self.padded_bytes_per_row),
                    rows_per_image: std::num::NonZeroU32::new(self.settings.resolution.1),
                },
            },
            wgpu::Extent3d {
                width: self.settings.resolution.0,
                height: self.settings.resolution.1,
                depth_or_array_layers: 1,
            },
        );

        self.queue.submit(core::iter::once(encoder.finish()));
        #[cfg(feature = "preview")]
        output.present();
        #[cfg(feature = "preview")]
        return None;

        #[cfg(not(feature = "preview"))]
        {
            info!("Copying buffers...");

            let buffer_slice = self.out_buffer.slice(..);
            let (tx, rx) = futures_intrusive::channel::shared::oneshot_channel();
            buffer_slice.map_async(wgpu::MapMode::Read, move |result| {
                tx.send(result).unwrap();
            });
            self.device.poll(wgpu::Maintain::Wait);
            let result = pollster::block_on(rx.receive()).unwrap();

            match result {
                Ok(()) => {
                    let padded_data = buffer_slice.get_mapped_range();
                    let data = padded_data
                        .chunks(self.padded_bytes_per_row as _)
                        .flat_map(|chunk| &chunk[..self.unpadded_bytes_per_row as _])
                        .copied()
                        .collect::<Vec<_>>();
                    drop(padded_data);
                    self.out_buffer.unmap();
                    Some(data)
                }
                _ => panic!("Something went wrong while copying GPU buffer to RAM for encoding!"),
            }
        }
    }
}
