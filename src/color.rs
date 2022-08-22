use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
#[derive(Clone, Copy)]

pub enum Color {
    Cyan,
    Yellow,
    Purple,
    Green,
    Red,
    Blue,
    Orange,
}
