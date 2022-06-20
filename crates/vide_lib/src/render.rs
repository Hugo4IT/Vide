use std::num::NonZeroU32;

use crate::video::VideoSettings;

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
    pub fn push_clip(mut self, clip_frame: u64, clip_time: f64, clip_progress: f64) -> Self {
        self.sequence_frame = self.clip_frame;
        self.sequence_time = self.clip_time;
        self.sequence_progress = self.clip_progress;
        self.clip_frame = clip_frame;
        self.clip_time = clip_time;
        self.clip_progress = clip_progress;

        self
    }
}

#[derive(Debug)]
pub enum RenderEvent {
    
}

#[derive(Debug)]
pub struct Renderer {
    settings: VideoSettings,

    // WGPU Special
    queue: wgpu::Queue,
    device: wgpu::Device,
    
    // VRAM -> RAM transfer
    out_texture: wgpu::Texture,
    out_texture_view: wgpu::TextureView,
    unpadded_bytes_per_row: u32,
    padded_bytes_per_row: u32,
    out_buffer: wgpu::Buffer,

    events: Vec<RenderEvent>,
}

impl Renderer {
    pub fn new(settings: VideoSettings) -> Self {
        let instance = wgpu::Instance::new(wgpu::Backends::all());
        let adapter = pollster::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions::default())).unwrap();
        let (device, queue) = pollster::block_on(
            adapter
                .request_device(
                    &wgpu::DeviceDescriptor {
                        label: Some("Device"),
                        features: wgpu::Features::empty(),
                        limits: if cfg!(target_arch = "wasm32") {
                            wgpu::Limits::downlevel_webgl2_defaults()
                        } else {
                            wgpu::Limits::default()
                        },
                    },
                None
            )).unwrap();
        
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

        Self {
            settings,
            queue,
            device,
            out_texture,
            out_texture_view,
            unpadded_bytes_per_row,
            padded_bytes_per_row,
            out_buffer,
            events: Vec::new(),
        }
    }

    #[inline]
    pub fn fps(&self) -> f64 {
        self.settings.fps
    }


    pub(crate) fn begin(&mut self) {

    }

    pub(crate) fn end(&mut self) -> Vec<u8> {
        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Main Command Encoder"),
        });
        
        let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Render Pass"),
            color_attachments: &[wgpu::RenderPassColorAttachment {
                view: &self.out_texture_view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color {
                        r: 0.3,
                        g: 0.6,
                        b: 0.9,
                        a: 1.0,
                    }),
                    store: true,
                },
            }],
            depth_stencil_attachment: None,
        });

        drop(pass);

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
                    bytes_per_row: NonZeroU32::new(self.padded_bytes_per_row),
                    rows_per_image: NonZeroU32::new(self.settings.resolution.1),
                },
            },
            wgpu::Extent3d {
                width: self.settings.resolution.0,
                height: self.settings.resolution.1,
                depth_or_array_layers: 1,
            }
        );

        self.queue.submit(core::iter::once(encoder.finish()));

        let buffer_slice = self.out_buffer.slice(..);
        let request = buffer_slice.map_async(wgpu::MapMode::Read);
        self.device.poll(wgpu::Maintain::Wait);
        let result = pollster::block_on(request);

        match result {
            Ok(()) => {
                let padded_data = buffer_slice.get_mapped_range();
                let data = padded_data
                    .chunks(self.padded_bytes_per_row as _)
                    .map(|chunk| &chunk[..self.unpadded_bytes_per_row as _])
                    .flatten()
                    .map(|x| *x)
                    .collect::<Vec<_>>();
                drop(padded_data);
                self.out_buffer.unmap();
                data
            },
            _ => panic!("Something went wrong while copying GPU buffer to RAM for encoding!"),
        }
    }
}