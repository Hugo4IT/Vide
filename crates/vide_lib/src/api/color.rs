use super::animation::Interpolate;

/// Holds RGBA values converted to SRGB color space
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Color {
    /// Amount of red in the color
    pub r: f64,
    /// Amount of green in the color
    pub g: f64,
    /// Amount of blue in the color
    pub b: f64,
    /// How opaque this color is, `0.0` is completely transparent, `1.0` is fully opaque
    pub a: f64,
}

impl Color {
    /// Opaque white `(255, 255, 255, 255 / #ffffffff)`
    pub const WHITE: Color = Color { r: 1.0, g: 1.0, b: 1.0, a: 1.0 };
    /// Opaque black `(0, 0, 0, 255 / #000000ff)`
    pub const BLACK: Color = Color { r: 0.0, g: 0.0, b: 0.0, a: 1.0 };
    /// Opaque red `(255, 0, 0, 255 / #ff0000ff)`
    pub const RED: Color = Color { r: 1.0, g: 0.0, b: 0.0, a: 1.0 };
    /// Opaque green `(0, 255, 0, 255 / #00ff00ff)`
    pub const GREEN: Color = Color { r: 0.0, g: 1.0, b: 0.0, a: 1.0 };
    /// Opaque blue `(0, 0, 255, 255 / #0000ffff)`
    pub const BLUE: Color = Color { r: 0.0, g: 0.0, b: 1.0, a: 1.0 };
    /// Transparent white `(255, 255, 255 / #ffffff00)`, alias of Color::TRANSPARENT_WHITE
    pub const TRANSPARENT: Color = Self::TRANSPARENT_WHITE;
    /// Transparent white `(255, 255, 255 / #ffffff00)`
    pub const TRANSPARENT_WHITE: Color = Color { r: 1.0, g: 1.0, b: 1.0, a: 0.0 };
    /// Transparent black `(0, 0, 0, 0 / #00000000)`
    pub const TRANSPARENT_BLACK: Color = Color { r: 1.0, g: 1.0, b: 1.0, a: 0.0 };

    /// Create a new color from 4 linear components, it will automatically be converted to srgb at runtime
    pub fn new(r: f64, g: f64, b: f64, a: f64) -> Self {
        Self::from_raw(r.powf(2.2), g.powf(2.2), b.powf(2.2), a)
    }

    /// Create a new color from 4 srgb components. Only use this if you know what you're doing, otherwise use [`Color::new`]
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

impl From<&str> for Color {
    fn from(string: &str) -> Self {
        match string.chars().collect::<Vec<char>>().as_slice() {
            ['#', r, g, b] => Self::new(
                (r.to_digit(16).unwrap() + (r.to_digit(16).unwrap() << 4)) as f64 / 255.0,
                (g.to_digit(16).unwrap() + (g.to_digit(16).unwrap() << 4)) as f64 / 255.0,
                (b.to_digit(16).unwrap() + (b.to_digit(16).unwrap() << 4)) as f64 / 255.0,
                1.0,
            ),
            ['#', r, g, b, a] => Self::new(
                (r.to_digit(16).unwrap() + (r.to_digit(16).unwrap() << 4)) as f64 / 255.0,
                (g.to_digit(16).unwrap() + (g.to_digit(16).unwrap() << 4)) as f64 / 255.0,
                (b.to_digit(16).unwrap() + (b.to_digit(16).unwrap() << 4)) as f64 / 255.0,
                (a.to_digit(16).unwrap() + (a.to_digit(16).unwrap() << 4)) as f64 / 255.0,
            ),
            ['#', r2, r1, g2, g1, b2, b1] => Self::new(
                (r1.to_digit(16).unwrap() + (r2.to_digit(16).unwrap() << 4)) as f64 / 255.0,
                (g1.to_digit(16).unwrap() + (g2.to_digit(16).unwrap() << 4)) as f64 / 255.0,
                (b1.to_digit(16).unwrap() + (b2.to_digit(16).unwrap() << 4)) as f64 / 255.0,
                1.0,
            ),
            ['#', r2, r1, g2, g1, b2, b1, a2, a1] => Self::new(
                (r1.to_digit(16).unwrap() + (r2.to_digit(16).unwrap() << 4)) as f64 / 255.0,
                (g1.to_digit(16).unwrap() + (g2.to_digit(16).unwrap() << 4)) as f64 / 255.0,
                (b1.to_digit(16).unwrap() + (b2.to_digit(16).unwrap() << 4)) as f64 / 255.0,
                (a1.to_digit(16).unwrap() + (a2.to_digit(16).unwrap() << 4)) as f64 / 255.0,
            ),
            chars => match chars.into_iter().map(|c| *c as u8).collect::<Vec<u8>>().as_slice() {
                b"black" => Self::BLACK,
                b"white" => Self::WHITE,
                b"transparent" => Self::TRANSPARENT,
                b"transparent white" | b"transparent_white" => Self::TRANSPARENT_WHITE,
                b"transparent black" | b"transparent_black" => Self::TRANSPARENT_BLACK,
                b"red" => Self::RED,
                b"green" => Self::GREEN,
                b"blue" => Self::BLUE,
                _ => panic!("Unrecognized color: {}", string),
            }
        }
    }
}

/// Use this macro if you have a hex color you would like to use. Use
/// [`rgba8!(r, g, b, a)`] instead if you color isn't fully opaque.
/// 
/// ## Example
/// 
/// Let's say I want to use the hex color #da0037, I can do that by
/// writing the red, green and blue components as their hex values:
/// 
/// ```
/// # fn main() {
/// let the_best_color = rgb8!(0xda, 0x00, 0x37);
/// # }
/// ```
#[macro_export] macro_rules! rgb8 {
    ($r:expr, $g:expr, $b:expr) => {
        {
            use $crate::api::color::Color;
            Color::new($r as f64 / 255.0, $g as f64 / 255.0, $b as f64 / 255.0, 1.0)
        }
    };
}

/// Use this macro if you have a hex color you would like to use. Use
/// [`rgb8!(r, g, b)`] instead if you color is fully opaque.
/// 
/// ## Example
/// 
/// Let's say I want to use the hex color #da0037ee, I can do that by
/// writing the red, green and blue components as their hex values:
/// 
/// ```
/// # fn main() {
/// let the_best_color = rgba8!(0xda, 0x00, 0x37, 0xee);
/// # }
/// ```
#[macro_export] macro_rules! rgba8 {
    ($r:expr, $g:expr, $b:expr, $a:expr) => {
        {
            use $crate::api::color::Color;
            Color::new($r as f64 / 255.0, $g as f64 / 255.0, $b as f64 / 255.0, $a as f64 / 255.0)
        }
    };
}