use bevy::prelude::*;
use crate::common::*;

#[derive(Component)]
pub struct HudRoot;

#[derive(Component)]
pub struct ScoreText;

#[derive(Component)]
pub struct LivesText;

pub fn spawn_hud(mut commands: Commands) {
    commands
        .spawn((
            HudRoot,
            Node {
                width: Val::Percent(100.0),
                height: Val::Px(40.0),
                flex_direction: FlexDirection::Row,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::SpaceBetween,
                padding: UiRect::horizontal(Val::Px(20.0)),
                position_type: PositionType::Absolute,
                top: Val::Px(0.0),
                left: Val::Px(0.0),
                ..default()
            },
            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.5)),
        ))
        .with_children(|parent| {
            parent.spawn((
                ScoreText,
                Text::new("Score: 0"),
                TextFont {
                    font_size: 24.0,
                    ..default()
                },
                TextColor(Color::WHITE),
            ));

            parent.spawn((
                LivesText,
                Text::new("Lives: 3"),
                TextFont {
                    font_size: 24.0,
                    ..default()
                },
                TextColor(Color::WHITE),
            ));
        });
}

pub fn update_hud(
    mut score_query: Query<&mut Text, (With<ScoreText>, Without<LivesText>)>,
    mut lives_query: Query<&mut Text, (With<LivesText>, Without<ScoreText>)>,
    mut player_died_events: MessageReader<PlayerDiedEvent>,
    mut score_changed_events: MessageReader<ScoreChangedEvent>,
    mut game_data: ResMut<GameData>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    let mut changed = false;
    let mut game_over = false;

    for ScoreChangedEvent(new_score) in score_changed_events.read() {
        game_data.score = *new_score;
        changed = true;
    }

    for _event in player_died_events.read() {
        if game_data.lives > 0 {
            game_data.lives -= 1;
            changed = true;
        }
        if game_data.lives == 0 {
            game_over = true;
        }
    }

    if changed {
        for mut text in &mut score_query {
            **text = format!("Score: {}", game_data.score);
        }
        for mut text in &mut lives_query {
            **text = format!("Lives: {}", game_data.lives);
        }
    }

    if game_over {
        next_state.set(GameState::GameOver);
    }
}

pub fn despawn_hud(mut commands: Commands, query: Query<Entity, With<HudRoot>>) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}
