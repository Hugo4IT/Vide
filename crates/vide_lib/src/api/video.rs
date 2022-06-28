use core::time::Duration;

use crate::{render::{Renderer, Time}, io::Export, api::color::Color, rgb8, clip::Clip};

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

pub struct Video<'a> {
    #[cfg(feature = "preview")] event_loop: winit::event_loop::EventLoop<()>,
    #[cfg(feature = "preview")] window: winit::window::Window,
    renderer: Renderer,
    root: Clip<'a>,
    pub settings: VideoSettings,
}

impl<'a> Video<'a> {
    pub fn new(settings: VideoSettings) -> Self {
        #[cfg(feature = "preview")]
        let (event_loop, window, renderer) = {
            let event_loop = winit::event_loop::EventLoop::new();
            let window = winit::window::WindowBuilder::new()
                .with_inner_size(winit::dpi::PhysicalSize::new(settings.resolution.0, settings.resolution.1))
                .with_resizable(false)
                .build(&event_loop)
                .unwrap();
            let renderer = Renderer::new(settings, &window);

            (event_loop, window, renderer)
        };

        Self {
            #[cfg(feature = "preview")] event_loop,
            #[cfg(feature = "preview")] window,
            #[cfg(feature = "preview")] renderer,
            #[cfg(not(feature = "preview"))] renderer: Renderer::new(settings),
            root: Clip::empty(settings.duration, settings.fps),
            settings,
        }
    }

    pub fn root(&mut self) -> &mut Clip<'a> {
        &mut self.root
    }

    #[allow(unused_variables)]
    pub fn render(mut self, exporter: impl Export) where Self: 'static {
        self.renderer.register_effects(self.root.get_registration_packets());

        #[cfg(feature = "preview")] self.preview();
        #[cfg(not(feature = "preview"))] self.export(exporter);
    }

    #[cfg(feature = "preview")]
    fn preview(self) where Self: 'static {
        let Self {
            settings,
            window,
            event_loop,
            mut renderer,
            mut root,
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
                render_frame(frame, &mut renderer, &mut root);
                frame = (frame + 1) % (settings.duration.as_secs_f64() * settings.fps) as u64;
            },
            winit::event::Event::MainEventsCleared => {
                window.request_redraw();
            },
            _ => (),
        });
    }

    #[cfg(not(feature = "preview"))]
    fn export(&mut self, mut exporter: impl Export) {
        use crate::clip::IntoFrame;

        info!("Starting render...");
        let start_time = std::time::Instant::now();

        exporter.begin(self.settings);

        for frame in 0..self.settings.duration.into_frame(self.settings.fps) {
            info!("Encoding frame...");
            exporter.push_frame(true, &render_frame(frame, &mut self.renderer, &mut self.root).unwrap()[..]);
        }

        info!("Finalizing encoding...");

        exporter.end();

        info!("Done! Rendering took {:0.05}s", (std::time::Instant::now() - start_time).as_secs_f32());
    }
}

fn render_frame(frame: u64, renderer: &mut Renderer, clip: &mut Clip<'_>) -> Option<Vec<u8>> {
    let time = frame as f64 / renderer.fps();
    let progress = time / renderer.duration().as_secs_f64();

    info!("Rendering frame {}...", frame);

    renderer.render(clip.render(
        Time {
            video_frame: frame,
            sequence_frame: frame,
            clip_frame: frame,
            video_time: time,
            sequence_time: time,
            clip_time: time,
            video_progress: progress,
            sequence_progress: progress,
            clip_progress: progress,
        },
        renderer.last_frame(),
        renderer.screen_matrix,
    ))
}