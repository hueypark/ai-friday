use bevy::prelude::*;

pub const COYOTE_TIME: f32 = 0.12;

#[derive(Component, Default)]
pub struct Grounded(pub bool);

#[derive(Component, Default)]
pub struct CoyoteTimer {
    pub timer: f32,
}

#[derive(Component, Default)]
pub struct JumpState {
    pub is_jumping: bool,
}

#[derive(Resource, Default)]
pub struct PlayerInput {
    pub move_direction: f32,
    pub jump_pressed: bool,
    pub jump_released: bool,
}
