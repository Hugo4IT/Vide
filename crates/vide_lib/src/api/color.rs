#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color {
    pub r: f64,
    pub g: f64,
    pub b: f64,
    pub a: f64,
}

impl Color {
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

#[macro_export] macro_rules! rgb8 {
    ($r:expr, $g:expr, $b:expr) => {
        {
            use $crate::api::color::Color;
            Color::new($r as f64 / 255.0, $g as f64 / 255.0, $b as f64 / 255.0, 1.0)
        }
    };
}