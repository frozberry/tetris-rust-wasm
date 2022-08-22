use rand::seq::SliceRandom;
use wasm_bindgen::prelude::wasm_bindgen;

use crate::vec2::Vec2Int;

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
    Z,
    S,
    T,
    I,
    L,
    J,
}

impl Shape {
    pub fn rand() -> Self {
        let mut rng = rand::thread_rng();
        *[
            Shape::Q,
            Shape::Z,
            Shape::S,
            Shape::T,
            Shape::I,
            Shape::L,
            Shape::J,
        ]
        .choose(&mut rng)
        .unwrap()
    }

    pub fn color(&self) -> Color {
        match self {
            Shape::Q => Color::Red,
            Shape::I => Color::Blue,
            _ => Color::Blue,
        }
    }

    pub fn local(&self) -> [Vec2Int; 4] {
        match self {
            Shape::Q => {
                let a = Vec2Int::new(0, 0);
                let b = Vec2Int::new(1, 0);
                let c = Vec2Int::new(0, 1);
                let d = Vec2Int::new(1, 1);

                [a, b, c, d]
            }
            Shape::Z => {
                let a = Vec2Int::new(1, 1);
                let b = Vec2Int::new(2, 1);
                let c = Vec2Int::new(0, 0);
                let d = Vec2Int::new(1, 0);

                [a, b, c, d]
            }
            Shape::S => {
                let a = Vec2Int::new(0, 0);
                let b = Vec2Int::new(1, 0);
                let c = Vec2Int::new(1, 1);
                let d = Vec2Int::new(2, 1);

                [a, b, c, d]
            }
            Shape::T => {
                let a = Vec2Int::new(1, 1);
                let b = Vec2Int::new(0, 0);
                let c = Vec2Int::new(1, 0);
                let d = Vec2Int::new(2, 0);

                [a, b, c, d]
            }
            Shape::I => {
                let a = Vec2Int::new(0, 0);
                let b = Vec2Int::new(1, 0);
                let c = Vec2Int::new(2, 0);
                let d = Vec2Int::new(3, 0);

                [a, b, c, d]
            }
            Shape::L => {
                let a = Vec2Int::new(0, 0);
                let b = Vec2Int::new(0, 1);
                let c = Vec2Int::new(0, 2);
                let d = Vec2Int::new(1, 0);

                [a, b, c, d]
            }
            Shape::J => {
                let a = Vec2Int::new(0, 0);
                let b = Vec2Int::new(1, 0);
                let c = Vec2Int::new(1, 1);
                let d = Vec2Int::new(1, 2);

                [a, b, c, d]
            }
        }
    }
}
