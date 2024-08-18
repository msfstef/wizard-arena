use macroquad::math::Vec2;

#[derive(PartialEq, Clone)]
pub enum SceneType {
    Fighting,
    MainMenu,
    Quitting,
}

pub struct Player {
    pub size: f32,
    pub position: Vec2,
}

pub struct State {
    pub scene_type: SceneType,
    pub players: Vec<Player>,
}
