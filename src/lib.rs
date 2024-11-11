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
    width: usize,
    snake: Snake,
}

#[wasm_bindgen]
impl World {
    pub fn new() -> Self {
        World {
            width: 8,
            snake: Snake::new(10),
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn snake_head_idx(&self) -> usize {
        self.snake.body[0].0
    }
}

// wasm-pack build --target web
