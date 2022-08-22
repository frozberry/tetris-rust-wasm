use wasm_bindgen::prelude::wasm_bindgen;

#[derive(Debug, Clone, Copy)]
// Use copy instead of clone
#[wasm_bindgen]
pub struct Vec2 {
    pub x: usize,
    pub y: usize,
}

impl Vec2 {
    pub fn new(x: usize, y: usize) -> Self {
        Vec2 { x, y }
    }
}
