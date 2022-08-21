mod utils;

use vec2::Vec2;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

mod vec2;

const WIDTH: u32 = 10;
const HEIGHT: u32 = 20;

#[wasm_bindgen]
pub struct Universe {
    // width: u32,
    // height: u32,
    board: [bool; (WIDTH * HEIGHT) as usize],
    falling_tetrimino: Vec2,
    paused: bool,
}

fn i(pos: Vec2) -> usize {
    (pos.y * WIDTH as usize + pos.x) as usize
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
        }
    }

    pub fn width(&self) -> u32 {
        WIDTH
    }

    pub fn height(&self) -> u32 {
        HEIGHT
    }

    pub fn board(&self) -> *const bool {
        self.board.as_ptr()
    }

    pub fn toggle_pause(&mut self) -> bool {
        self.paused = !self.paused;
        self.paused
    }

    pub fn tick(&mut self) {
        if self.paused {
            return;
        }

        self.board[i(self.falling_tetrimino)] = false;
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
    }

    fn check_bottom_row(&self) -> bool {
        self.falling_tetrimino.y == HEIGHT as usize - 1
    }

    fn check_collision(&self) -> bool {
        let square = i(self.falling_tetrimino);
        self.board[square]
    }
}
