use bevy::prelude::*;
use avian2d::prelude::*;
use crate::common::*;
use super::components::{Grounded, PlayerInput};

pub fn spawn_player(mut commands: Commands) {
    commands.spawn((
        Player,
        Sprite {
            color: Color::srgb(0.2, 0.4, 0.9),
            custom_size: Some(Vec2::new(32.0, 48.0)),
            ..default()
        },
        Transform::from_xyz(0.0, 200.0, 0.0),
        RigidBody::Dynamic,
        Collider::rectangle(32.0, 48.0),
        LockedAxes::ROTATION_LOCKED,
        LinearVelocity::default(),
        Grounded(false),
        CollidingEntities::default(),
    ));
}

pub fn player_movement(
    input: Res<PlayerInput>,
    mut query: Query<&mut LinearVelocity, With<Player>>,
) {
    for mut velocity in &mut query {
        velocity.x = input.move_direction * PLAYER_SPEED;
    }
}

pub fn player_jump(
    input: Res<PlayerInput>,
    mut query: Query<(&mut LinearVelocity, &Grounded), With<Player>>,
) {
    if !input.jump_pressed {
        return;
    }
    for (mut velocity, grounded) in &mut query {
        if grounded.0 {
            velocity.y = JUMP_IMPULSE;
        }
    }
}

pub fn check_ground(
    mut query: Query<(&mut Grounded, &CollidingEntities, &Transform), With<Player>>,
    transform_query: Query<&Transform, (With<Platform>, Without<Player>)>,
) {
    for (mut grounded, colliding, player_transform) in &mut query {
        grounded.0 = colliding.iter().any(|&entity| {
            if let Ok(other_transform) = transform_query.get(entity) {
                other_transform.translation.y < player_transform.translation.y
            } else {
                false
            }
        });
    }
}

pub fn check_player_death(
    mut query: Query<(&mut Transform, &mut LinearVelocity), With<Player>>,
    mut death_events: EventWriter<PlayerDiedEvent>,
) {
    for (mut transform, mut velocity) in &mut query {
        if transform.translation.y < -500.0 {
            death_events.send(PlayerDiedEvent);
            transform.translation = Vec3::new(0.0, 200.0, 0.0);
            velocity.0 = Vec2::ZERO;
        }
    }
}

pub fn despawn_player(mut commands: Commands, query: Query<Entity, With<Player>>) {
    for entity in &query {
        commands.entity(entity).despawn_recursive();
    }
}
