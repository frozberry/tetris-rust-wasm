use wasm_bindgen::prelude::*;

use crate::{shape::Shape, tetrimino::Tetrimino};

const WIDTH: usize = 4;
const HEIGHT: usize = 20;

#[wasm_bindgen]
pub struct Engine {
    board: [[bool; WIDTH]; HEIGHT],
    falling_tetrimino: Option<Tetrimino>,
    paused: bool,
    frames: u32,
}

#[wasm_bindgen]
impl Engine {
    pub fn new() -> Engine {
        let t = Tetrimino::new(Shape::Q, 1, 3);
        let board = [[false; WIDTH]; HEIGHT];

        Engine {
            board,
            falling_tetrimino: Some(t),
            paused: false,
            frames: 0,
        }
    }

    pub fn width(&self) -> usize {
        WIDTH
    }

    pub fn height(&self) -> usize {
        HEIGHT
    }

    pub fn board(&self) -> *const [bool; WIDTH] {
        self.board.as_ptr()
    }

    pub fn left(&mut self) {
        if self.falling_tetrimino.is_some() {
            if self
                .falling_tetrimino
                .as_ref()
                .unwrap()
                .get_squares()
                .iter()
                .all(|square| square.x > 0)
            {
                self.clear_current_square();
                self.falling_tetrimino.as_mut().unwrap().pos.x -= 1;
                self.set_current_square();
            }
        }
    }

    pub fn right(&mut self) {
        if self.falling_tetrimino.is_some() {
            if self
                .falling_tetrimino
                .as_ref()
                .unwrap()
                .get_squares()
                .iter()
                .all(|square| square.x <= WIDTH - 2)
            {
                self.clear_current_square();
                self.falling_tetrimino.as_mut().unwrap().pos.x += 1;
                self.set_current_square();
            }
        }
    }

    pub fn down(&mut self) {
        if self.falling_tetrimino.is_some() {
            self.clear_current_square();
            if self.falling_tetrimino.as_ref().unwrap().pos.y <= HEIGHT - 2 {
                self.falling_tetrimino.as_mut().unwrap().pos.y += 1;

                // Copied collision detecion from tick
                if self.check_bottom_row() || self.check_collision() {
                    self.resolve_collision();
                    return;
                }
            }
            self.set_current_square();
        }
    }

    fn clear_current_square(&mut self) {
        self.falling_tetrimino
            .as_ref()
            .unwrap()
            .get_squares()
            .iter()
            .for_each(|square| {
                self.board[square.y][square.x] = false;
            });
    }

    fn set_current_square(&mut self) {
        self.falling_tetrimino
            .as_ref()
            .unwrap()
            .get_squares()
            .iter()
            .for_each(|square| {
                self.board[square.y][square.x] = true;
            });
    }

    pub fn toggle_pause(&mut self) -> bool {
        self.paused = !self.paused;
        self.paused
    }

    pub fn reset(&mut self) {
        let t = Tetrimino::new(Shape::Q, 1, 3);
        let board = [[false; WIDTH]; HEIGHT];
        self.falling_tetrimino = Some(t);
        self.board = board;
    }

    pub fn tick(&mut self) {
        if self.paused {
            return;
        }

        if self.frames % 40 == 0 {
            if self.falling_tetrimino.is_some() {
                // If there is an falling_tetrimino, update its position
                self.falling_tetrimino
                    .as_ref()
                    .unwrap()
                    .get_squares()
                    .iter()
                    .for_each(|square| {
                        self.board[square.y][square.x] = false;
                    });

                self.falling_tetrimino.as_mut().unwrap().pos.y += 1;

                if !self.check_collision() {
                    self.falling_tetrimino
                        .as_ref()
                        .unwrap()
                        .get_squares()
                        .iter()
                        .for_each(|square| {
                            self.board[square.y][square.x] = true;
                        });
                } else {
                    self.resolve_collision()
                }
            }
        }

        self.frames += 1;
    }

    fn check_collision(&self) -> bool {
        self.check_bottom_row() || self.check_tetrimino_collision()
    }

    fn check_bottom_row(&self) -> bool {
        self.falling_tetrimino
            .as_ref()
            .unwrap()
            .get_squares()
            .iter()
            .any(|square| square.y > HEIGHT - 1)
    }

    fn check_tetrimino_collision(&self) -> bool {
        self.falling_tetrimino
            .as_ref()
            .unwrap()
            .get_squares()
            .iter()
            .any(|square| self.board[square.y][square.x])
    }

    fn resolve_collision(&mut self) {
        self.falling_tetrimino.as_mut().unwrap().pos.y -= 1;
        self.falling_tetrimino
            .as_ref()
            .unwrap()
            .get_squares()
            .iter()
            .for_each(|square| {
                self.board[square.y][square.x] = true;
            });

        self.clear_full_rows();
        let t = Tetrimino::spawn();
        self.falling_tetrimino = Some(t);
    }

    fn clear_full_rows(&mut self) {
        self.board
            .clone()
            .iter()
            .enumerate()
            .for_each(|(index, row)| {
                if row.iter().all(|square| *square) {
                    self.falling_tetrimino = None;
                    let board_copy = self.board;

                    for i in (1..=index).rev() {
                        self.board[i] = board_copy[i - 1]
                    }
                }
            })
    }
}
