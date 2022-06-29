pub mod clip;
pub mod render;
pub mod io;
pub mod api;
pub mod effect;

pub use paste;
pub use cgmath;

pub mod prelude {
    pub use super::api::rect::Rect;
    pub use super::api::transform::Transform;
    pub use super::api::animation::AnimatedPropertyBuilder as Animation;
    pub use super::api::animation::KeyframeTiming::*;
    pub use super::api::animation::ease::*;
    pub use super::api::video::*;
    pub use super::api::color::*;
    pub use super::cubic_bezier;
    pub use super::rgba8;
    pub use super::rgb8;
    pub use super::lerp;
}