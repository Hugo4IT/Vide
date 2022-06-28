#[macro_export] macro_rules! lerp {
    ($start:expr, $end:expr, $progress:expr) => {
        ((($end) - ($start)) * ($progress) + ($start))
    };
}

#[macro_export] macro_rules! cubic_bezier {
    ($x1:expr, $y1:expr, $x2:expr, $y2:expr) => {
        |t| {
            t * $y1 * (3.0 * (1.0 - t).powi(2)) + $y2 * (3.0 * (1.0 - t) * t.powi(2)) + t.powi(3)
        }
    };
}

#[macro_export] macro_rules! keyframes {
    (initial $initial:expr, $($at:expr => $ease:ident => $value:expr),*$(,)?) => {
        $crate::paste::paste! {
            $crate::api::animation::AnimatablePropertyBuilder::new($initial)
                $(.keyframe($crate::api::animation::Keyframe { frame: $at, easing: $crate::api::animation::[<EASE_ $ease>], state: $value }))*
                .build()
        }
    };
}

macro_rules! impl_interpolate {
    ($typ:ty) => {
        impl Interpolate for $typ {
            fn interpolate(a: Self, b: Self, t: f64) -> Self {
                ((b - a) as f64 * t) as $typ
            }
        }
    };
}

pub type EasingFunction = fn(f64)->f64;

/// `f(t)=t`
pub const EASE_LINEAR:      EasingFunction = |t|t;
/// `f(t)=t^2`
pub const EASE_OUT_QUADRATIC:   EasingFunction = |t|t*t;
/// `f(t)=t^3`
pub const EASE_OUT_CUBIC:       EasingFunction = |t|t*t*t;
/// `f(t)=t^4`
pub const EASE_OUT_QUARTIC:     EasingFunction = |t|t*t*t*t;
/// `f(t)=t^5`
pub const EASE_OUT_QUINTIC:     EasingFunction = |t|t*t*t*t*t;
/// `f(t)=t^10`
pub const EASE_OUT_EXPONENTIAL: EasingFunction = |t|t*t*t*t*t*t;
/// Overshoots
pub const EASE_IN_BACK:     EasingFunction = cubic_bezier!(0.69, -0.53, 0.06, 0.99);
/// Overshoots
pub const EASE_OUT_BACK:    EasingFunction = cubic_bezier!(0.42, 1.5, 0.35, 1.0);
/// Overshoots
pub const EASE_IN_OUT_BACK: EasingFunction = cubic_bezier!(0.84, -0.43, 0.11, 1.29);

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
        (
            A::interpolate(a.0, b.0, t),
            B::interpolate(a.1, b.1, t),
        )
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

impl<T: Interpolate + Clone> Keyframe<T> {
    pub fn evaluate(&self, previous: Keyframe<T>, frame: u64) -> T {
        let progress = frame as f64 / (self.frame - previous.frame) as f64;
        T::interpolate(previous.state, self.state.clone(), self.easing.clone()(progress))
    }
}

pub struct AnimatableProperty<T: Interpolate + Clone> {
    initial: T,
    keyframes: Vec<Keyframe<T>>,
}

impl<T: Interpolate + Clone> AnimatableProperty<T> {
    pub fn new(initial: T) -> Self {
        Self {
            initial,
            keyframes: Vec::new(),
        }
    }

    pub fn push_keyframe(&mut self, keyframe: Keyframe<T>) {
        self.keyframes.push(keyframe)
    }

    pub fn evaluate(&self, frame: u64) -> T {
        // Fallback when no keyframes
        if self.keyframes.len() == 0 {
            return self.initial.clone()
        }

        // When on first keyframe, interpolate with self.initial
        if self.keyframes.len() == 1 {
            let keyframe = self.keyframes.first().unwrap();
            if keyframe.frame >= frame {
                return keyframe.evaluate(Keyframe {
                    easing: EASE_LINEAR,
                    state: self.initial.clone(),
                    frame: 0,
                }, frame)
            }
        }

        // Interpolate between keyframes
        for keyframes in self.keyframes.windows(2) {
            let previous = keyframes[0].clone();
            let keyframe = keyframes[1].clone();
            if keyframe.frame >= frame {
                return keyframe.evaluate(previous, frame)
            }
        }

        // When all keyframes have passed
        if let Some(keyframe) = self.keyframes.last() {
            return keyframe.state.clone()
        }

        self.initial.clone() // Fallback
    }
}

pub struct AnimatablePropertyBuilder<T: Interpolate + Clone> {
    property: AnimatableProperty<T>,
}

impl<T: Interpolate + Clone> AnimatablePropertyBuilder<T> {
    pub fn new(initial: T) -> Self {
        Self {
            property: AnimatableProperty::new(initial),
        }
    }

    pub fn keyframe(mut self, keyframe: Keyframe<T>) -> Self {
        self.property.push_keyframe(keyframe);
        self
    }

    pub fn build(self) -> AnimatableProperty<T> {
        self.property
    }
}