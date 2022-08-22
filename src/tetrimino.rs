use wasm_bindgen::prelude::wasm_bindgen;

use crate::{
    engine::{Engine, HEIGHT, WIDTH},
    shape::{Color, Shape},
    vec2::Vec2,
};

#[derive(Clone, Copy)]
#[wasm_bindgen]
pub struct Tetrimino {
    pub pos: Vec2,
    pub rotation: usize,
    pub shape: Shape,
}

impl Tetrimino {
    pub fn new(shape: Shape, x: usize, y: usize) -> Self {
        Tetrimino {
            pos: Vec2::new(x, y),
            shape,
            rotation: 0,
        }
    }
    pub fn spawn() -> Self {
        Tetrimino {
            pos: Vec2::new(0, 10),
            // shape: Shape::rand(),
            shape: Shape::I,
            rotation: 0,
        }
    }

    pub fn get_squares(&self) -> [Vec2; 4] {
        let local = self.shape.local(self.rotation);
        let mut world = [Vec2::new(0, 0); 4];
        for i in 0..4 {
            let x = local[i].x + self.pos.x as i32;
            let y = local[i].y + self.pos.y as i32;
            world[i] = Vec2::new(x as usize, y as usize);
        }
        world
    }
}
