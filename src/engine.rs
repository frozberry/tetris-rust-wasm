use wasm_bindgen::prelude::*;

use crate::{collision::check_collision, color::Color, tetrimino::Tetrimino, vec2::Vec2};

pub const WIDTH: usize = 10;
pub const HEIGHT: usize = 20;

#[wasm_bindgen]
pub struct Engine {
    board: [[Color; WIDTH]; HEIGHT],
    falling_tetrimino: Tetrimino,
    paused: bool,
    frames: u32,
    ghost_squares: [Vec2; 4],
}

#[wasm_bindgen]
impl Engine {
    pub fn new() -> Engine {
        let ghost_squares = [
            Vec2::new(0, 0),
            Vec2::new(0, 0),
            Vec2::new(0, 0),
            Vec2::new(0, 0),
        ];
        let board = [[Color::Empty; WIDTH]; HEIGHT];
        let falling_tetrimino = Tetrimino::spawn();
        let mut engine = Engine {
            board,
            falling_tetrimino,
            paused: false,
            frames: 0,
            ghost_squares,
        };

        // engine.board[19][0] = Some(Color::Yellow);
        // engine.board[19][1] = Some(Color::Yellow);
        // engine.board[19][4] = Some(Color::Yellow);
        // engine.board[19][5] = Some(Color::Yellow);
        //
        // engine.board[18][0] = Some(Color::Yellow);
        // engine.board[18][1] = Some(Color::Yellow);
        // engine.board[18][4] = Some(Color::Yellow);
        // engine.board[18][5] = Some(Color::Yellow);

        // engine.board[17][2] = Some(Color::Yellow);
        // engine.board[17][3] = Some(Color::Yellow);
        // engine.board[17][4] = Some(Color::Yellow);
        // engine.board[17][5] = Some(Color::Yellow);

        // engine.board[16][2] = Some(Color::Yellow);
        // engine.board[16][3] = Some(Color::Yellow);
        // engine.board[15][2] = Some(Color::Yellow);
        // engine.board[15][3] = Some(Color::Yellow);
        // engine.board[14][2] = Some(Color::Yellow);
        // engine.board[14][3] = Some(Color::Yellow);
        // engine.board[13][2] = Some(Color::Yellow);
        // engine.board[13][3] = Some(Color::Yellow);

        engine.set_ghost();
        engine
    }

    pub fn tick(&mut self) {
        if self.paused {
            return;
        }

        if self.frames % 40 == 0 {
            // self.down()
        }

        self.frames += 1;
    }

    pub fn left(&mut self) {
        self.clear_current_tetrimino_pos();

        self.falling_tetrimino.pos.x -= 1;

        if !check_collision(self.falling_tetrimino, self.board) {
            self.set_ghost();
        } else {
            self.falling_tetrimino.pos.x += 1;
            self.set_current_tetrimino_pos();
        }
    }

    pub fn right(&mut self) {
        self.clear_current_tetrimino_pos();

        self.falling_tetrimino.pos.x += 1;

        if !check_collision(self.falling_tetrimino, self.board) {
            self.set_ghost();
        } else {
            self.falling_tetrimino.pos.x -= 1;
            self.set_current_tetrimino_pos()
        }
    }

    pub fn down(&mut self) {
        // If there is an falling_tetrimino, update its position
        // Clear the board at the tetriminos current position
        self.clear_current_tetrimino_pos();

        // Increment position
        self.falling_tetrimino.pos.y += 1;

        if !check_collision(self.falling_tetrimino, self.board) {
            self.set_current_tetrimino_pos()
        } else {
            self.resolve_collision()
        }
    }

    pub fn hard_down(&mut self) {
        loop {
            self.clear_current_tetrimino_pos();
            self.falling_tetrimino.pos.y += 1;
            if check_collision(self.falling_tetrimino, self.board) {
                self.resolve_collision();
                return;
            }
            self.set_current_tetrimino_pos()
        }
    }

