use crate::clip::IntoFrame;

use self::ease::{EasingFunction, LINEAR};

#[macro_export]
macro_rules! lerp {
    ($start:expr, $end:expr, $progress:expr) => {
        ((($end) - ($start)) * ($progress) + ($start))
    };
}

#[macro_export]
macro_rules! cubic_bezier {
    ($x1:expr, $y1:expr, $x2:expr, $y2:expr) => {
        |t| t /* TODO: figure this out */ /* lerp!(lerp!(0.0, $x1, t), lerp!($x2, 1.0, t), t), lerp!(lerp!(0.0, $y1, t), lerp!($y2, 1.0, t), t) */
    };
}

#[macro_export]
macro_rules! unanimated {
    ($value:expr) => {
        $crate::api::animation::AnimatedPropertyBuilder::new(60.0)
            .keyframe(
                $crate::api::animation::KeyframeTiming::Abs(0),
                $crate::api::animation::ease::LINEAR,
                $value,
            )
            .build()
    };
}

macro_rules! impl_interpolate {
    ($typ:ty) => {
        impl Interpolate for $typ {
            fn interpolate(a: Self, b: Self, t: f64) -> Self {
                ((b - a) as f64 * t) as $typ + a
            }
        }
    };
}

pub mod ease {
    pub type EasingFunction = fn(f64) -> f64;

    /// `f(t)=t`
    pub const LINEAR: EasingFunction = |t| t;
    /// `f(t)=t^2`
    pub const IN_QUADRATIC: EasingFunction = |t| t * t;
    /// `f(t)=t^3`
    pub const IN_CUBIC: EasingFunction = |t| t * t * t;
    /// `f(t)=t^4`
    pub const IN_QUARTIC: EasingFunction = |t| t * t * t * t;
    /// `f(t)=t^5`
    pub const IN_QUINTIC: EasingFunction = |t| t * t * t * t * t;
    /// `f(t)=t^10`
    pub const IN_EXPONENTIAL: EasingFunction = |t| t * t * t * t * t * t;
    /// `f(t)=t^2`
    pub const OUT_QUADRATIC: EasingFunction = |t| 1.0 - (1.0 - t).powi(2);
    /// `f(t)=t^3`
    pub const OUT_CUBIC: EasingFunction = |t| 1.0 - (1.0 - t).powi(3);
    /// `f(t)=t^4`
    pub const OUT_QUARTIC: EasingFunction = |t| 1.0 - (1.0 - t).powi(4);
    /// `f(t)=t^5`
    pub const OUT_QUINTIC: EasingFunction = |t| 1.0 - (1.0 - t).powi(5);
    /// `f(t)=t^10`
    pub const OUT_EXPONENTIAL: EasingFunction = |t| 1.0 - (1.0 - t).powi(10);
    /// Overshoots, catapult-ish motion
    pub const IN_BACK: EasingFunction = cubic_bezier!(0.69, -0.53, 0.06, 0.99);
    /// Overshoots at end
    pub const OUT_BACK: EasingFunction = cubic_bezier!(0.42, 1.5, 0.35, 1.0);
    /// Overshoots at both sides of animation
    pub const IN_OUT_BACK: EasingFunction = cubic_bezier!(0.84, -0.43, 0.11, 1.29);

    pub const IN_OUT_QUINTIC: EasingFunction = |t| {
        if t < 0.5 {
            16.0 * t * t * t * t * t
        } else {
            1.0 - (-2.0 * t + 2.0).powi(5) / 2.0
        }
    };
}

pub trait Interpolate {
    fn interpolate(a: Self, b: Self, t: f64) -> Self;
}

impl_interpolate!(u8);
impl_interpolate!(u16);
impl_interpolate!(u32);
impl_interpolate!(u64);
impl_interpolate!(u128);
impl_interpolate!(i8);
impl_interpolate!(i16);
impl_interpolate!(i32);
impl_interpolate!(i64);
impl_interpolate!(i128);
impl_interpolate!(f32);
impl_interpolate!(f64);

impl<A, B> Interpolate for (A, B)
where
    A: Interpolate,
    B: Interpolate,
{
    fn interpolate(a: Self, b: Self, t: f64) -> Self {
        (A::interpolate(a.0, b.0, t), B::interpolate(a.1, b.1, t))
    }
}

impl<A, B, C> Interpolate for (A, B, C)
where
    A: Interpolate,
    B: Interpolate,
    C: Interpolate,
{
    fn interpolate(a: Self, b: Self, t: f64) -> Self {
        (
            A::interpolate(a.0, b.0, t),
            B::interpolate(a.1, b.1, t),
            C::interpolate(a.2, b.2, t),
        )
    }
}

