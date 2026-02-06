pub mod game_over;
pub mod hud;
pub mod menu;

use bevy::prelude::*;
use crate::common::*;

pub struct UiPlugin;

fn init_game_data(mut game_data: ResMut<GameData>) {
    game_data.score = 0;
    game_data.lives = INITIAL_LIVES;
}

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        // Menu
        app.add_systems(OnEnter(GameState::Menu), menu::spawn_menu)
            .add_systems(
                Update,
                menu::menu_button_interaction.run_if(in_state(GameState::Menu)),
            )
            .add_systems(OnExit(GameState::Menu), menu::despawn_menu);

        // Playing - init GameData and HUD
        app.add_systems(OnEnter(GameState::Playing), (init_game_data, hud::spawn_hud))
            .add_systems(
                Update,
                hud::update_hud.run_if(in_state(GameState::Playing)),
            )
            .add_systems(OnExit(GameState::Playing), hud::despawn_hud);

        // Game Over
        app.add_systems(OnEnter(GameState::GameOver), game_over::spawn_game_over)
            .add_systems(
                Update,
                game_over::game_over_button_interaction.run_if(in_state(GameState::GameOver)),
            )
            .add_systems(OnExit(GameState::GameOver), game_over::despawn_game_over);
    }
}
