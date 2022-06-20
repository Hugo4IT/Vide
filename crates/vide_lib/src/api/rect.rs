use super::color::Color;

pub struct Rect {
    is_initialized: bool,
    pub color: Color,
}

impl Default for Rect {
    fn default() -> Self {
        Self {
            is_initialized: false,
        }
    }
}