mod models;
mod scenes;
use macroquad::prelude::*;
use models::{Player, SceneType, State};
use scenes::*;

#[macroquad::main("WizardArena")]
async fn main() {
    let scale_factor = screen_dpi_scale();
    request_new_screen_size(
        screen_width() * scale_factor,
        screen_height() * scale_factor,
    );
    let mut state = State {
        scene_type: SceneType::MainMenu,
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
    let mut current_scene_type: SceneType = state.scene_type.clone();
    let mut current_scene: Box<dyn GameScene> = Box::new(scenes::main_menu::MainMenu {});

    loop {
        if current_scene_type != state.scene_type {
            current_scene_type = state.scene_type.clone();
            current_scene = match current_scene_type {
                SceneType::Quitting => break,
                SceneType::MainMenu => Box::new(scenes::main_menu::MainMenu {}),
                SceneType::Fighting => Box::new(scenes::fighting::Fighting {}),
            };
        }

        current_scene.handle_input(&mut state);
        current_scene.render(&state);
        current_scene.render_ui(&mut state);

        next_frame().await
    }
}
