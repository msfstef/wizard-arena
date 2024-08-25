use crate::player_controller::KeyMappings;
use macroquad::{
    math::{vec2, Vec2},
    window::{screen_height, screen_width},
};

#[derive(PartialEq, Clone)]
pub enum SceneType {
    Fighting,
    MainMenu,
    Quitting,
}

#[derive(Default)]
pub struct Bounds {
    pub lower: Vec2,
    pub upper: Vec2,
}

impl Bounds {
    pub fn screen() -> Self {
        Self {
            lower: vec2(0., 0.),
            upper: vec2(screen_width(), screen_height()),
        }
    }
}

#[derive(Default, Clone)]
pub struct GameObject {
    pub position: Vec2,
    pub velocity: Option<Vec2>,
    pub size: Vec2,
}

impl GameObject {
    pub fn out_of_bounds(&self, bounds: &Bounds) -> bool {
        self.position.x - self.size.x / 2. < bounds.lower.x
            || self.position.x + self.size.x / 2. > bounds.upper.x
            || self.position.y - self.size.y / 2. < bounds.lower.y
            || self.position.y + self.size.y / 2. > bounds.upper.y
    }

    pub fn collides_with(&self, other: &Self) -> bool {
        self.position.x - self.size.x / 2. < other.position.x + other.size.x / 2.
            && self.position.x + self.size.x / 2. > other.position.x - other.size.x / 2.
            && self.position.y - self.size.y / 2. < other.position.y + other.size.y / 2.
            && self.position.y + self.size.y / 2. > other.position.y - other.size.y / 2.
    }

    pub fn update(&mut self, dt: f32) -> () {
        if self.velocity.is_some() {
            self.position += self.velocity.unwrap() * dt;
        }
    }

    pub fn update_clamped(&mut self, dt: f32, bounds: &Bounds) -> () {
        self.update(dt);
        if self.out_of_bounds(bounds) {
            self.position = self
                .position
                .clamp(bounds.lower + self.size / 2., bounds.upper - self.size / 2.);
            self.velocity = None;
        }
    }
}

pub type PlayerId = i8;

#[derive(Default)]
pub struct Player {
    pub id: PlayerId,
    pub game_obj: GameObject,
    pub orientation: Vec2,
}

impl Player {
    pub fn fire_projectile(&self, velocity: f32) -> Projectile {
        Projectile {
            projectile_type: ProjectileType::Regular,
            game_obj: GameObject {
                position: self.game_obj.position.clone(),
                velocity: Some(self.orientation.clone() * velocity),
                size: vec2(4., 4.),
            },
            owner_id: Some(self.id.clone()),
        }
    }
}

#[derive(Clone)]
pub enum ProjectileType {
    Regular,
}

#[derive(Clone)]
pub struct Projectile {
    pub projectile_type: ProjectileType,
    pub game_obj: GameObject,
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
                    game_obj: GameObject {
                        position: Bounds::screen().upper * 0.1,
                        size: vec2(10., 10.),
                        ..Default::default()
                    },
                    orientation: Default::default(),
                },
                Player {
                    id: 2,
                    game_obj: GameObject {
                        position: Bounds::screen().upper * 0.9,
                        size: vec2(10., 10.),
                        ..Default::default()
                    },
                    orientation: Default::default(),
                },
            ],
            projectiles: vec![],
        }
    }
}
