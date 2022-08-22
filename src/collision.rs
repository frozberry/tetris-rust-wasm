use crate::{
    engine::{HEIGHT, WIDTH},
    shape::Color,
    tetrimino::Tetrimino,
};

pub fn check_down_collision(tetrimino: Tetrimino, board: [[Option<Color>; WIDTH]; HEIGHT]) -> bool {
    check_floor_collision(tetrimino) || check_tetrimino_collision(tetrimino, board)
}

fn check_tetrimino_collision(
    tetrimino: Tetrimino,
    board: [[Option<Color>; WIDTH]; HEIGHT],
) -> bool {
    tetrimino
        .get_squares()
        .iter()
        .any(|square| board[square.y][square.x].is_some())
}

fn check_floor_collision(tetrimino: Tetrimino) -> bool {
    tetrimino
        .get_squares()
        .iter()
        .any(|square| square.y >= HEIGHT)
}

fn check_left_wall_collision(tetrimino: Tetrimino) -> bool {
    tetrimino.get_squares().iter().any(|square| square.x < 0)
}

fn check_right_wall_collision(tetrimino: Tetrimino) -> bool {
    tetrimino
        .get_squares()
        .iter()
        .any(|square| square.x > WIDTH - 1)
}
