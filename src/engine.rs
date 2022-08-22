use wasm_bindgen::prelude::*;

use crate::{
    collision::check_collision,
    shape::{Color, Shape},
    tetrimino::Tetrimino,
};

pub const WIDTH: usize = 8;
pub const HEIGHT: usize = 20;

#[wasm_bindgen]
pub struct Engine {
    board: [[Option<Color>; WIDTH]; HEIGHT],
    falling_tetrimino: Option<Tetrimino>,
    paused: bool,
    frames: u32,
}

#[wasm_bindgen]
impl Engine {
    pub fn new() -> Engine {
        // let t = Tetrimino::new(Shape::I, 4, 4);
        let t = Tetrimino::new(Shape::Q, 4, 4);
        let board = [[None; WIDTH]; HEIGHT];

        let mut engine = Engine {
            board,
            falling_tetrimino: Some(t),
            paused: false,
            frames: 0,
        };

        engine.board[19][0] = Some(Color::Yellow);
        engine.board[19][1] = Some(Color::Yellow);
        engine.board[19][4] = Some(Color::Yellow);
        engine.board[19][5] = Some(Color::Yellow);

        engine.board[18][0] = Some(Color::Yellow);
        engine.board[18][1] = Some(Color::Yellow);
        engine.board[18][4] = Some(Color::Yellow);
        engine.board[18][5] = Some(Color::Yellow);

        engine.board[17][2] = Some(Color::Yellow);
        engine.board[17][3] = Some(Color::Yellow);
        engine.board[17][4] = Some(Color::Yellow);
        engine.board[17][5] = Some(Color::Yellow);

        engine.board[16][2] = Some(Color::Yellow);
        engine.board[16][3] = Some(Color::Yellow);
        engine.board[15][2] = Some(Color::Yellow);
        engine.board[15][3] = Some(Color::Yellow);
        engine.board[14][2] = Some(Color::Yellow);
        engine.board[14][3] = Some(Color::Yellow);
        engine.board[13][2] = Some(Color::Yellow);
        engine.board[13][3] = Some(Color::Yellow);

        engine.set_current_tetrimino_pos();
        engine
    }

    pub fn tick(&mut self) {
        if self.paused {
            return;
        }

        if self.frames % 40 == 0 {
            if self.falling_tetrimino.is_some() {
                self.down()
            }
        }

        self.frames += 1;
    }

    pub fn left(&mut self) {
        if self.falling_tetrimino.is_some() {
            self.clear_current_tetrimino_pos();

            self.falling_tetrimino.as_mut().unwrap().pos.x -= 1;

            if !check_collision(self.falling_tetrimino.unwrap(), self.board) {
                self.set_current_tetrimino_pos()
            } else {
                self.falling_tetrimino.as_mut().unwrap().pos.x += 1;
                self.set_current_tetrimino_pos();
            }
        }
    }

    pub fn right(&mut self) {
        if self.falling_tetrimino.is_some() {
            self.clear_current_tetrimino_pos();

            self.falling_tetrimino.as_mut().unwrap().pos.x += 1;

            if !check_collision(self.falling_tetrimino.unwrap(), self.board) {
                self.set_current_tetrimino_pos()
            } else {
                self.falling_tetrimino.as_mut().unwrap().pos.x -= 1;
                self.set_current_tetrimino_pos();
            }
        }
    }

    pub fn down(&mut self) {
        // If there is an falling_tetrimino, update its position
        if self.falling_tetrimino.is_some() {
            // Clear the board at the tetriminos current position
            self.clear_current_tetrimino_pos();

            self.falling_tetrimino.as_mut().unwrap().pos.y += 1;

            if !check_collision(self.falling_tetrimino.unwrap(), self.board) {
                self.set_current_tetrimino_pos()
            } else {
                self.resolve_collision()
            }
        }
    }
    pub fn up(&mut self) {
        if self.falling_tetrimino.is_some() {
            self.clear_current_tetrimino_pos();
            self.falling_tetrimino.as_mut().unwrap().rotation += 1;

            self.set_current_tetrimino_pos()
        }
    }

    fn clear_current_tetrimino_pos(&mut self) {
        self.falling_tetrimino
            .as_ref()
            .unwrap()
            .get_squares()
            .iter()
            .for_each(|square| {
                self.board[square.y as usize][square.x as usize] = None;
            });
        // collision should already have checked coords are valid
    }

    fn set_current_tetrimino_pos(&mut self) {
        let color = self.get_color();
        self.falling_tetrimino
            .as_ref()
            .unwrap()
            .get_squares()
            .iter()
            .for_each(|square| self.board[square.y as usize][square.x as usize] = color);
        // collision should already have checked usize are valid
    }

    fn resolve_collision(&mut self) {
        // Move tetrimino back to free space
        self.falling_tetrimino.as_mut().unwrap().pos.y -= 1;
        self.set_current_tetrimino_pos();
        self.clear_full_rows();
        self.falling_tetrimino = Some(Tetrimino::spawn());

        self.set_current_tetrimino_pos();
    }

    fn clear_full_rows(&mut self) {
        self.board
            .clone()
            .iter()
            .enumerate()
            .for_each(|(index, row)| {
                if row.iter().all(|square| square.is_some()) {
                    self.falling_tetrimino = None;
                    let board_copy = self.board;

                    for i in (1..=index).rev() {
                        self.board[i] = board_copy[i - 1]
                    }
                }
            })
    }

    fn get_color(&self) -> Option<Color> {
        match self.falling_tetrimino {
            Some(tetrimino) => Some(tetrimino.shape.color()),
            None => None,
        }
    }

    pub fn tetrimino(&self) -> Option<Color> {
        // Some(self.falling_tetrimino.unwrap().shape.color())
        // Some(Color::Red)
        None
    }

    pub fn width(&self) -> usize {
        WIDTH
    }

    pub fn height(&self) -> usize {
        HEIGHT
    }

    pub fn board(&self) -> *const [Option<Color>; WIDTH] {
        self.board.as_ptr()
    }

    pub fn toggle_pause(&mut self) -> bool {
        self.paused = !self.paused;
        self.paused
    }

    pub fn reset(&mut self) {
        let t = Tetrimino::new(Shape::Q, 1, 3);
        let board = [[None; WIDTH]; HEIGHT];
        self.falling_tetrimino = Some(t);
        self.board = board;
    }
}
