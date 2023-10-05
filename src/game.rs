use ::rand::thread_rng;
use ::rand::Rng;
use macroquad::prelude::*;

pub const TILE_SIZE: f32 = 10.0;

pub enum GameState {
    Start,
    Running,
    GameOver,
}

#[derive(PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub struct Game {
    pub snake: Vec<Vec2>,
    pub food: Vec2,
    pub direction: Direction,
    pub direction_lock: bool,
    pub running: bool,
    pub speed: f64,
    pub score: i32,
    pub game_state: GameState,
}

impl Game {
    pub fn new() -> Self {
        let mut new_game = Self {
            snake: vec![
                vec2(200.0, 200.0),
                vec2(200.0, 210.0),
                vec2(200.0, 220.0),
                vec2(200.0, 230.0),
            ],
            food: Vec2::ZERO, // Placeholder, you'll generate a position for it
            direction: Direction::Up,
            direction_lock: false,
            running: true,
            speed: 0.1,
            score: 0,
            game_state: GameState::Start,
        };
        new_game.food = new_game.new_food();
        new_game
    }

    pub fn move_snake(&mut self) {
        let head = *self.snake.first().unwrap();

        let new_head = match self.direction {
            Direction::Up => vec2(head.x, head.y - TILE_SIZE),
            Direction::Down => vec2(head.x, head.y + TILE_SIZE),
            Direction::Left => vec2(head.x - TILE_SIZE, head.y),
            Direction::Right => vec2(head.x + TILE_SIZE, head.y),
        };

        self.snake.insert(0, new_head);
        self.snake.pop();
        self.direction_lock = false;
    }

    pub fn handle_input(&mut self) {
        if self.direction_lock == false {
            if is_key_pressed(KeyCode::Up) && self.direction != Direction::Down {
                self.direction = Direction::Up;
                self.direction_lock = true;
            } else if is_key_pressed(KeyCode::Down) && self.direction != Direction::Up {
                self.direction = Direction::Down;
                self.direction_lock = true;
            } else if is_key_pressed(KeyCode::Left) && self.direction != Direction::Right {
                self.direction = Direction::Left;
                self.direction_lock = true;
            } else if is_key_pressed(KeyCode::Right) && self.direction != Direction::Left {
                self.direction = Direction::Right;
                self.direction_lock = true;
            }
        }
    }
    
    pub fn new_food(&self) -> Vec2 {
        loop {
            let mut rng = thread_rng();
            let x = (rng.gen_range(2..=40) as f32) * TILE_SIZE;
            let y = (rng.gen_range(2..=40) as f32) * TILE_SIZE;
            let food_location = vec2(x, y);

            if !self.snake.contains(&food_location) {
                return food_location;
            }
        }
    }

    pub fn collision_with_self(&mut self) {
        for segment in &self.snake[1..] {
            if *segment == self.snake[0] {
                self.running = false;
                break;
            }
        }
    }

    pub fn collision_with_border(&mut self) {
        let head = self.snake[0];
        if head.x < 10.0 || head.x >= 410.0 || head.y < 10.0 || head.y >= 410.0 {
            self.running = false;
        }
    }

    pub fn collision_with_food(&mut self) {
        if self.snake[0] == self.food {
            self.score += 1;
            let last_element = *self.snake.last().unwrap();
            self.snake.push(last_element);
            self.speed = f64::max(0.03, self.speed * 0.95);
            self.food = self.new_food();
        }
    }

    pub fn restart(&mut self) {
        *self = Game::new();
    }
}
