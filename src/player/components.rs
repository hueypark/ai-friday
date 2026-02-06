use bevy::prelude::*;

#[derive(Component, Default)]
pub struct Grounded(pub bool);

#[derive(Resource, Default)]
pub struct PlayerInput {
    pub move_direction: f32,
    pub jump_pressed: bool,
}
