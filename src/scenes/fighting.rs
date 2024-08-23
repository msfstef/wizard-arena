use crate::models::*;
use crate::player_controller::*;
use crate::scenes::GameScene;
use macroquad::color::*;
use macroquad::prelude::*;

const BASE_PLAYER_SIZE: f32 = 15.0;
const BASE_PROJECTILE_SIZE: f32 = 5.0;

pub struct Fighting {}

impl GameScene for Fighting {
    fn update(&self, state: &mut State) -> () {
        for projectile in state.projectiles.iter_mut() {
            projectile.position += projectile.veolcity;
        }

        state.projectiles = state
            .projectiles
            .clone()
            .into_iter()
            .filter(|proj| -> bool {
                let pos = proj.position;
                !(pos.x > 1.0 || pos.x < 0.0 || pos.y > 1.0 || pos.y < 0.0)
            })
            .collect();
    }

    fn handle_input(&self, state: &mut State) -> () {
        if is_key_pressed(KeyCode::Escape) {
            state.scene_type = SceneType::MainMenu;
            return;
        }
        let player1: &mut Player = state.players.get_mut(1).expect("Missing Player 1");

        let direction = get_movement_direction(&state.mappings);
        if direction.is_some() {
            player1.position += direction.unwrap() * 0.01;
        }

        let orientation = get_orientation(&state.mappings, &player1.position);
        if orientation.is_some() {
            player1.orientation = orientation.unwrap()
        }

        if should_attack_primary(&state.mappings) {
            state.projectiles.push(player1.fire_projectile(0.01))
        }
    }

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

            draw_rectangle_ex(
                player.position.x * screen_width(),
                player.position.y * screen_height(),
                size,
                size,
                DrawRectangleParams {
                    color: BLACK,
                    rotation: player.orientation.to_angle(),
                    offset: vec2(0.5, 0.5),
                },
            );
        }

        for projectile in state.projectiles.iter() {
            let size = BASE_PROJECTILE_SIZE;

            draw_circle(
                projectile.position.x * screen_width(),
                projectile.position.y * screen_height(),
                size,
                YELLOW,
            );
        }
    }
    fn render_ui(&self, _state: &mut State) -> () {
        // no-op
    }
}
