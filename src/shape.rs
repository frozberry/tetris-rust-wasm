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

    pub fn local(&self, rotation: usize) -> [Vec2Int; 4] {
        let angle = rotation % 4;
        match self {
            Shape::Q => {
                let a = Vec2Int::new(0, 0);
                let b = Vec2Int::new(1, 0);
                let c = Vec2Int::new(0, 1);
                let d = Vec2Int::new(1, 1);

                [a, b, c, d]
            }
            Shape::Z => match angle {
                0 => {
                    let a = Vec2Int::new(0, 0);
                    let b = Vec2Int::new(1, 0);
                    let c = Vec2Int::new(0, -1);
                    let d = Vec2Int::new(-1, -1);

                    [a, b, c, d]
                }
                1 => {
                    let a = Vec2Int::new(0, 0);
                    let b = Vec2Int::new(0, 1);
                    let c = Vec2Int::new(1, 0);
                    let d = Vec2Int::new(1, -1);

                    [a, b, c, d]
                }
                2 => {
                    let a = Vec2Int::new(0, 0);
                    let b = Vec2Int::new(-1, 0);
                    let c = Vec2Int::new(0, 1);
                    let d = Vec2Int::new(1, 1);

                    [a, b, c, d]
                }
                3 => {
                    let a = Vec2Int::new(0, 0);
                    let b = Vec2Int::new(0, -1);
                    let c = Vec2Int::new(-1, 0);
                    let d = Vec2Int::new(-1, 1);

                    [a, b, c, d]
                }
                _ => panic!("Should never happen, angle should always be between 0 and 3"),
            },
            Shape::S => match angle {
                0 => {
                    let a = Vec2Int::new(0, 0);
                    let b = Vec2Int::new(-1, 0);
                    let c = Vec2Int::new(0, -1);
                    let d = Vec2Int::new(1, -1);

                    [a, b, c, d]
                }
                1 => {
                    let a = Vec2Int::new(0, 0);
                    let b = Vec2Int::new(0, -1);
                    let c = Vec2Int::new(1, 0);
                    let d = Vec2Int::new(1, 1);

                    [a, b, c, d]
                }
                2 => {
                    let a = Vec2Int::new(0, 0);
                    let b = Vec2Int::new(1, 0);
                    let c = Vec2Int::new(0, 1);
                    let d = Vec2Int::new(-1, 1);

                    [a, b, c, d]
                }
                3 => {
                    let a = Vec2Int::new(0, 0);
                    let b = Vec2Int::new(-1, 0);
                    let c = Vec2Int::new(0, 1);
                    let d = Vec2Int::new(-1, -1);

                    [a, b, c, d]
                }
                _ => panic!("Should never happen, angle should always be between 0 and 3"),
            },
            Shape::T => match angle {
                0 => {
                    let a = Vec2Int::new(0, 0);
                    let b = Vec2Int::new(1, 0);
                    let c = Vec2Int::new(-1, 0);
                    let d = Vec2Int::new(0, -1);

                    [a, b, c, d]
                }
                1 => {
                    let a = Vec2Int::new(0, 0);
                    let b = Vec2Int::new(1, 0);
                    let c = Vec2Int::new(0, 1);
                    let d = Vec2Int::new(0, -1);

                    [a, b, c, d]
                }
                2 => {
                    let a = Vec2Int::new(0, 0);
                    let b = Vec2Int::new(1, 0);
                    let c = Vec2Int::new(-1, 0);
                    let d = Vec2Int::new(0, 1);

                    [a, b, c, d]
                }
                3 => {
                    let a = Vec2Int::new(0, 0);
                    let b = Vec2Int::new(-1, 0);
                    let c = Vec2Int::new(0, 1);
                    let d = Vec2Int::new(0, -1);

                    [a, b, c, d]
                }
                _ => panic!("Should never happen, angle should always be between 0 and 3"),
            },
            Shape::I => match angle {
                0 => {
                    let a = Vec2Int::new(0, 0);
                    let b = Vec2Int::new(1, 0);
                    let c = Vec2Int::new(2, 0);
                    let d = Vec2Int::new(3, 0);

                    [a, b, c, d]
                }
                1 => {
                    let a = Vec2Int::new(0, 1);
                    let b = Vec2Int::new(0, 2);
                    let c = Vec2Int::new(0, 3);
                    let d = Vec2Int::new(0, 4);

                    [a, b, c, d]
                }
                2 => {
                    let a = Vec2Int::new(0, 0);
                    let b = Vec2Int::new(1, 0);
                    let c = Vec2Int::new(2, 0);
                    let d = Vec2Int::new(3, 0);

                    [a, b, c, d]
                }
                3 => {
                    let a = Vec2Int::new(0, 1);
                    let b = Vec2Int::new(0, 2);
                    let c = Vec2Int::new(0, 3);
                    let d = Vec2Int::new(0, 4);

                    [a, b, c, d]
                }
                _ => panic!("Should never happen, angle should always be between 0 and 3"),
            },
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
