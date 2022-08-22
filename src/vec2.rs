use wasm_bindgen::prelude::wasm_bindgen;

#[derive(Debug, Clone, Copy)]
// Use copy instead of clone
#[wasm_bindgen]
pub struct Vec2 {
    pub x: i32,
    pub y: i32,
}

impl Vec2 {
    pub fn new(x: i32, y: i32) -> Self {
        Vec2 { x, y }
    }
}

pub struct Vec2Int {
    pub x: i32,
    pub y: i32,
}

impl Vec2Int {
    pub fn new(x: i32, y: i32) -> Self {
        Vec2Int { x, y }
    }
}
