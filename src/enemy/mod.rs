pub mod components;
pub mod systems;

use bevy::prelude::*;
use crate::common::GameState;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), systems::spawn_enemies)
            .add_systems(
                Update,
                (systems::enemy_patrol, systems::enemy_player_collision)
                    .run_if(in_state(GameState::Playing)),
            )
            .add_systems(OnExit(GameState::Playing), systems::despawn_enemies);
    }
}
