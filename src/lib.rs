use wasm_bindgen::prelude::*;
use wee_alloc::WeeAlloc;

#[global_allocator]
static ALLOC: WeeAlloc = WeeAlloc::INIT;

#[wasm_bindgen]
#[derive(PartialEq)]
pub enum Direction {
    Left,
    Right,
    Down,
    Up,
}

struct SnakeCell(usize);

struct Snake {
    body: Vec<SnakeCell>,
    direction: Direction,
}

impl Snake {
    pub fn new(spawn_index: usize) -> Self {
        Snake {
            body: vec![SnakeCell(spawn_index)],
            direction: Direction::Right,
        }
    }
}

// World is the square grid that our snake will move inside it
// Although world grid is a 2d matrix, we consider it like 1d array[0..width*width]
#[wasm_bindgen]
pub struct World {
    // number rows/columns in grid
    width: usize,

    // number cells in grid
    size: usize,

    snake: Snake,
}

#[wasm_bindgen]
impl World {
    pub fn new(width: usize, snake_idx: usize) -> Self {
        World {
            width,
            size: width * width,
            snake: Snake::new(snake_idx),
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn snake_head_idx(&self) -> usize {
        self.snake.body[0].0
    }

    fn set_snake_head(&mut self, idx: usize) {
        self.snake.body[0].0 = idx;
    }

    pub fn change_snake_direction(&mut self, direction: Direction) {
        self.snake.direction = direction;
    }

    pub fn update(&mut self) {
        let snake_idx = self.snake_head_idx();
        
        let (mut row, mut column) = self.idx_to_cell(snake_idx);

        match self.snake.direction {
            Direction::Left => {
                column = (column - 1 + self.width) % self.width;
            },
            Direction::Right => {
                column = (column + 1) % self.width;
            },
            Direction::Down => {
                row = (row + 1) % self.width;
            },
            Direction::Up => {
                row = (row - 1  + self.width) % self.width;
            },
        }

        let new_idx = self.cell_to_idx(row, column);
        self.set_snake_head(new_idx);
    }

    fn idx_to_cell(&self, idx: usize) -> (usize, usize) {
        let row = idx / self.width;
        let column = idx % self.width;

        (row, column)
    }

    fn cell_to_idx(&self, row: usize, column: usize) -> usize {
        let row = row % self.width;
        let column = column % self.width;

        row * self.width + column
    }
}

// wasm-pack build --target web
