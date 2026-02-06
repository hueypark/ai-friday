use bevy::prelude::*;
use crate::common::*;

#[derive(Component)]
pub struct MenuRoot;

#[derive(Component)]
pub struct PlayButton;

pub fn spawn_menu(mut commands: Commands) {
    commands
        .spawn((
            MenuRoot,
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                row_gap: Val::Px(40.0),
                ..default()
            },
        ))
        .with_children(|parent| {
            // Title
            parent.spawn((
                Text::new("AI Friday Platformer"),
                TextFont {
                    font_size: 64.0,
                    ..default()
                },
                TextColor(Color::WHITE),
            ));

            // Subtitle
            parent.spawn((
                Text::new("A 2D Platformer Adventure"),
                TextFont {
                    font_size: 28.0,
                    ..default()
                },
                TextColor(Color::srgb(0.7, 0.7, 0.7)),
            ));

            // Play button
            parent
                .spawn((
                    PlayButton,
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
                        Text::new("Play"),
                        TextFont {
                            font_size: 32.0,
                            ..default()
                        },
                        TextColor(Color::WHITE),
                    ));
                });

            // Controls hint
            parent.spawn((
                Text::new("Move: A/D/Arrows | Jump: Space | Stomp enemies!"),
                TextFont {
                    font_size: 18.0,
                    ..default()
                },
                TextColor(Color::srgb(0.6, 0.6, 0.6)),
            ));
        });
}

pub fn menu_button_interaction(
    mut query: Query<(&Interaction, &mut BackgroundColor), (Changed<Interaction>, With<PlayButton>)>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for (interaction, mut bg_color) in &mut query {
        match interaction {
            Interaction::Pressed => {
                next_state.set(GameState::Playing);
            }
            Interaction::Hovered => {
                *bg_color = BackgroundColor(Color::srgb(0.3, 0.85, 0.3));
            }
            Interaction::None => {
                *bg_color = BackgroundColor(Color::srgb(0.2, 0.7, 0.2));
            }
        }
    }
}

pub fn despawn_menu(mut commands: Commands, query: Query<Entity, With<MenuRoot>>) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}