impl<A, B, C, D> Interpolate for (A, B, C, D)
where
    A: Interpolate,
    B: Interpolate,
    C: Interpolate,
    D: Interpolate,
{
    fn interpolate(a: Self, b: Self, t: f64) -> Self {
        (
            A::interpolate(a.0, b.0, t),
            B::interpolate(a.1, b.1, t),
            C::interpolate(a.2, b.2, t),
            D::interpolate(a.3, b.3, t),
        )
    }
}

#[derive(Clone)]
pub struct Keyframe<T: Interpolate> {
    pub easing: EasingFunction,
    pub state: T,
    pub frame: u64,
}

impl<T: Interpolate + Clone + std::fmt::Debug> Keyframe<T> {
    pub fn evaluate(&self, previous: Keyframe<T>, frame: u64) -> T {
        // t: 0.0..=1.0
        let t = (frame - previous.frame) as f64 / (self.frame - previous.frame) as f64;
        T::interpolate(previous.state, self.state.clone(), (self.easing)(t))
    }
}

pub struct AnimatedProperty<T: Interpolate + Clone> {
    initial: T,
    keyframes: Vec<Keyframe<T>>,
}

impl<T: Interpolate + Clone + std::fmt::Debug> AnimatedProperty<T> {
    pub fn new(initial: T, keyframes: Vec<Keyframe<T>>) -> Self {
        Self { initial, keyframes }
    }

    pub fn push_keyframe(&mut self, keyframe: Keyframe<T>) {
        self.keyframes.push(keyframe)
    }

    pub fn evaluate(&self, frame: u64) -> T {
        // Fallback when no keyframes
        if self.keyframes.is_empty() {
            return self.initial.clone();
        } else {
            // When on first keyframe, interpolate with self.initial
            let keyframe = self.keyframes.first().unwrap();
            if keyframe.frame >= frame {
                return keyframe.evaluate(
                    Keyframe {
                        easing: LINEAR,
                        state: self.initial.clone(),
                        frame: 0,
                    },
                    frame,
                );
            }
        }

        // Interpolate between keyframes
        for keyframes in self.keyframes.windows(2) {
            let previous = keyframes[0].clone();
            let keyframe = keyframes[1].clone();
            if keyframe.frame >= frame {
                return keyframe.evaluate(previous, frame);
            }
        }

        // When all keyframes have passed
        if let Some(keyframe) = self.keyframes.last() {
            return keyframe.state.clone();
        }

        self.initial.clone() // Fallback
    }
}

impl<T> Default for AnimatedProperty<T>
where
    T: Default + Interpolate + Clone,
{
    fn default() -> Self {
        Self {
            initial: T::default(),
            keyframes: Vec::new(),
        }
    }
}

pub enum KeyframeTiming<T: IntoFrame> {
    Abs(T),
    Rel(T),
}

pub struct AnimatedPropertyBuilder<T: Interpolate + Clone> {
    initial: Option<T>,
    keyframes: Vec<Keyframe<T>>,
    fps: f64,
}

impl<T: Interpolate + Clone + std::fmt::Debug> AnimatedPropertyBuilder<T> {
    pub fn new(fps: f64) -> Self {
        Self {
            initial: None,
            keyframes: Vec::new(),
            fps,
        }
    }

    pub fn keyframe(
        mut self,
        at: KeyframeTiming<impl IntoFrame>,
        easing: EasingFunction,
        state: impl Into<T>,
    ) -> Self {
        let frame = match at {
            KeyframeTiming::Abs(at) => at.into_frame(self.fps),
            KeyframeTiming::Rel(at) => {
                self.keyframes.last().map(|k| k.frame).unwrap_or(0) + at.into_frame(self.fps)
            }
        };

        if frame == 0 {
            self.initial = Some(state.into());
            self
        } else {
            self.push_keyframe(Keyframe {
                frame,
                easing,
                state: state.into(),
            })
        }
    }

    pub fn push_keyframe(mut self, keyframe: Keyframe<T>) -> Self {
        self.keyframes.push(keyframe);
        self
    }

    pub fn hold(self, time: impl IntoFrame) -> Self {
        let frame = time.into_frame(self.fps);
        let initial = self.initial.as_ref().unwrap();
        let keyframe = if let Some(last) = self.keyframes.last().cloned() {
            Keyframe {
                state: last.state.clone(),
                easing: LINEAR,
                frame: last.frame + frame,
            }
        } else {
            Keyframe {
                state: initial.clone(),
                easing: LINEAR,
                frame,
            }
        };

        self.push_keyframe(keyframe)
    }

    pub fn build(self) -> AnimatedProperty<T> {
        AnimatedProperty::new(self.initial.unwrap(), self.keyframes)
    }
}
