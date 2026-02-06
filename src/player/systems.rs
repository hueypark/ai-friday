use bevy::prelude::*;
use avian2d::prelude::*;
use crate::common::*;
use super::components::{CoyoteTimer, Grounded, JumpState, PlayerInput, COYOTE_TIME};

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
        CoyoteTimer::default(),
        JumpState::default(),
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
    mut query: Query<(&mut LinearVelocity, &Grounded, &mut CoyoteTimer, &mut JumpState), With<Player>>,
) {
    if !input.jump_pressed {
        return;
    }
    for (mut velocity, grounded, mut coyote, mut jump_state) in &mut query {
        if grounded.0 || coyote.timer > 0.0 {
            velocity.y = JUMP_IMPULSE;
            coyote.timer = 0.0;
            jump_state.is_jumping = true;
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
    mut death_events: MessageWriter<PlayerDiedEvent>,
) {
    for (mut transform, mut velocity) in &mut query {
        if transform.translation.y < -500.0 {
            death_events.write(PlayerDiedEvent);
            transform.translation = Vec3::new(0.0, 200.0, 0.0);
            velocity.0 = Vec2::ZERO;
        }
    }
}

pub fn update_coyote_timer(
    mut query: Query<(&mut CoyoteTimer, &Grounded), With<Player>>,
    time: Res<Time>,
) {
    for (mut coyote, grounded) in &mut query {
        if grounded.0 {
            coyote.timer = COYOTE_TIME;
        } else {
            coyote.timer = (coyote.timer - time.delta_secs()).max(0.0);
        }
    }
}

pub fn variable_jump_height(
    input: Res<PlayerInput>,
    mut query: Query<(&mut LinearVelocity, &mut JumpState, &Grounded), With<Player>>,
) {
    for (mut velocity, mut jump_state, grounded) in &mut query {
        if input.jump_released && jump_state.is_jumping && velocity.y > 0.0 {
            velocity.y *= 0.5;
        }
        if grounded.0 {
            jump_state.is_jumping = false;
        }
    }
}

pub fn apply_fall_gravity(
    mut query: Query<(&mut GravityScale, &LinearVelocity), With<Player>>,
) {
    const RISE_GRAVITY: f32 = 3.0;
    const FALL_GRAVITY: f32 = 10.0;

    for (mut gravity, velocity) in &mut query {
        gravity.0 = if velocity.y > 0.0 { RISE_GRAVITY } else { FALL_GRAVITY };
    }
}

pub fn despawn_player(mut commands: Commands, query: Query<Entity, With<Player>>) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}
