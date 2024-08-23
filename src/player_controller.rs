use macroquad::prelude::*;

pub struct KeyMappings {
    up: KeyCode,
    down: KeyCode,
    left: KeyCode,
    right: KeyCode,
    primary_attack: MouseButton,
}

impl Default for KeyMappings {
    fn default() -> Self {
        Self {
            up: KeyCode::W,
            down: KeyCode::S,
            left: KeyCode::D,
            right: KeyCode::A,
            primary_attack: MouseButton::Right,
        }
    }
}

pub fn get_movement_direction(mappings: &KeyMappings) -> Option<Vec2> {
    let move_down = is_key_down(mappings.down);
    let move_up = is_key_down(mappings.up);
    let move_right = is_key_down(mappings.right);
    let move_left = is_key_down(mappings.left);

    vec2(
        if move_right { -1. } else { 0. } + if move_left { 1. } else { 0. },
        if move_up { -1. } else { 0. } + if move_down { 1. } else { 0. },
    )
    .try_normalize()
}

pub fn get_orientation(_mappings: &KeyMappings, src_pos: &Vec2) -> Option<Vec2> {
    let mouse_pos = mouse_position_local() / 2. + 0.5;
    (mouse_pos - src_pos.to_owned()).try_normalize()
}

pub fn should_attack_primary(mappings: &KeyMappings) -> bool {
    is_mouse_button_released(mappings.primary_attack)
}
