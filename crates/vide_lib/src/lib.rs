// #![warn(missing_docs)]

// TODO: Add docs for these modules

pub mod api;
pub mod clip;
pub mod effect;
pub mod io;
pub mod render;

pub use cgmath;
pub use paste;

/// Contains everything you need to get started with Vide, just `use vide::prelude::*` and you're set!
pub mod prelude {
    pub use super::api::animation::ease;
    pub use super::api::animation::AnimatedPropertyBuilder as Animation;
    pub use super::api::animation::KeyframeTiming::*;
    pub use super::api::color::*;
    pub use super::api::rect::Rect;
    pub use super::api::transform::Transform;
    pub use super::api::video::*;
    pub use super::cubic_bezier;
    pub use super::lerp;
    pub use super::rgb8;
    pub use super::rgba8;
    pub use super::unanimated;
}
