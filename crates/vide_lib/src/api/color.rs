use super::animation::Interpolate;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color {
    pub r: f64,
    pub g: f64,
    pub b: f64,
    pub a: f64,
}

impl Color {
    pub const WHITE: Color = Color { r: 1.0, g: 1.0, b: 1.0, a: 1.0 };
    pub const BLACK: Color = Color { r: 0.0, g: 0.0, b: 0.0, a: 1.0 };
    pub const TRANSPARENT: Color = Color { r: 1.0, g: 1.0, b: 1.0, a: 0.0 };

    pub fn new(r: f64, g: f64, b: f64, a: f64) -> Self {
        Self::from_raw(r.powf(2.2), g.powf(2.2), b.powf(2.2), a)
    }

    pub const fn from_raw(r: f64, g: f64, b: f64, a: f64) -> Self {
        Self {
            r,
            g,
            b,
            a,
        }
    }
}

impl Interpolate for Color {
    fn interpolate(a: Self, b: Self, t: f64) -> Self {
        Self {
            r: f64::interpolate(a.r, b.r, t),
            g: f64::interpolate(a.g, b.g, t),
            b: f64::interpolate(a.b, b.b, t),
            a: f64::interpolate(a.a, b.a, t),
        }
    }
}

impl From<Color> for [f32; 4] {
    fn from(col: Color) -> Self {
        [col.r as f32, col.g as f32, col.b as f32, col.a as f32]
    }
}

impl From<Color> for [f64; 4] {
    fn from(col: Color) -> Self {
        [col.r, col.g, col.b, col.a]
    }
}

#[macro_export] macro_rules! rgb8 {
    ($r:expr, $g:expr, $b:expr) => {
        {
            use $crate::api::color::Color;
            Color::new($r as f64 / 255.0, $g as f64 / 255.0, $b as f64 / 255.0, 1.0)
        }
    };
}

#[macro_export] macro_rules! rgba8 {
    ($r:expr, $g:expr, $b:expr, $a:expr) => {
        {
            use $crate::api::color::Color;
            Color::new($r as f64 / 255.0, $g as f64 / 255.0, $b as f64 / 255.0, $a as f64 / 255.0)
        }
    };
}