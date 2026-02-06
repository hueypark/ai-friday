use bevy::prelude::*;

#[derive(Component)]
pub struct PatrolPath {
    pub start: Vec2,
    pub end: Vec2,
}

#[derive(Component)]
pub struct PatrolDirection(pub f32);

#[derive(Component)]
pub struct EnemySpeed(pub f32);

#[derive(Component)]
pub struct Stationary;
