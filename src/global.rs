use crate::tetromino::*;
use macroquad::prelude::*;

pub const BLOCK_SIZE: f32 = 40.0;
pub const BOARD_WIDTH: usize = 10;
pub const BOARD_HEIGHT: usize = 20;
pub const GAME_WIDTH: usize = BOARD_WIDTH + 5;

pub const SHAPES: [Tetromino; 7] = [
    Tetromino {
        pos: Vec2::new(0.0, 0.0),
        shape: [
            [false, false, false, false],
            [true, true, true, true],
            [false, false, false, false],
            [false, false, false, false],
        ],
        color: (0, 190, 225),
    },
    Tetromino {
        pos: Vec2::new(0.0, 0.0),
        shape: [
            [false, false, false, false],
            [false, true, false, false],
            [false, true, true, true],
            [false, false, false, false],
        ],
        color: (60, 60, 230),
    },
    Tetromino {
        pos: Vec2::new(0.0, 0.0),
        shape: [
            [false, false, false, false],
            [false, false, false, true],
            [false, true, true, true],
            [false, false, false, false],
        ],
        color: (220, 150, 50),
    },
    Tetromino {
        pos: Vec2::new(0.0, 0.0),
        shape: [
            [false, false, false, false],
            [false, true, true, false],
            [false, true, true, false],
            [false, false, false, false],
        ],
        color: (240, 215, 0),
    },
    Tetromino {
        pos: Vec2::new(0.0, 0.0),
        shape: [
            [false, false, false, false],
            [false, false, true, true],
            [false, true, true, false],
            [false, false, false, false],
        ],
        color: (135, 220, 130),
    },
    Tetromino {
        pos: Vec2::new(0.0, 0.0),
        shape: [
            [false, false, false, false],
            [false, false, true, false],
            [false, true, true, true],
            [false, false, false, false],
        ],
        color: (110, 40, 230),
    },
    Tetromino {
        pos: Vec2::new(0.0, 0.0),
        shape: [
            [false, false, false, false],
            [false, true, true, false],
            [false, false, true, true],
            [false, false, false, false],
        ],
        color: (220, 60, 90),
    },
];

pub fn new_board() -> Vec<Vec<(u8, u8, u8)>> {
    return vec![vec![(0, 0, 0); BOARD_WIDTH]; BOARD_HEIGHT];
}

pub fn new_tetromino() -> Tetromino {
    let random = rand::gen_range(0, SHAPES.len());
    return Tetromino::new(
        Vec2::new((BOARD_WIDTH / 2 - 2) as f32, -1.0),
        SHAPES[random].shape,
        SHAPES[random].color,
    );
}

pub fn draw_block(x: f32, y: f32, color: (u8, u8, u8)) {
    let x_loc = x * BLOCK_SIZE;
    let y_loc = y * BLOCK_SIZE;

    draw_rectangle(
        x_loc + 2.0,
        y_loc + 2.0,
        BLOCK_SIZE,
        BLOCK_SIZE,
        Color::from_rgba(color.0 / 3, color.1 / 3, color.2 / 3, 150),
    );

    draw_rectangle(
        x_loc,
        y_loc,
        BLOCK_SIZE,
        BLOCK_SIZE,
        Color::from_rgba(color.0 / 2, color.1 / 2, color.2 / 2, 255),
    );

    for i in 0..10 {
        let factor = i as f32 / 10.0;
        let red =
            (color.0 as f32 * (1.0 - factor) + (color.0 as f32 * 1.2).min(255.0) * factor) as u8;
        let green =
            (color.1 as f32 * (1.0 - factor) + (color.1 as f32 * 1.2).min(255.0) * factor) as u8;
        let blue =
            (color.2 as f32 * (1.0 - factor) + (color.2 as f32 * 1.2).min(255.0) * factor) as u8;
        draw_rectangle(
            x_loc + 1.0,
            y_loc + 1.0 + (BLOCK_SIZE - 2.0) * factor,
            BLOCK_SIZE - 2.0,
            (BLOCK_SIZE - 2.0) / 10.0,
            Color::from_rgba(red, green, blue, 255),
        );
    }

    draw_rectangle(
        x_loc + 3.0,
        y_loc + 3.0,
        BLOCK_SIZE - 6.0,
        BLOCK_SIZE - 6.0,
        Color::from_rgba(color.0, color.1, color.2, 255),
    );
}