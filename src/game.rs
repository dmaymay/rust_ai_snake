use crate::agent::Action;
use ::rand::thread_rng;
use ::rand::Rng;
use macroquad::prelude::*;
use std::cmp::Ordering;

pub const TILE_SIZE: f32 = 10.0;

pub enum GameStatus {
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
    pub high_score: i32,
    pub game_status: GameStatus,
    pub time_starving: i32,
    pub just_ate: bool,
    pub n_games: i32,
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
            speed: 0.001,
            score: 0,
            high_score: 0,
            n_games: 0,
            game_status: GameStatus::Start,
            time_starving: 0,
            just_ate: false,
        };
        new_game.food = new_game.new_food();
        new_game
    }

    pub fn move_snake(&mut self) {
        let (x, y) = (self.snake[0].x, self.snake[0].y);

        let new_head = match self.direction {
            Direction::Up => vec2(x, y - TILE_SIZE),
            Direction::Down => vec2(x, y + TILE_SIZE),
            Direction::Left => vec2(x - TILE_SIZE, y),
            Direction::Right => vec2(x + TILE_SIZE, y),
        };

        self.snake.insert(0, new_head);
        self.snake.pop();
        self.direction_lock = false;
    }

    pub fn handle_input(&mut self) -> Action {
        if self.direction_lock == false {
            if is_key_pressed(KeyCode::Up) && self.direction != Direction::Down {
                self.direction = Direction::Up;
                self.direction_lock = true;
                Action::Up
            } else if is_key_pressed(KeyCode::Down) && self.direction != Direction::Up {
                self.direction = Direction::Down;
                self.direction_lock = true;
                Action::Down
            } else if is_key_pressed(KeyCode::Left) && self.direction != Direction::Right {
                self.direction = Direction::Left;
                self.direction_lock = true;
                Action::Left
            } else if is_key_pressed(KeyCode::Right) && self.direction != Direction::Left {
                self.direction = Direction::Right;
                self.direction_lock = true;
                Action::Right
            } else {
                match self.direction {
                    Direction::Up => Action::Up,
                    Direction::Down => Action::Down,
                    Direction::Left => Action::Left,
                    Direction::Right => Action::Right,
                }
            }
        } else {
            // Assuming you want to return the current action when direction_lock is true.
            match self.direction {
                Direction::Up => Action::Up,
                Direction::Down => Action::Down,
                Direction::Left => Action::Left,
                Direction::Right => Action::Right,
            }
        }
    }

    pub fn handle_action(&mut self, action: &Action) {
        let current_direction = &self.direction;

        match action {
            Action::Up if current_direction != &Direction::Down => self.direction = Direction::Up,
            Action::Down if current_direction != &Direction::Up => self.direction = Direction::Down,
            Action::Left if current_direction != &Direction::Right => {
                self.direction = Direction::Left
            }
            Action::Right if current_direction != &Direction::Left => {
                self.direction = Direction::Right
            }
            _ => {} // Do nothing if the direction is opposite or any other reason
        };
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

    pub fn collision_with_food(&mut self) -> bool {
        if self.snake[0] == self.food {
            self.score += 1;
            let last_element = *self.snake.last().unwrap();
            self.snake.push(last_element);
            self.speed = f64::max(0.03, self.speed * 0.95);
            self.time_starving = 0;
            self.just_ate = true;
            return true;
        }
        false
    }

    pub fn restart(&mut self) {
        if self.score > self.high_score {
            self.high_score = self.score;
        }
        let current_high_score = self.high_score; // Store the current high score before reinitializing
        let ngames = self.n_games;
        *self = Game::new();
        self.high_score = current_high_score; // Set the high score in the new instance
        self.n_games = ngames + 1;
    }

    pub fn get_game_state(&mut self) -> [i32; 12] {
        let direction_state: [i32; 4] = match self.direction {
            Direction::Up => [1, 0, 0, 0],
            Direction::Down => [0, 1, 0, 0],
            Direction::Left => [0, 0, 1, 0],
            Direction::Right => [0, 0, 0, 1],
        };
        let head_x = self.snake[0].x as i32;
        let head_y = self.snake[0].y as i32;
        let food_x = self.food.x as i32;
        let food_y = self.food.y as i32;
        let mut food_direction: [i32; 4] = [0, 0, 0, 0];
        if self.just_ate {
            self.just_ate = false;
        } else {
            let y_direction = match head_y.cmp(&food_y) {
                Ordering::Greater => [1, 0],
                Ordering::Less => [0, 1],
                _ => [0, 0],
            };

            let x_direction = match head_x.cmp(&food_x) {
                Ordering::Greater => [1, 0],
                Ordering::Less => [0, 1],
                _ => [0, 0],
            };

            food_direction = [
                y_direction[0],
                y_direction[1],
                x_direction[0],
                x_direction[1],
            ];
        }
        let mut danger = [0, 0, 0, 0];
        if self.snake.len() > 4 {
            println!("longer than 4");
            for segment in &self.snake[1..] {
                let x_seg = segment.x as i32;
                let y_seg = segment.y as i32;

                if (head_x - x_seg).abs() + (head_y - y_seg).abs() == 10 {
                    if ((head_y - y_seg == 10) && (self.direction != Direction::Down))
                        || (head_y - 10 == 0)
                    {
                        danger[0] = 1;
                    }
                    if ((head_y - y_seg == -10) && (self.direction != Direction::Up))
                        || (head_y + 10 == 410)
                    {
                        danger[1] = 1;
                    }
                    if ((head_x - x_seg == 10) && (self.direction != Direction::Right))
                        || (head_x - 10 == 0)
                    {
                        danger[2] = 1;
                    }
                    if ((head_x - x_seg == -10) && (self.direction != Direction::Left))
                        || (head_x + 10 == 410)
                    {
                        danger[3] = 1;
                    }
                }
            }
        }
        let game_state: [i32; 12] = direction_state
            .iter()
            .chain(&food_direction)
            .chain(&danger)
            .cloned()
            .collect::<Vec<_>>()
            .try_into()
            .expect("Length mismatch");
        //println!("{:?}", game_state);
        game_state
    }
}
