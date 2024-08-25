mod models;
mod player_controller;
mod scenes;
use macroquad::prelude::*;
use models::{SceneType, State};
use scenes::*;

fn window_conf() -> Conf {
    Conf {
        window_title: "WizardArena".to_owned(),
        window_width: 1920,
        window_height: 1080,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut state = State::default();
    let mut current_scene_type: SceneType = state.scene_type.clone();
    let mut current_scene: Box<dyn GameScene> = Box::new(scenes::fighting::Fighting {});

    loop {
        if current_scene_type != state.scene_type {
            current_scene_type = state.scene_type.clone();
            current_scene = match current_scene_type {
                SceneType::Quitting => break,
                SceneType::MainMenu => Box::new(scenes::main_menu::MainMenu {}),
                SceneType::Fighting => Box::new(scenes::fighting::Fighting {}),
            };
        }

        current_scene.update(&mut state);
        current_scene.handle_input(&mut state);
        current_scene.render(&state);
        current_scene.render_ui(&mut state);

        next_frame().await
    }
}
