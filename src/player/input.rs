use bevy::prelude::*;
use super::components::PlayerInput;

pub fn read_input(keyboard: Res<ButtonInput<KeyCode>>, mut input: ResMut<PlayerInput>) {
    let mut direction = 0.0;
    if keyboard.pressed(KeyCode::KeyA) || keyboard.pressed(KeyCode::ArrowLeft) {
        direction -= 1.0;
    }
    if keyboard.pressed(KeyCode::KeyD) || keyboard.pressed(KeyCode::ArrowRight) {
        direction += 1.0;
    }
    input.move_direction = direction;
    input.jump_pressed = keyboard.just_pressed(KeyCode::Space);
    input.jump_released = keyboard.just_released(KeyCode::Space);
}
