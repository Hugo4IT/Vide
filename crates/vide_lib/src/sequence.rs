use core::time::Duration;

use crate::render::{Renderer, Time};

use log::info;

pub trait IntoFrame {
    fn into_frame(self, fps: f64) -> u64;
}

impl IntoFrame for i32 {
    fn into_frame(self, fps: f64) -> u64 {
        (self as u64).into_frame(fps)
    }
}

impl IntoFrame for u64 {
    fn into_frame(self, _fps: f64) -> u64 {
        self
    }
}

impl IntoFrame for Duration {
    fn into_frame(self, fps: f64) -> u64 {
        self.as_secs_f64().into_frame(fps)
    }
}

impl IntoFrame for f64 {
    fn into_frame(self, fps: f64) -> u64 {
        (self * fps) as u64
    }
}

#[derive(Debug, Clone)]
pub struct ClipInfo {
    pub name: Option<String>,
    pub duration: Duration,
}

pub trait Clip {
    fn info(&self) -> ClipInfo;
    fn render(&self, time: Time, renderer: &mut Renderer);
}

struct ClipHolder {
    pub clip: Box<dyn Clip>,
    pub start_frame: u64,
}

pub struct Sequence {
    name: Option<String>,
    duration: Duration,
    clips: Vec<ClipHolder>,
    fps: f64,
}

impl Sequence {
    pub fn new(fps: f64, duration: Duration) -> Self {
        Self {
            name: None,
            duration,
            clips: Vec::new(),
            fps,
        }
    }

    #[inline]
    pub fn with_name<S: ToString>(mut self, name: S) -> Self {
        self.name = Some(name.to_string());
        self
    }

    pub fn push_clip<F: IntoFrame, C: 'static + Clip>(&mut self, start_time: F, clip: C) {
        self.clips.push(ClipHolder {
            start_frame: start_time.into_frame(self.fps),
            clip: Box::new(clip),
        })
    }
}

impl Clip for Sequence {
    #[inline]
    fn info(&self) -> ClipInfo {
        ClipInfo {
            name: self.name.clone(),
            duration: self.duration,
        }
    }

    fn render(&self, time: Time, renderer: &mut Renderer) {
        info!("Rendering subsequence {} (frame {})", self.name.as_ref().unwrap_or(&String::from("<anonymous>")), time.clip_frame);
        for clip_holder in self.clips.iter() {
            let info = clip_holder.clip.info();
            if clip_holder.start_frame <= time.clip_frame && clip_holder.start_frame + (info.duration.as_secs_f64() * renderer.fps()) as u64 > time.clip_frame {
                let clip_frame = time.clip_frame - clip_holder.start_frame;
                let clip_time = time.clip_time - clip_holder.start_frame as f64 / renderer.fps();
                clip_holder.clip.render(time.push_clip(clip_frame, clip_time, clip_time / info.duration.as_secs_f64()), renderer);
            }
        }
    }
}