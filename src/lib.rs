use wasm_bindgen::prelude::*;
use wee_alloc::WeeAlloc;

#[global_allocator]
static ALLOC: WeeAlloc = WeeAlloc::INIT;

#[wasm_bindgen(module = "/www/utils/rand.js")]
extern {
    fn random(max: usize) -> usize;
}

#[wasm_bindgen]
#[derive(Clone, Copy)]
pub enum GameStatus {
    Win,
    Lost,
    Playing
}

#[wasm_bindgen]
#[derive(PartialEq)]
pub enum Direction {
    Left,
    Right,
    Down,
    Up,
}

#[wasm_bindgen]
pub struct RewardCell(usize);

#[wasm_bindgen]
#[derive(Clone, PartialEq)]
pub struct SnakeCell(usize);

struct Snake {
    body: Vec<SnakeCell>,
    direction: Direction,
}

impl Snake {
    pub fn new(spawn_index: usize, len: usize) -> Self {
        let mut body = Vec::new();

        for i in 0..len {
            body.push(SnakeCell(spawn_index - i));
        }

        Snake {
            body,
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
    reward_cell: RewardCell,
    game_status: GameStatus,
}

#[wasm_bindgen]
impl World {
    fn gen_reward_cell(max: usize, snake: &Snake) -> RewardCell {
        loop {
            let reward_cell = random(max);

            if !snake.body.contains(&SnakeCell(reward_cell)) {
                return RewardCell(reward_cell);
            }
        }
    }

    pub fn new(width: usize, snake_idx: usize, snake_len: usize) -> Self {
        let size = width * width;
        let snake = Snake::new(snake_idx, snake_len);
        let reward_cell = Self::gen_reward_cell(size, &snake);
        let game_status = GameStatus::Playing;

        World {
            width,
            size,
            snake,
            reward_cell,
            game_status
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn game_status(&self) -> GameStatus {
        self.game_status
    }

    pub fn reward_cell(&self) -> *const RewardCell {
        &self.reward_cell
    }

    pub fn snake_length(&self) -> usize {
        self.snake.body.len()
    }

    // can not return a reference to JS because of the borrowing rules
    // pub fn snake_cells(&self) -> &Vec<SnakeCell> {
    //     &self.snake.body
    // }

    // *const is raw pointer
    // borrowing rules does not apply to it
    pub fn snake_cells(&self) -> *const SnakeCell {
        self.snake.body.as_ptr()
    }

    pub fn snake_head_idx(&self) -> usize {
        self.snake.body[0].0
    }

    pub fn change_snake_direction(&mut self, direction: Direction) {
        let next_cell = self.gen_next_snake_cell(&direction);

        if next_cell == self.snake.body[1] {
            return;
        }

        self.snake.direction = direction;
    }

    fn gen_next_snake_cell(&self, direction: &Direction) -> SnakeCell {
        let snake_idx = self.snake_head_idx();
        let row = snake_idx / self.width;

        return match direction {
            Direction::Left => SnakeCell((row * self.width) + (snake_idx - 1) % self.width),
            Direction::Right => SnakeCell((row * self.width) + (snake_idx + 1) % self.width),
            Direction::Down => SnakeCell((snake_idx + self.width) % self.size),
            Direction::Up => SnakeCell((snake_idx - self.width) % self.size),
        };
    }

    pub fn step(&mut self) {
        let snake_cells = &mut self.snake.body;
        let snake_len = snake_cells.len();

        for id in (1..snake_len).rev() {
            let previous_cell = &snake_cells[id - 1];
            snake_cells[id] = previous_cell.clone();
        }

        let direction = &self.snake.direction;
        self.snake.body[0] = self.gen_next_snake_cell(direction);

        if self.reward_cell.0 == self.snake_head_idx() {
            self.snake.body.push(SnakeCell(self.snake.body[1].0));

            if self.snake_length() == self.size {
                self.game_status = GameStatus::Win;
                self.reward_cell = RewardCell(10000);
            } else {
                self.reward_cell = Self::gen_reward_cell(self.size, &self.snake);
            }
        }

        if self.snake.body[1..self.snake_length()].contains(&SnakeCell(self.snake_head_idx())) {
            self.game_status = GameStatus::Lost;
        }
    }
}

// wasm-pack build --target web
