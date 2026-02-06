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
        });
}

pub fn menu_button_interaction(
    query: Query<&Interaction, (Changed<Interaction>, With<PlayButton>)>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for interaction in &query {
        if *interaction == Interaction::Pressed {
            next_state.set(GameState::Playing);
        }
    }
}

pub fn despawn_menu(mut commands: Commands, query: Query<Entity, With<MenuRoot>>) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}
