use rand::seq::SliceRandom;
use wasm_bindgen::prelude::wasm_bindgen;

use crate::vec2::Vec2;

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
            Shape::I => Color::Cyan,
            Shape::Q => Color::Yellow,
            Shape::Z => Color::Red,
            Shape::S => Color::Green,
            Shape::T => Color::Purple,
            Shape::L => Color::Orange,
            Shape::J => Color::Blue,
        }
    }

    // Rotations from https://tetris.fandom.com/wiki/SRS?file=SRS-pieces.png
    pub fn local(&self, rotation: usize) -> [Vec2; 4] {
        let angle = rotation % 4;
        match self {
            Shape::Q => {
                let a = Vec2::new(0, 0);
                let b = Vec2::new(1, 0);
                let c = Vec2::new(0, 1);
                let d = Vec2::new(1, 1);

                [a, b, c, d]
            }
            Shape::Z => match angle {
                0 => {
                    let a = Vec2::new(0, 0);
                    let b = Vec2::new(1, 0);
                    let c = Vec2::new(0, -1);
                    let d = Vec2::new(-1, -1);

                    [a, b, c, d]
                }
                1 => {
                    let a = Vec2::new(0, 0);
                    let b = Vec2::new(0, 1);
                    let c = Vec2::new(1, 0);
                    let d = Vec2::new(1, -1);

                    [a, b, c, d]
                }
                2 => {
                    let a = Vec2::new(0, 0);
                    let b = Vec2::new(-1, 0);
                    let c = Vec2::new(0, 1);
                    let d = Vec2::new(1, 1);

                    [a, b, c, d]
                }
                3 => {
                    let a = Vec2::new(0, 0);
                    let b = Vec2::new(0, -1);
                    let c = Vec2::new(-1, 0);
                    let d = Vec2::new(-1, 1);

                    [a, b, c, d]
                }
                _ => panic!("Should never happen, angle should always be between 0 and 3"),
            },
            Shape::S => match angle {
                0 => {
                    let a = Vec2::new(0, 0);
                    let b = Vec2::new(-1, 0);
                    let c = Vec2::new(0, -1);
                    let d = Vec2::new(1, -1);

                    [a, b, c, d]
                }
                1 => {
                    let a = Vec2::new(0, 0);
                    let b = Vec2::new(0, -1);
                    let c = Vec2::new(1, 0);
                    let d = Vec2::new(1, 1);

                    [a, b, c, d]
                }
                2 => {
                    let a = Vec2::new(0, 0);
                    let b = Vec2::new(1, 0);
                    let c = Vec2::new(0, 1);
                    let d = Vec2::new(-1, 1);

                    [a, b, c, d]
                }
                3 => {
                    let a = Vec2::new(0, 0);
                    let b = Vec2::new(-1, 0);
                    let c = Vec2::new(0, 1);
                    let d = Vec2::new(-1, -1);

                    [a, b, c, d]
                }
                _ => panic!("Should never happen, angle should always be between 0 and 3"),
            },
            Shape::T => match angle {
                0 => {
                    let a = Vec2::new(0, 0);
                    let b = Vec2::new(1, 0);
                    let c = Vec2::new(-1, 0);
                    let d = Vec2::new(0, -1);

                    [a, b, c, d]
                }
                1 => {
                    let a = Vec2::new(0, 0);
                    let b = Vec2::new(1, 0);
                    let c = Vec2::new(0, 1);
                    let d = Vec2::new(0, -1);

                    [a, b, c, d]
                }
                2 => {
                    let a = Vec2::new(0, 0);
                    let b = Vec2::new(1, 0);
                    let c = Vec2::new(-1, 0);
                    let d = Vec2::new(0, 1);

                    [a, b, c, d]
                }
                3 => {
                    let a = Vec2::new(0, 0);
                    let b = Vec2::new(-1, 0);
                    let c = Vec2::new(0, 1);
                    let d = Vec2::new(0, -1);

                    [a, b, c, d]
                }
                _ => panic!("Should never happen, angle should always be between 0 and 3"),
            },
            Shape::I => match angle {
                0 => {
                    let a = Vec2::new(0, 0);
                    let b = Vec2::new(-1, 0);
                    let c = Vec2::new(1, 0);
                    let d = Vec2::new(2, 0);

                    [a, b, c, d]
                }
                1 => {
                    let a = Vec2::new(1, 0);
                    let b = Vec2::new(1, 1);
                    let c = Vec2::new(1, 2);
                    let d = Vec2::new(1, -1);

                    [a, b, c, d]
                }
                2 => {
                    let a = Vec2::new(0, 1);
                    let b = Vec2::new(-1, 1);
                    let c = Vec2::new(1, 1);
                    let d = Vec2::new(2, 1);

                    [a, b, c, d]
                }
                3 => {
                    let a = Vec2::new(0, 1);
                    let b = Vec2::new(0, 2);
                    let c = Vec2::new(0, -1);
                    let d = Vec2::new(0, 0);

                    [a, b, c, d]
                }
                _ => panic!("Should never happen, angle should always be between 0 and 3"),
            },
            Shape::L => match angle {
                0 => {
                    let a = Vec2::new(0, 0);
                    let b = Vec2::new(1, 0);
                    let c = Vec2::new(-1, 0);
                    let d = Vec2::new(1, -1);

                    [a, b, c, d]
                }
                1 => {
                    let a = Vec2::new(0, 0);
                    let b = Vec2::new(1, 1);
                    let c = Vec2::new(0, -1);
                    let d = Vec2::new(0, 1);

                    [a, b, c, d]
                }
                2 => {
                    let a = Vec2::new(0, 0);
                    let b = Vec2::new(1, 0);
                    let c = Vec2::new(-1, 0);
                    let d = Vec2::new(-1, 1);

                    [a, b, c, d]
                }
                3 => {
                    let a = Vec2::new(0, 0);
                    let b = Vec2::new(0, -1);
                    let c = Vec2::new(0, 1);
                    let d = Vec2::new(-1, -1);

                    [a, b, c, d]
                }
                _ => panic!("Should never happen, angle should always be between 0 and 3"),
            },
            Shape::J => match angle {
                0 => {
                    let a = Vec2::new(0, 0);
                    let b = Vec2::new(1, 0);
                    let c = Vec2::new(-1, 0);
                    let d = Vec2::new(-1, -1);

                    [a, b, c, d]
                }
                1 => {
                    let a = Vec2::new(0, 0);
                    let b = Vec2::new(1, -1);
                    let c = Vec2::new(0, -1);
                    let d = Vec2::new(0, 1);

                    [a, b, c, d]
                }
                2 => {
                    let a = Vec2::new(0, 0);
                    let b = Vec2::new(1, 0);
                    let c = Vec2::new(-1, 0);
                    let d = Vec2::new(1, 1);

                    [a, b, c, d]
                }
                3 => {
                    let a = Vec2::new(0, 0);
                    let b = Vec2::new(0, -1);
                    let c = Vec2::new(0, 1);
                    let d = Vec2::new(-1, 1);

                    [a, b, c, d]
                }
                _ => panic!("Should never happen, angle should always be between 0 and 3"),
            },
        }
    }
}
