use bevy::prelude::*;
use crate::common::Player;

const CAMERA_LERP_FACTOR: f32 = 0.1;
const CAMERA_MIN_X: f32 = -800.0;
const CAMERA_MAX_X: f32 = 800.0;

pub fn camera_follow_player(
    player_query: Query<&Transform, With<Player>>,
    mut camera_query: Query<&mut Transform, (With<Camera2d>, Without<Player>)>,
) {
    let Ok(player_transform) = player_query.get_single() else {
        return;
    };
    let Ok(mut camera_transform) = camera_query.get_single_mut() else {
        return;
    };

    let target_x = player_transform.translation.x.clamp(CAMERA_MIN_X, CAMERA_MAX_X);
    camera_transform.translation.x +=
        (target_x - camera_transform.translation.x) * CAMERA_LERP_FACTOR;
}