    pub fn rotate_clockwise(&mut self) {
        self.clear_current_tetrimino_pos();
        self.falling_tetrimino.rotation += 1;

        if !check_collision(self.falling_tetrimino, self.board) {
            self.set_ghost();
        } else {
            self.falling_tetrimino.rotation -= 1;
            self.set_current_tetrimino_pos()
        }
    }

    pub fn rotate_counter_clockwise(&mut self) {
        self.clear_current_tetrimino_pos();
        self.falling_tetrimino.rotation -= 1;

        if !check_collision(self.falling_tetrimino, self.board) {
            self.set_current_tetrimino_pos()
        } else {
            self.falling_tetrimino.rotation += 1;
            self.set_current_tetrimino_pos()
        }
    }

    fn clear_current_tetrimino_pos(&mut self) {
        self.falling_tetrimino
            .get_squares()
            .iter()
            .for_each(|square| {
                self.board[square.y as usize][square.x as usize] = Color::Empty;
            });
        // collision should already have checked coords are valid
    }

    fn set_current_tetrimino_pos(&mut self) {
        let color = self.get_color();
        self.falling_tetrimino
            .get_squares()
            .iter()
            .for_each(|square| self.board[square.y as usize][square.x as usize] = color);
        // collision should already have checked usize are valid
    }

    fn resolve_collision(&mut self) {
        // Move tetrimino back to free space
        self.falling_tetrimino.pos.y -= 1;
        self.set_current_tetrimino_pos();
        self.clear_full_rows();
        self.spawn_tetrimino();
    }

    fn spawn_tetrimino(&mut self) {
        let tetrimino = Tetrimino::spawn();

        if check_collision(tetrimino, self.board) {
            self.reset();
            return;
        }
        self.falling_tetrimino = tetrimino;

        self.set_ghost();
    }

    fn set_ghost(&mut self) {
        // Clear the previous positions ghost
        self.clear_ghost();

        let mut ghost = self.falling_tetrimino.clone();

        // Clear board so ghost doesn't collide with it's own tetrimino
        self.clear_current_tetrimino_pos();

        loop {
            ghost.pos.y += 1;
            if check_collision(ghost, self.board) {
                ghost.pos.y -= 1;

                // Save the ghost squares so it can be cleared when the user moves the
                // tetrimino. This saves having to check every square in the board.
                for (i, square) in ghost.get_squares().iter().enumerate() {
                    self.ghost_squares[i] = Vec2::new(square.x, square.y);
                }

                // Update the board with the ghost squares
                for square in self.ghost_squares {
                    self.board[square.y as usize][square.x as usize] = Color::Ghost
                }

                break;
            }
        }

        self.set_current_tetrimino_pos();
    }

    fn clear_ghost(&mut self) {
        for square in self.ghost_squares {
            if !self.board[square.y as usize][square.x as usize].is_solid() {
                self.board[square.y as usize][square.x as usize] = Color::Empty;
            }
        }
    }

    fn clear_full_rows(&mut self) {
        self.board
            .clone()
            .iter()
            .enumerate()
            .for_each(|(index, row)| {
                if row.iter().all(|square| square.is_solid()) {
                    let board_copy = self.board;

                    for i in (1..=index).rev() {
                        self.board[i] = board_copy[i - 1]
                    }
                }
            })
    }

    fn get_color(&self) -> Color {
        self.falling_tetrimino.shape.color()
    }

    pub fn width(&self) -> usize {
        WIDTH
    }

    pub fn height(&self) -> usize {
        HEIGHT
    }

    pub fn board(&self) -> *const [Color; WIDTH] {
        self.board.as_ptr()
    }

    pub fn toggle_pause(&mut self) -> bool {
        self.paused = !self.paused;
        self.paused
    }

    pub fn reset(&mut self) {
        let tetrimino = Tetrimino::spawn();
        let board = [[Color::Empty; WIDTH]; HEIGHT];
        self.falling_tetrimino = tetrimino;
        self.board = board;
        self.set_current_tetrimino_pos();
    }
}
