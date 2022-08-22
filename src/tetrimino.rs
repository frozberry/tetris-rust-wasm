use crate::{vec2::Vec2, shape::Shape};

pub struct Tetrimino {
    pub pos: Vec2,
    pub shape: Shape,
}

impl Tetrimino {
    pub fn new(shape: Shape, x: usize, y: usize) -> Self {
        Tetrimino {
            pos: Vec2::new(x, y),
            shape,
        }
    }
    pub fn spawn() -> Self {
        Tetrimino {
            pos: Vec2::new(0, 10),
            shape: Shape::rand(),
        }
    }

    pub fn get_squares(&self) -> [Vec2; 4] {
        match self.shape {
            Shape::Q => {
                let a = Vec2::new(self.pos.x, self.pos.y);
                let b = Vec2::new(self.pos.x + 1, self.pos.y);
                let c = Vec2::new(self.pos.x, self.pos.y - 1);
                let d = Vec2::new(self.pos.x + 1, self.pos.y - 1);

                [a, b, c, d]
            }
            Shape::I => {
                let a = Vec2::new(self.pos.x, self.pos.y);
                let b = Vec2::new(self.pos.x + 1, self.pos.y);
                let c = Vec2::new(self.pos.x + 2, self.pos.y);
                let d = Vec2::new(self.pos.x + 3, self.pos.y);

                [a, b, c, d]
            }
        }
    }
}
