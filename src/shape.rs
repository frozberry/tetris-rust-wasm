use rand::seq::SliceRandom;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
#[derive(Clone, Copy)]
pub enum Color {
    Red,
    Blue,
}

#[wasm_bindgen]
#[derive(Clone, Copy)]
pub enum Shape {
    Q,
    I,
}

impl Shape {
    pub fn rand() -> Self {
        let mut rng = rand::thread_rng();
        *[Shape::Q, Shape::I].choose(&mut rng).unwrap()
    }

    pub fn color(&self) -> Color {
        match self {
            Shape::Q => Color::Red,
            Shape::I => Color::Blue,
        }
    }
}
