use core::time::Duration;
use std::time::Instant;

use crate::{render::{Renderer, Time}, sequence::{Sequence, Clip}, io::Export, api::color::Color, rgb8};

use log::info;

#[derive(Debug, Clone, Copy)]
pub struct VideoSettings {
    pub fps: f64,
    pub resolution: (u32, u32),
    pub duration: Duration,
    pub background_color: Color,
}

impl Default for VideoSettings {
    fn default() -> Self {
        Self {
            fps: 60.0,
            resolution: (1920, 1080),
            duration: Duration::from_secs(30),
            background_color: rgb8!(0x17, 0x17, 0x17),
        }
    }
}

pub struct Video {
    #[cfg(feature = "preview")] event_loop: winit::event_loop::EventLoop<()>,
    #[cfg(feature = "preview")] window: winit::window::Window,
    renderer: Renderer,
    root_sequence: Sequence,
    settings: VideoSettings,
}

impl Video {
    pub fn new(video_settings: VideoSettings) -> Self {
        #[cfg(feature = "preview")]
        let (event_loop, window, renderer) = {
            let event_loop = winit::event_loop::EventLoop::new();
            let window = winit::window::WindowBuilder::new()
                .with_inner_size(winit::dpi::PhysicalSize::new(video_settings.resolution.0, video_settings.resolution.1))
                .with_resizable(false)
                .build(&event_loop)
                .unwrap();
            let renderer = Renderer::new(video_settings, &window);

            (event_loop, window, renderer)
        };

        Self {
            #[cfg(feature = "preview")] event_loop,
            #[cfg(feature = "preview")] window,
            #[cfg(feature = "preview")] renderer,
            #[cfg(not(feature = "preview"))] renderer: Renderer::new(video_settings),
            root_sequence: Sequence::new(60.0, Duration::from_secs(5)).with_name("root"),
            settings: video_settings,
        }
    }

    pub fn root(&mut self) -> &mut Sequence {
        &mut self.root_sequence
    }

    pub fn render(mut self, exporter: impl Export) {
        #[cfg(feature = "preview")] self.preview();
        #[cfg(not(feature = "preview"))] self.export(exporter);
    }

    #[cfg(feature = "preview")]
    fn preview(mut self) {
        let Self {
            settings,
            window,
            event_loop,
            mut renderer,
            mut root_sequence,
            ..
        } = self;

        let mut frame = 0u64;
        event_loop.run(move |event, _, control_flow| match event {
            winit::event::Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == window.id() => match event {
                winit::event::WindowEvent::CloseRequested => *control_flow = winit::event_loop::ControlFlow::Exit,
                _ => (),
            },
            winit::event::Event::RedrawRequested(window_id) if window_id == window.id() => {
                render_frame(frame, &mut renderer, &mut root_sequence);
                frame = (frame + 1) % (self.settings.duration.as_secs_f64() * settings.fps) as u64;
            },
            winit::event::Event::MainEventsCleared => {
                window.request_redraw();
            },
            _ => (),
        });
    }

    #[cfg(not(feature = "preview"))]
    fn export(&mut self, mut exporter: impl Export) {
        info!("Starting render...");
        let start_time = Instant::now();

        exporter.begin(self.settings);

        let root_info = self.root_sequence.info();
        for frame in 0..((root_info.duration.as_secs_f64() * self.renderer.fps()) as u64) {
            info!("Encoding frame...");
            exporter.push_frame(true, &render_frame(frame, &mut self.renderer, &mut self.root_sequence).unwrap()[..]);
        }

        info!("Finalizing encoding...");

        exporter.end();

        info!("Done! Rendering took {:0.05}s", (Instant::now() - start_time).as_secs_f32());
    }
}

fn render_frame(frame: u64, renderer: &mut Renderer, root_sequence: &mut Sequence) -> Option<Vec<u8>> {
    let time = frame as f64 / renderer.fps();
    let progress = time / root_sequence.info().duration.as_secs_f64();

    info!("Rendering frame {}...", frame);

    renderer.begin();
    root_sequence.render(Time {
        video_frame: frame,
        sequence_frame: frame,
        clip_frame: frame,
        video_time: time,
        sequence_time: time,
        clip_time: time,
        video_progress: progress,
        sequence_progress: progress,
        clip_progress: progress,
    }, renderer);

    #[cfg(not(feature = "preview"))] info!("Copying buffers...");
    
    renderer.end()
}