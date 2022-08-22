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
    pub fn new(shape: Shape, x: i32, y: i32) -> Self {
        Tetrimino {
            pos: Vec2::new(x, y),
            shape,
            rotation: 0,
        }
    }

    pub fn spawn() -> Self {
        let shape = Shape::rand();
        let pos = shape.get_spawn_pos();

        Tetrimino {
            pos,
            shape,
            rotation: 0,
        }
    }

    pub fn get_squares(&self) -> [Vec2; 4] {
        let local = self.shape.get_local_coords(self.rotation);
        let mut world = [Vec2::new(0, 0); 4];
        for i in 0..4 {
            let x = local[i].x + self.pos.x as i32;
            let y = local[i].y + self.pos.y as i32;
            world[i] = Vec2::new(x as i32, y as i32);
        }
        world
    }
}
