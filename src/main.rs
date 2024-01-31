mod agent;
mod game;
mod nn;
mod render;

use agent::Agent;
use game::{Game, GameStatus};
use macroquad::prelude::*;

#[macroquad::main("Rusty Snake")]
async fn main() {
    let mut ai_controlled = true;
    let mut game = Game::new();
    let mut agent = Agent::new();
    let mut last_update = get_time();
    let mut game_over_time: Option<f64> = None;

    loop {
        clear_background(BLACK);
        game.draw_borders();

        match game.game_status {
            GameStatus::Start => {
                game.start_game();
                if is_key_pressed(KeyCode::Space) {
                    game.game_status = GameStatus::Running;
                }
            }
            GameStatus::Running => {
                if is_key_pressed(KeyCode::T) {
                    ai_controlled = !ai_controlled; // Toggle the AI control
                }
                game.handle_input();
                game.draw();
                game.score_counter();
                game.high_score();
                game.n_games();
                if get_time() - last_update > game.speed {
                    let current_state = game.get_game_state();
                    let mut action = agent.select_action(&current_state);
                    if ai_controlled {
                        game.handle_action(&action);
                    } else {
                        action = game.handle_input();
                    }
                    game.move_snake();
                    game.collision_with_border();
                    game.collision_with_self();

                    let mut reward: f64 = 0.0;
                    if !game.running {
                        reward = -10.0;
                        game.game_status = GameStatus::GameOver;
                    } else if game.collision_with_food() {
                        reward = 5.0;
                        game.food = game.new_food();
                    } else if game.time_starving > 10000 + (game.snake.len() as i32) * 10 {
                        let calculated_value =
                            (game.time_starving as f64 - 10000.0 - game.snake.len() as f64 * 10.0)
                                * 0.01;
                        reward = (-calculated_value as f64).max(-0.5);
                    }
                    //new state
                    println!("Reward: {}", reward);
                    let done = !game.running;
                    let new_state = game.get_game_state();
                    // train the agent
                    println!("Action Taken :{}", action);
                    agent.train(&current_state, action.clone(), reward, &new_state, done);
                    agent.remember(&current_state, action, reward, &new_state, done);
                    last_update = get_time();
                }
            }
            GameStatus::GameOver => {
                game.game_over();
                agent.train_long_memory();
                if game_over_time.is_none() {
                    game_over_time = Some(get_time());
                }

                let current_time = get_time();
                if let Some(start_time) = game_over_time {
                    if current_time - start_time > 0.2 {
                        // .2 seconds delay
                        game.restart();
                        game.game_status = GameStatus::Running;
                        game_over_time = None;
                    }
                }
            }
        }

        next_frame().await;
    }
}
