mod utils;

use input::Input;
use vec2::Vec2;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

mod input;
mod vec2;

const WIDTH: usize = 10;
const HEIGHT: usize = 20;

fn i(pos: Vec2) -> usize {
    (pos.y * WIDTH as usize + pos.x) as usize
}

#[wasm_bindgen]
pub struct Universe {
    // width: u32,
    // height: u32,
    board: [bool; (WIDTH * HEIGHT) as usize],
    falling_tetrimino: Vec2,
    paused: bool,
    input: Option<Input>,
}

#[wasm_bindgen]
impl Universe {
    pub fn new() -> Universe {
        let t = Vec2::new(2, 3);
        let board = [false; (WIDTH * HEIGHT) as usize];

        Universe {
            board,
            falling_tetrimino: t,
            paused: false,
            input: None,
        }
    }

    pub fn width(&self) -> usize {
        WIDTH
    }

    pub fn height(&self) -> usize {
        HEIGHT
    }

    pub fn board(&self) -> *const bool {
        self.board.as_ptr()
    }

    pub fn left(&mut self) {
        self.input = Some(Input::Left)
    }

    pub fn right(&mut self) {
        self.input = Some(Input::Right)
    }

    pub fn down(&mut self) {
        self.input = Some(Input::Down)
    }

    pub fn toggle_pause(&mut self) -> bool {
        self.paused = !self.paused;
        self.paused
    }

    pub fn reset(&mut self) {
        let t = Vec2::new(2, 3);
        let board = [false; (WIDTH * HEIGHT) as usize];
        self.falling_tetrimino = t;
        self.board = board;
    }

    pub fn tick(&mut self) {
        if self.paused {
            return;
        }
        self.board[i(self.falling_tetrimino)] = false;

        if let Some(input) = &self.input {
            match input {
                Input::Down => self.falling_tetrimino.y += 1,
                Input::Left => self.falling_tetrimino.x -= 1,
                Input::Right => self.falling_tetrimino.x += 1,
            }
        }

        self.falling_tetrimino.y += 1;

        if self.check_collision() {
            self.falling_tetrimino.y -= 1;
            self.board[i(self.falling_tetrimino)] = true;
            self.falling_tetrimino.y = 10;
            return;
        }

        if self.check_bottom_row() {
            self.board[i(self.falling_tetrimino)] = true;
            self.falling_tetrimino.y = 10;
        }

        self.board[i(self.falling_tetrimino)] = true;

        self.input = None;
    }

    fn check_bottom_row(&self) -> bool {
        self.falling_tetrimino.y == HEIGHT as usize - 1
    }

    fn check_collision(&self) -> bool {
        let square = i(self.falling_tetrimino);
        self.board[square]
    }
}
