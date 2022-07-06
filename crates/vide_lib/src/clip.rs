use core::time::Duration;
use std::{ops::{Range, RangeBounds, Bound}, marker::PhantomData};

use crate::{render::{Time, RenderEvent}, effect::{EffectData, RegisteredEffectData, EffectRegistrationPacket}, api::transform::{Transform, OPENGL_TO_WGPU_MATRIX}};

pub trait IntoFrame {
    fn into_frame(self, fps: f64) -> u64;
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

pub struct Clip<'a> {
    children: Vec<Clip<'a>>,
    transform: Transform,
    effects: Vec<EffectData>,
    /// Effect emit an EffectRegistrationPacket when their backend hasn't been
    /// initialized yet.
    effect_registration_packets: Option<Vec<EffectRegistrationPacket>>,
    /// When `None`, the clip will play from frame 0
    start: Option<u64>,
    /// When `None`, the clip will play until the end of its parent sequence
    end: Option<u64>,
    fps: f64,
    /// Prevent unused lifetime error
    _phantom: PhantomData<&'a ()>,
}

impl<'a> Clip<'a> {
    pub(crate) fn empty(duration: Duration, fps: f64) -> Self {
        Self {
            children: Vec::new(),
            transform: Transform::default(),
            effects: Vec::new(),
            effect_registration_packets: Some(Vec::new()),
            start: Some(0),
            end: Some(duration.into_frame(fps)),
            fps,
            _phantom: PhantomData::default(),
        }
    }

    fn start(&self) -> u64 {
        self.start.unwrap_or(0)
    }

    fn end(&self, parent_end: u64) -> u64 {
        self.end.unwrap_or(parent_end)
    }

    fn in_time_frame(&self, frame: u64) -> bool {
        self.start.map(|s|s<=frame).unwrap_or(true)
        && self.end.map(|e|e>frame).unwrap_or(true)
    }

    fn progress(&self, frame: u64, parent_end: u64) -> f64 {
        let start = self.start();
        let end = self.end(parent_end);

        (frame as f64 - start as f64) / (end - start) as f64
    }

    pub fn translate(&mut self, transform: impl cgmath::Transform3) -> &mut Clip<'a> {
        
        self
    }

    pub fn new_clip(&mut self, time_range: Range<impl IntoFrame + Copy>) -> &mut Clip<'a> {
        self.children.push(Clip::<'a> {
            children: Vec::new(),
            transform: Transform::default(),
            effects: Vec::new(),
            effect_registration_packets: Some(Vec::new()),
            start: match time_range.start_bound() {
                Bound::Included(n) => Some(n.into_frame(self.fps)),
                Bound::Excluded(n) => Some(n.into_frame(self.fps) + 1),
                Bound::Unbounded => None,
            },
            end: match time_range.end_bound() {
                Bound::Included(n) => Some(n.into_frame(self.fps)),
                Bound::Excluded(n) => Some(n.into_frame(self.fps) - 1),
                Bound::Unbounded => None,
            },
            fps: self.fps,
            _phantom: PhantomData::default(),
        });

        self.children.last_mut().unwrap()
    }

    pub fn effect<E: 'static + RegisteredEffectData>(&mut self, effect: E) -> &mut Clip<'a> {
        if unsafe { !E::is_registered() } {
            self.effect_registration_packets.as_mut().unwrap().push(EffectRegistrationPacket {
                id: unsafe { E::get_id() },
                push_function: E::_push,
                render_function: E::_render,
                init_function: E::_new,
            });
        }

        self.effects.push(EffectData {
            id: unsafe { E::get_id() },
            params: Box::new(effect),
        });

        self
    }

    pub(crate) fn get_registration_packets(&mut self) -> Vec<EffectRegistrationPacket> {
        let mut packets = self.effect_registration_packets.take().unwrap();
        packets.extend(self.children.iter_mut().flat_map(|child| child.get_registration_packets()));
        packets
    }

    pub(crate) fn render(&mut self, time: Time, clip_end: u64, parent_matrix: cgmath::Matrix4<f32>) -> Vec<RenderEvent> {
        let matrix = self.transform.matrix(parent_matrix);
        let mut events = Vec::new();

        for clip in self.children.iter_mut() {
            if clip.in_time_frame(time.clip_frame) {
                let clip_frame = time.clip_frame - clip.start();
                let clip_time = time.clip_time - clip.start() as f64 / self.fps;
                let clip_progress = clip.progress(clip_frame, clip_end);
                events.extend(clip.render(time.derive_clip(clip_frame, clip_time, clip_progress), clip.end(clip_end), matrix));
            }
        }

        events.push(RenderEvent::SetTransform(matrix * OPENGL_TO_WGPU_MATRIX));
        events.extend(self.effects.iter().map(|effect| RenderEvent::Effect {
            id: effect.id,
            params: &effect.params,
            frame: time.clip_frame,
        }));

        events
    }
}