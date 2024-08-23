use crate::player_controller::KeyMappings;
use macroquad::math::Vec2;

#[derive(PartialEq, Clone)]
pub enum SceneType {
    Fighting,
    MainMenu,
    Quitting,
}

pub type PlayerId = i8;

#[derive(Default)]
pub struct Player {
    pub id: PlayerId,
    pub size: f32,
    pub position: Vec2,
    pub orientation: Vec2,
}

impl Player {
    pub fn fire_projectile(&self, velocity: f32) -> Projectile {
        Projectile {
            projectile_type: ProjectileType::Regular,
            position: self.position.clone(),
            veolcity: self.orientation.clone() * velocity,
            owner_id: Some(self.id.clone()),
        }
    }
}

#[derive(Copy, Clone)]
pub enum ProjectileType {
    Regular,
}

#[derive(Copy, Clone)]
pub struct Projectile {
    pub projectile_type: ProjectileType,
    pub position: Vec2,
    pub veolcity: Vec2,
    pub owner_id: Option<PlayerId>,
}

pub struct State {
    pub scene_type: SceneType,
    pub players: Vec<Player>,
    pub projectiles: Vec<Projectile>,
    pub mappings: KeyMappings,
}

impl Default for State {
    fn default() -> Self {
        Self {
            mappings: Default::default(),
            scene_type: SceneType::Fighting,
            players: vec![
                Player {
                    id: 1,
                    size: 1.0,
                    position: Vec2::new(0.1, 0.1),
                    orientation: Default::default(),
                },
                Player {
                    id: 2,
                    size: 1.0,
                    position: Vec2::new(0.9, 0.9),
                    orientation: Default::default(),
                },
            ],
            projectiles: vec![],
        }
    }
}
