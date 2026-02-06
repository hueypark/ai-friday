mod common;
mod enemy;
mod level;
mod player;
mod ui;

use bevy::prelude::*;
use common::*;
use player::components::PlayerInput;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "AI Friday Platformer".into(),
                resolution: (1280.0, 720.0).into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(avian2d::prelude::PhysicsPlugins::default())
        .init_state::<GameState>()
        .init_resource::<GameData>()
        .init_resource::<PlayerInput>()
        .add_event::<PlayerDiedEvent>()
        .add_event::<ScoreChangedEvent>()
        .add_plugins((
            player::PlayerPlugin,
            level::LevelPlugin,
            enemy::EnemyPlugin,
            ui::UiPlugin,
        ))
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}
