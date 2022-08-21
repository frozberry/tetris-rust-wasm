#[derive(Debug, Clone, Copy)]
// Use copy instead of clone
pub struct Vec2 {
    pub x: usize,
    pub y: usize,
}

impl Vec2 {
    pub fn new(x: usize, y: usize) -> Self {
        Vec2 { x, y }
    }
}
