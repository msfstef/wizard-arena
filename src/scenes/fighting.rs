use crate::models::*;
use crate::player_controller::*;
use crate::scenes::GameScene;
use macroquad::color::*;
use macroquad::prelude::*;

const BASE_PLAYER_SIZE: f32 = 15.0;
const BASE_PROJECTILE_SIZE: f32 = 5.0;
const DT: f32 = 1.0;

pub struct Fighting {}

impl GameScene for Fighting {
    fn update(&self, state: &mut State) -> () {
        for player in state.players.iter_mut() {
            player.game_obj.update_clamped(DT);
        }

        let mut new_projectiles: Vec<Projectile> = vec![];

        'outer: for projectile in state.projectiles.iter_mut() {
            projectile.game_obj.update(DT);

            // TODO:should have smarter check for collisions
            // if it collides with a player, mark game as ended
            for player in state.players.iter() {
                // no friendly fire
                if projectile.owner_id.is_some_and(|id| id == player.id) {
                    continue;
                }

                if player.game_obj.collides_with(&projectile.game_obj) {
                    state.scene_type = SceneType::MainMenu;
                    continue 'outer;
                }
            }

            // if it is out of bounds, drop it
            if projectile.game_obj.out_of_bounds() {
                continue;
            }

            new_projectiles.push(projectile.clone());
        }

        state.projectiles = new_projectiles
    }

    fn handle_input(&self, state: &mut State) -> () {
        if is_key_pressed(KeyCode::Escape) {
            state.scene_type = SceneType::MainMenu;
            return;
        }
        let player1: &mut Player = state.players.get_mut(1).expect("Missing Player 1");

        let direction = get_movement_direction(&state.mappings);

        player1.game_obj.velocity = if direction.is_some() {
            Some(direction.unwrap() * 0.01)
        } else {
            None
        };

        let orientation = get_orientation(&state.mappings, &player1.game_obj.position);
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
            draw_rectangle_ex(
                player.game_obj.position.x * screen_width(),
                player.game_obj.position.y * screen_height(),
                player.game_obj.size.x * screen_width(),
                player.game_obj.size.x * screen_width(),
                DrawRectangleParams {
                    color: BLACK,
                    rotation: player.orientation.to_angle(),
                    offset: vec2(0.5, 0.5),
                },
            );
        }

        for projectile in state.projectiles.iter() {
            draw_circle(
                projectile.game_obj.position.x * screen_width(),
                projectile.game_obj.position.y * screen_height(),
                projectile.game_obj.size.x * screen_width(),
                YELLOW,
            );
        }
    }
    fn render_ui(&self, _state: &mut State) -> () {
        // no-op
    }
}
