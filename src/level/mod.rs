pub mod camera;
pub mod components;
pub mod map;

use bevy::prelude::*;
use crate::common::GameState;

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), (map::spawn_level, map::spawn_collectibles))
            .add_systems(
                Update,
                camera::camera_follow_player.run_if(in_state(GameState::Playing)),
            )
            .add_systems(OnExit(GameState::Playing), map::despawn_level);
    }
}
