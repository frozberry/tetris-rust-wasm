use crate::{
    color::Color,
    engine::{HEIGHT, WIDTH},
    tetrimino::Tetrimino,
};

pub fn check_collision(tetrimino: Tetrimino, board: [[Option<Color>; WIDTH]; HEIGHT]) -> bool {
    check_out_of_bounds(tetrimino) || check_tetrimino_collision(tetrimino, board)
}

fn check_tetrimino_collision(
    tetrimino: Tetrimino,
    board: [[Option<Color>; WIDTH]; HEIGHT],
) -> bool {
    tetrimino
        .get_squares()
        .iter()
        .any(|square| board[square.y as usize][square.x as usize].is_some())
}

fn check_out_of_bounds(tetrimino: Tetrimino) -> bool {
    tetrimino.get_squares().iter().any(|square| {
        square.x < 0 || square.x >= WIDTH as i32 || square.y < 0 || square.y >= HEIGHT as i32
    })
}
