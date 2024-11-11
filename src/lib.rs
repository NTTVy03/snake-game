use wasm_bindgen::prelude::*;
use wee_alloc::WeeAlloc;

#[global_allocator]
static ALLOC: WeeAlloc = WeeAlloc::INIT;

struct SnakeCell(usize);

struct Snake {
    body: Vec<SnakeCell>,
}

impl Snake {
    pub fn new(spawn_index: usize) -> Self {
        Snake {
            body: vec![SnakeCell(spawn_index)],
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
    pub fn new() -> Self {
        let width = 8;

        World {
            width,
            size: width * width,
            snake: Snake::new(10),
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn snake_head_idx(&self) -> usize {
        self.snake.body[0].0
    }

    pub fn update(&mut self) {
        let snake_head_idx = self.snake_head_idx();
        self.snake.body[0].0 = (snake_head_idx + 1) % self.size;
    }
}

// wasm-pack build --target web
