use crate::models::{SceneType, State};
use crate::scenes::GameScene;
use macroquad::color::*;
use macroquad::prelude::*;
use macroquad::ui::root_ui;

pub struct MainMenu {}

impl GameScene for MainMenu {
    fn render(&self, _state: &State) {
        clear_background(WHITE);
    }

    fn handle_input(&self, state: &mut State) -> () {
        if is_key_pressed(KeyCode::Escape) {
            state.scene_type = SceneType::Quitting;
            return;
        }
    }

    fn render_ui(&self, state: &mut State) -> () {
        root_ui().label(None, "Wizard Arena");

        if root_ui().button(None, "Start Game") {
            state.scene_type = SceneType::Fighting
        }

        if root_ui().button(None, "Exit") {
            state.scene_type = SceneType::Quitting
        }
    }
}
