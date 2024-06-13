use crate::global::*;
use crate::tetromino::*;
use macroquad::prelude::*;
use rand::ChooseRandom;

pub struct Board {
    pub controls: Vec<KeyCode>,
    pub offset: u32,

    pub t1: Tetromino,
    pub t2: Tetromino,
    pub p1: Tetromino,

    pub bag: Vec<Tetromino>,
    pub board: Vec<Vec<(u8, u8, u8)>>,
    pub direction: Vec2,

    pub horizontal_delay: f64,
    pub drop_delay: f64,
    pub gravity_delay: f64,

    pub last_horizontal: f64,
    pub last_drop: f64,
    pub last_gravity: f64,
}

impl Board {
    pub fn new(controls: Vec<KeyCode>, offset: u32) -> Self {
        let mut board = Board {
            controls,
            offset: offset * GAME_WIDTH as u32,

            t1: Tetromino::new(SHAPES[0].width, SHAPES[0].shape, SHAPES[0].color, None),
            t2: Tetromino::new(SHAPES[0].width, SHAPES[0].shape, SHAPES[0].color, None),
            p1: Tetromino::new(SHAPES[0].width, SHAPES[0].shape, SHAPES[0].color, None),

            bag: Vec::new(),
            board: vec![vec![BOARD_COLOR; BOARD_WIDTH]; BOARD_HEIGHT],
            direction: Vec2::new(0.0, 0.0),

            horizontal_delay: 110.0,
            drop_delay: 20.0,
            gravity_delay: 1000.0,

            last_horizontal: 0.0,
            last_drop: 0.0,
            last_gravity: 0.0,
        };

        board.refill_bag();
        board
    }

    pub fn run(&mut self) {
        self.input();
        self.update_phantom();
        self.game_loop();
        self.draw();
    }

    fn refill_bag(&mut self) {
        if self.bag.is_empty() {
            self.bag = SHAPES
                .iter()
                .map(|shape| Tetromino::new(shape.width, shape.shape, shape.color, None))
                .collect();
            self.bag.shuffle();
        }
        self.t1 = self.t2;
        self.t1.pos = DEFAULT_TETROMINO_POS;
        self.t2 = self.bag.pop().unwrap();
        self.t2.pos = NEXT_TETROMINO_POS;
    }

    fn input(&mut self) {
        let time = get_current_time() as f64;
        self.direction = Vec2::new(0.0, 0.0);

        if time - self.last_horizontal >= self.horizontal_delay {
            if is_key_down(self.controls[0]) {
                self.direction.x = -1.0;
                self.last_horizontal = time;
            }
            if is_key_down(self.controls[1]) {
                self.direction.x = 1.0;
                self.last_horizontal = time;
            }
        }

        if time - self.last_drop >= self.drop_delay {
            if is_key_down(self.controls[2]) {
                self.direction.y = 1.0;
                self.last_drop = time;
            }
        }

        if is_key_pressed(self.controls[3]) {
            self.rotate_tetromino();
        }
        if is_key_pressed(self.controls[4]) {
            self.drop_tetromino();
        }

        if time - self.last_gravity >= self.gravity_delay {
            self.direction.y = 1.0;
            self.last_gravity = time;
        }

        self.t1.pos.x += self.direction.x;
        if self.check_collision(self.t1, None) {
            self.t1.pos.x -= self.direction.x;
        }
        self.t1.pos.y += self.direction.y;
        if self.check_collision(self.t1, None) {
            self.t1.pos.y -= self.direction.y;
        }
    }

    fn game_loop(&mut self) {
        if self.check_collision(self.t1, Some(Vec2::new(0.0, 1.0))) {
            if self.direction.y == 1.0 {
                self.place_tetromino();
            }
        }
    }

    fn check_collision(&mut self, tetromino: Tetromino, offset: Option<Vec2>) -> bool {
        let offset = offset.unwrap_or(Vec2::new(0.0, 0.0));
        for y in 0..4 {
            for x in 0..4 {
                if tetromino.shape[y][x] {
                    let mut index = Vec2::new(x as f32, y as f32);
                    index += tetromino.pos + offset;

                    if index.x < 0.0
                        || index.x >= BOARD_WIDTH as f32
                        || index.y >= BOARD_HEIGHT as f32
                    {
                        return true;
                    }
                    if index.y > 0.0
                        && self.board[index.y as usize][index.x as usize] != BOARD_COLOR
                    {
                        return true;
                    }
                }
            }
        }
        false
    }

    fn update_phantom(&mut self) {
        self.p1 = self.t1.clone();
        for _ in 0..BOARD_HEIGHT + 1 {
            if !self.check_collision(self.p1, None) {
                self.p1.pos.y += 1.0;
            }
        }
        self.p1.pos.y -= 1.0;
    }

    fn drop_tetromino(&mut self) {
        self.t1 = self.p1;
        self.place_tetromino();
    }

    fn clear_lines(&mut self) {
        let mut cleared_lines = Vec::new();

        for y in 0..BOARD_HEIGHT {
            if self.board[y].iter().all(|&cell| cell != BOARD_COLOR) {
                cleared_lines.push(y);
            }
        }

        for &line in cleared_lines.iter() {
            self.board.remove(line);
            self.board.insert(0, vec![BOARD_COLOR; BOARD_WIDTH]);
        }
    }

    fn place_tetromino(&mut self) {
        for y in 0..4 {
            for x in 0..4 {
                if self.t1.shape[y][x] {
                    let board_x = (self.t1.pos.x as i32 + x as i32) as usize;
                    let board_y = (self.t1.pos.y as i32 + y as i32) as usize;
                    if board_y < BOARD_HEIGHT {
                        self.board[board_y][board_x] = self.t1.color;
                    }
                }
            }
        }
        self.clear_lines();
        self.refill_bag();
    }

    fn rotate_tetromino(&mut self) {
        let old = self.t1.clone();
        self.t1.rotate();

        for offset in [0, -1, 1, -2, 2] {
            self.t1.pos.x = old.pos.x + offset as f32;
            if !self.check_collision(self.t1, None) {
                return;
            }
        }

        self.t1 = old;
    }

    fn draw(&mut self) {
        for y in 0..BOARD_HEIGHT {
            for x in 0..BOARD_WIDTH {
                draw_block(
                    x as f32 + self.offset as f32,
                    y as f32,
                    self.board[y][x],
                    false,
                );
            }
        }

        self.p1.draw(self.offset, true, false);
        self.t1.draw(self.offset, false, false);
        self.t2.draw(self.offset, false, true);
    }
}
