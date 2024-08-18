use crate::models::*;
use crate::scenes::GameScene;
use macroquad::color::*;
use macroquad::prelude::*;

const BASE_PLAYER_SIZE: f32 = 15.0;

pub struct Fighting {}

impl GameScene for Fighting {
    fn render(&self, state: &State) {
        clear_background(RED);

        let buffer_pct = 0.1;
        let buffer_x = screen_width() * buffer_pct;
        let buffer_y = screen_height() * buffer_pct;

        draw_rectangle(
            buffer_x,
            buffer_y,
            screen_width() - 2.0 * buffer_x,
            screen_height() - 2.0 * buffer_y,
            GRAY,
        );

        for player in state.players.iter() {
            let size = BASE_PLAYER_SIZE * player.size;
            draw_rectangle(
                player.position.x * screen_width() - size / 2.0,
                player.position.y * screen_height() - size / 2.0,
                size,
                size,
                BLACK,
            )
        }
    }

    fn handle_input(&self, state: &mut State) -> () {
        let player1: &mut Player = state.players.get_mut(1).expect("Missing Player 1");
        if is_key_down(KeyCode::W) {
            player1.position += vec2(0.0, -0.01);
        } else if is_key_down(KeyCode::S) {
            player1.position += vec2(0.0, 0.01);
        }
        if is_key_down(KeyCode::D) {
            player1.position += vec2(0.01, 0.0);
        } else if is_key_down(KeyCode::A) {
            player1.position += vec2(-0.01, 0.0);
        }
    }

    fn render_ui(&self, _state: &mut State) -> () {
        // no-op
    }
}
