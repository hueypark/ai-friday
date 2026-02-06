use bevy::prelude::*;
use crate::common::*;

#[derive(Component)]
pub struct GameOverRoot;

#[derive(Component)]
pub struct RestartButton;

#[derive(Component)]
pub struct MainMenuButton;

pub fn spawn_game_over(mut commands: Commands, game_data: Res<GameData>) {
    commands
        .spawn((
            GameOverRoot,
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                row_gap: Val::Px(30.0),
                ..default()
            },
        ))
        .with_children(|parent| {
            // Game Over title
            parent.spawn((
                Text::new("Game Over"),
                TextFont {
                    font_size: 72.0,
                    ..default()
                },
                TextColor(Color::srgb(1.0, 0.2, 0.2)),
            ));

            // Final score
            parent.spawn((
                Text::new(format!("Final Score: {}", game_data.score)),
                TextFont {
                    font_size: 36.0,
                    ..default()
                },
                TextColor(Color::WHITE),
            ));

            // Restart button
            parent
                .spawn((
                    RestartButton,
                    Button,
                    Node {
                        width: Val::Px(200.0),
                        height: Val::Px(60.0),
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.2, 0.7, 0.2)),
                ))
                .with_children(|btn| {
                    btn.spawn((
                        Text::new("Restart"),
                        TextFont {
                            font_size: 32.0,
                            ..default()
                        },
                        TextColor(Color::WHITE),
                    ));
                });

            // Main Menu button
            parent
                .spawn((
                    MainMenuButton,
                    Button,
                    Node {
                        width: Val::Px(200.0),
                        height: Val::Px(60.0),
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.4, 0.4, 0.4)),
                ))
                .with_children(|btn| {
                    btn.spawn((
                        Text::new("Main Menu"),
                        TextFont {
                            font_size: 32.0,
                            ..default()
                        },
                        TextColor(Color::WHITE),
                    ));
                });
        });
}

pub fn game_over_button_interaction(
    restart_query: Query<&Interaction, (Changed<Interaction>, With<RestartButton>)>,
    menu_query: Query<&Interaction, (Changed<Interaction>, With<MainMenuButton>)>,
    mut game_data: ResMut<GameData>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for interaction in &restart_query {
        if *interaction == Interaction::Pressed {
            game_data.score = 0;
            game_data.lives = INITIAL_LIVES;
            next_state.set(GameState::Playing);
        }
    }

    for interaction in &menu_query {
        if *interaction == Interaction::Pressed {
            game_data.score = 0;
            game_data.lives = INITIAL_LIVES;
            next_state.set(GameState::Menu);
        }
    }
}

pub fn despawn_game_over(mut commands: Commands, query: Query<Entity, With<GameOverRoot>>) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}
