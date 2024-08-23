use crate::player_controller::KeyMappings;
use macroquad::math::Vec2;

#[derive(PartialEq, Clone)]
pub enum SceneType {
    Fighting,
    MainMenu,
    Quitting,
}

#[derive(Default)]
pub struct Player {
    pub size: f32,
    pub position: Vec2,
    pub orientation: Vec2,
}

pub struct State {
    pub scene_type: SceneType,
    pub players: Vec<Player>,
    pub mappings: KeyMappings,
}
