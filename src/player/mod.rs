pub mod components;
pub mod input;
pub mod systems;

use bevy::prelude::*;
use crate::common::GameState;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), systems::spawn_player)
            .add_systems(
                Update,
                (
                    input::read_input,
                    systems::check_ground,
                    systems::update_coyote_timer,
                    systems::player_movement,
                    systems::player_jump,
                    systems::variable_jump_height,
                    systems::check_player_death,
                )
                    .chain()
                    .run_if(in_state(GameState::Playing)),
            )
            .add_systems(OnExit(GameState::Playing), systems::despawn_player);
    }
}
