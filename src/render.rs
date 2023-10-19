use crate::game::{Game, TILE_SIZE };
use macroquad::prelude::*;

const GAME_WIDTH: f32 = 400.0;
const GAME_HEIGHT: f32 = 400.0;

const START_X: f32 = 10.0;
const START_Y: f32 = 10.0;

impl Game {
    pub fn draw_borders(&self) {
        let border_thickness = 5.0;

        // Draw top border
        draw_rectangle(
            START_X - border_thickness,
            START_Y - border_thickness,
            GAME_WIDTH + 2.0 * border_thickness,
            border_thickness,
            WHITE,
        );
        // Draw bottom border
        draw_rectangle(
            START_X - border_thickness,
            START_Y + GAME_HEIGHT,
            GAME_WIDTH + 2.0 * border_thickness,
            border_thickness,
            WHITE,
        );
        // Draw left border
        draw_rectangle(
            START_X - border_thickness,
            START_Y - border_thickness,
            border_thickness,
            GAME_HEIGHT + 2.0 * border_thickness,
            WHITE,
        );
        // Draw right border
        draw_rectangle(
            START_X + GAME_WIDTH,
            START_Y - border_thickness,
            border_thickness,
            GAME_HEIGHT + 2.0 * border_thickness,
            WHITE,
        );
    }

    pub fn draw(&self) {
        for segment in &self.snake {
            draw_rectangle(segment.x, segment.y, TILE_SIZE, TILE_SIZE, WHITE);
        }

        draw_rectangle(self.food.x, self.food.y, TILE_SIZE, TILE_SIZE, GREEN);
    }

    pub fn game_over(&self) {
        let game_over_text = &format!("Game Over! Score: {}", self.score);
        let restart_text = "Press SPACE to restart the game";

        let game_over_width = measure_text(game_over_text, None, 34, 1.0).width;
        let restart_width = measure_text(restart_text, None, 22, 1.0).width;

        let center_x = START_X + GAME_WIDTH / 2.0;
        let center_y = START_Y + GAME_HEIGHT / 2.0;

        draw_text(
            game_over_text,
            center_x - game_over_width / 2.0,
            center_y - 20.0,
            34.0,
            RED,
        );
        draw_text(
            restart_text,
            center_x - restart_width / 2.0,
            center_y + 30.0,
            22.0,
            WHITE,
        );
    }
    pub fn start_game(&self) {
        let start_text = "Press SPACE to start";
        let start_width = measure_text(start_text, None, 34, 1.0).width;

        let center_x = START_X + GAME_WIDTH / 2.0;
        let center_y = START_Y + GAME_HEIGHT / 2.0;

        draw_text(
            start_text,
            center_x - start_width / 2.0,
            center_y - 20.0,
            34.0,
            WHITE,
        );
    }

    pub fn score_counter(&self) {
        let mut score_text = "Score: ".to_string();
        score_text = format!("{}{}", score_text, &self.score);
        draw_text(
            &score_text,
            300.0,
            30.0,
            18.0,
            WHITE,
        );
    }

    pub fn high_score(&self) {
        let mut score_text = "High score: ".to_string();
        score_text = format!("{}{}", score_text, &self.high_score);
        draw_text(
            &score_text,
            450.0,
            30.0,
            18.0,
            WHITE,
        );
    }

    pub fn n_games(&self) {
        let mut score_text = "N of Games: ".to_string();
        score_text = format!("{}{}", score_text, &self.n_games);
        draw_text(
            &score_text,
            580.0,
            30.0,
            18.0,
            WHITE,
        );
    }
}