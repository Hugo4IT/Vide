use core::time::Duration;
use std::time::Instant;

use crate::{render::{Renderer, Time}, sequence::{Sequence, Clip}, io::{Export, self}};

use log::info;

#[derive(Debug, Clone, Copy)]
pub struct VideoSettings {
    pub fps: f64,
    pub resolution: (u32, u32),
    pub duration: Duration,
}

impl Default for VideoSettings {
    fn default() -> Self {
        Self {
            fps: 60.0,
            resolution: (1920, 1080),
            duration: Duration::from_secs(30),
        }
    }
}

pub struct Video {
    renderer: Renderer,
    root_sequence: Sequence,
    video_settings: VideoSettings,
}

impl Video {
    pub fn new(video_settings: VideoSettings) -> Self {
        Self {
            renderer: Renderer::new(video_settings),
            root_sequence: Sequence::new(60.0, Duration::from_secs(5)).with_name("root"),
            video_settings,
        }
    }

    pub fn root<'a>(&'a mut self) -> &'a mut Sequence {
        &mut self.root_sequence
    }

    pub fn render(&mut self, mut exporter: impl Export) {
        info!("Starting render...");
        let start_time = Instant::now();

        exporter.begin(self.video_settings);

        let root_info = self.root_sequence.info();
        for i in 0..((root_info.duration.as_secs_f64() * self.renderer.fps()) as u64) {
            let frame = i;
            let time = i as f64 / self.renderer.fps();
            let progress = time / root_info.duration.as_secs_f64();

            info!("Rendering frame {}...", frame);

            self.renderer.begin();
            self.root_sequence.render(Time {
                video_frame: frame,
                sequence_frame: frame,
                clip_frame: frame,
                video_time: time,
                sequence_time: time,
                clip_time: time,
                video_progress: progress,
                sequence_progress: progress,
                clip_progress: progress,
            }, &mut self.renderer);

            info!("Copying buffers...");
            
            let frame_data = self.renderer.end();

            info!("Encoding frame...");

            exporter.push_frame(true, &frame_data[..]);
        }

        info!("Finalizing encoding...");

        exporter.end();

        info!("Done! Rendering took {:0.05}s", (Instant::now() - start_time).as_secs_f32());
    }
}