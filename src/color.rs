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
    Empty,
}

impl Color {
    // pub fn is_none(&self) -> bool {
    //     if let Color::Empty = self {
    //         return true;
    //     }
    //     return false;
    // }

    pub fn is_solid(&self) -> bool {
        if let Color::Empty = self {
            return false;
        }
        return true;
    }
}
