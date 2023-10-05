mod game;
mod render;

use game::{Game, GameState}; 
use macroquad::prelude::*;

#[macroquad::main("Rusty Snake")]
async fn main() {
    let mut game = Game::new();
    let mut last_update = get_time();
    loop {
        clear_background(BLACK);
        game.draw_borders();

        match game.game_state {
            GameState::Start => {
               game.start_game();
                if is_key_pressed(KeyCode::Space) {
                    game.game_state = GameState::Running;
                }
            }
            GameState::Running => {
                game.handle_input();
                game.draw();
                if get_time() - last_update > game.speed {
                    game.move_snake();
                    game.collision_with_food();
                    game.collision_with_border();
                    game.collision_with_self();
                    if !game.running {
                        game.game_state = GameState::GameOver;
                    }
                    last_update = get_time();
                }
            }
            GameState::GameOver => {
                game.game_over();
                if is_key_pressed(KeyCode::Space) {
                    game.restart();
                    game.game_state = GameState::Running;
                }
            }
        }

        next_frame().await;
    }
}
