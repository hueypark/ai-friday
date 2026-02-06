use bevy::prelude::*;
use crate::common::*;

#[derive(Component)]
pub struct HudRoot;

#[derive(Component)]
pub struct ScoreText;

#[derive(Component)]
pub struct LivesText;

#[derive(Component)]
pub struct ScoreFlash {
    pub timer: f32,
}

#[derive(Component)]
pub struct LivesFlash {
    pub timer: f32,
}

pub fn spawn_hud(mut commands: Commands) {
    commands
        .spawn((
            HudRoot,
            Node {
                width: Val::Percent(100.0),
                height: Val::Px(44.0),
                flex_direction: FlexDirection::Row,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::SpaceBetween,
                padding: UiRect::horizontal(Val::Px(20.0)),
                position_type: PositionType::Absolute,
                top: Val::Px(0.0),
                left: Val::Px(0.0),
                ..default()
            },
            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.7)),
        ))
        .with_children(|parent| {
            // Score row
            parent
                .spawn(Node {
                    flex_direction: FlexDirection::Row,
                    align_items: AlignItems::Center,
                    column_gap: Val::Px(8.0),
                    ..default()
                })
                .with_children(|row| {
                    // Gold square icon
                    row.spawn((
                        Node {
                            width: Val::Px(16.0),
                            height: Val::Px(16.0),
                            ..default()
                        },
                        BackgroundColor(Color::srgb(1.0, 0.84, 0.0)),
                    ));

                    row.spawn((
                        ScoreText,
                        ScoreFlash { timer: 0.0 },
                        Text::new("Score: 0"),
                        TextFont {
                            font_size: 26.0,
                            ..default()
                        },
                        TextColor(Color::WHITE),
                    ));
                });

            // Lives row
            parent
                .spawn(Node {
                    flex_direction: FlexDirection::Row,
                    align_items: AlignItems::Center,
                    column_gap: Val::Px(8.0),
                    ..default()
                })
                .with_children(|row| {
                    // Red square icon
                    row.spawn((
                        Node {
                            width: Val::Px(16.0),
                            height: Val::Px(16.0),
                            ..default()
                        },
                        BackgroundColor(Color::srgb(1.0, 0.2, 0.2)),
                    ));

                    row.spawn((
                        LivesText,
                        LivesFlash { timer: 0.0 },
                        Text::new("Lives: 3"),
                        TextFont {
                            font_size: 26.0,
                            ..default()
                        },
                        TextColor(Color::WHITE),
                    ));
                });
        });
}

pub fn update_hud(
    mut score_query: Query<(&mut Text, &mut ScoreFlash), (With<ScoreText>, Without<LivesText>)>,
    mut lives_query: Query<(&mut Text, &mut LivesFlash), (With<LivesText>, Without<ScoreText>)>,
    mut player_died_events: MessageReader<PlayerDiedEvent>,
    mut score_changed_events: MessageReader<ScoreChangedEvent>,
    mut game_data: ResMut<GameData>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    let mut score_changed = false;
    let mut lives_changed = false;
    let mut game_over = false;

    for ScoreChangedEvent(new_score) in score_changed_events.read() {
        game_data.score = *new_score;
        score_changed = true;
    }

    for _event in player_died_events.read() {
        if game_data.lives > 0 {
            game_data.lives -= 1;
            lives_changed = true;
        }
        if game_data.lives == 0 {
            game_over = true;
        }
    }

    if score_changed {
        for (mut text, mut flash) in &mut score_query {
            **text = format!("Score: {}", game_data.score);
            flash.timer = 0.4;
        }
    }

    if lives_changed {
        for (mut text, mut flash) in &mut lives_query {
            **text = format!("Lives: {}", game_data.lives);
            flash.timer = 0.4;
        }
    }

    if game_over {
        next_state.set(GameState::GameOver);
    }
}

pub fn update_hud_flash(
    time: Res<Time>,
    mut score_query: Query<(&mut ScoreFlash, &mut TextColor), (With<ScoreText>, Without<LivesText>)>,
    mut lives_query: Query<(&mut LivesFlash, &mut TextColor), (With<LivesText>, Without<ScoreText>)>,
) {
    for (mut flash, mut color) in &mut score_query {
        if flash.timer > 0.0 {
            flash.timer = (flash.timer - time.delta_secs()).max(0.0);
            let t = flash.timer / 0.4;
            // Lerp from white (t=0) to yellow (t=1)
            color.0 = Color::srgb(1.0, 1.0, 1.0 - t);
        }
    }
    for (mut flash, mut color) in &mut lives_query {
        if flash.timer > 0.0 {
            flash.timer = (flash.timer - time.delta_secs()).max(0.0);
            let t = flash.timer / 0.4;
            // Lerp from white (t=0) to red (t=1)
            color.0 = Color::srgb(1.0, 1.0 - t * 0.8, 1.0 - t * 0.8);
        }
    }
}

pub fn despawn_hud(mut commands: Commands, query: Query<Entity, With<HudRoot>>) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}
