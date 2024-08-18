use macroquad::{prelude::*, ui::root_ui};

enum GameState {
    Fighting,
    MainMenu,
}

const BASE_PLAYER_SIZE: f32 = 15.0;

struct Player {
    size: f32,
    position: Vec2,
}

struct State {
    game_state: GameState,
    players: Vec<Player>,
}

#[macroquad::main("WizardArena")]
async fn main() {
    let mut state = State {
        game_state: GameState::MainMenu,
        players: vec![
            Player {
                size: 1.0,
                position: Vec2::new(0.1, 0.1),
            },
            Player {
                size: 1.0,
                position: Vec2::new(0.9, 0.9),
            },
        ],
    };

    loop {
        match state.game_state {
            GameState::MainMenu => {
                clear_background(WHITE);

                root_ui().label(None, "Wizard Arena");

                if root_ui().button(None, "Start Game") {
                    state.game_state = GameState::Fighting
                }

                if root_ui().button(None, "Exit") {
                    break;
                }
            }

            GameState::Fighting => {
                if is_key_pressed(KeyCode::Escape) {
                    state.game_state = GameState::MainMenu;
                    continue;
                }

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
        }

        next_frame().await
    }
}
