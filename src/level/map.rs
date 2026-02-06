use bevy::prelude::*;
use avian2d::prelude::*;
use crate::common::*;
use crate::level::components::CoinValue;

pub fn spawn_level(mut commands: Commands) {
    // (x, y, width, height)
    let platforms: &[(f32, f32, f32, f32)] = &[
        // Ground layer - 3 segments with gaps
        (-600.0, -200.0, 500.0, 32.0),   // left ground
        (0.0, -200.0, 400.0, 32.0),       // center ground
        (550.0, -200.0, 500.0, 32.0),     // right ground

        // Tier 1 (y: -120 to -40) - 6 platforms
        (-450.0, -120.0, 128.0, 16.0),
        (-200.0, -80.0, 160.0, 16.0),
        (0.0, -40.0, 128.0, 16.0),
        (200.0, -100.0, 128.0, 16.0),
        (400.0, -60.0, 160.0, 16.0),
        (600.0, -120.0, 128.0, 16.0),

        // Tier 2 (y: 20 to 120) - 6 platforms
        (-500.0, 40.0, 128.0, 16.0),
        (-250.0, 80.0, 160.0, 16.0),
        (50.0, 20.0, 128.0, 16.0),
        (250.0, 60.0, 128.0, 16.0),
        (450.0, 100.0, 160.0, 16.0),
        (650.0, 40.0, 128.0, 16.0),

        // Tier 3 (y: 180 to 300) - 5 platforms
        (-350.0, 200.0, 128.0, 16.0),
        (-100.0, 240.0, 160.0, 16.0),
        (150.0, 180.0, 128.0, 16.0),
        (350.0, 260.0, 128.0, 16.0),
        (550.0, 300.0, 128.0, 16.0),

        // Summit - 1 wide platform
        (100.0, 360.0, 256.0, 16.0),
    ];

    for &(x, y, w, h) in platforms {
        commands.spawn((
            Platform,
            Sprite {
                color: Color::srgb(0.2, 0.7, 0.2),
                custom_size: Some(Vec2::new(w, h)),
                ..default()
            },
            Transform::from_xyz(x, y, 0.0),
            RigidBody::Static,
            Collider::rectangle(w, h),
        ));
    }
}

pub fn spawn_collectibles(mut commands: Commands) {
    // (x, y, coin_value, color)
    let coins: &[(f32, f32, u32, Color)] = &[
        // Tier 1 coins (10pts, gold)
        (-450.0, -95.0, 10, Color::srgb(1.0, 0.84, 0.0)),
        (-200.0, -55.0, 10, Color::srgb(1.0, 0.84, 0.0)),
        (200.0, -75.0, 10, Color::srgb(1.0, 0.84, 0.0)),
        (400.0, -35.0, 10, Color::srgb(1.0, 0.84, 0.0)),

        // Tier 2 coins (20pts, orange)
        (-500.0, 65.0, 20, Color::srgb(1.0, 0.6, 0.0)),
        (-250.0, 105.0, 20, Color::srgb(1.0, 0.6, 0.0)),
        (250.0, 85.0, 20, Color::srgb(1.0, 0.6, 0.0)),
        (450.0, 125.0, 20, Color::srgb(1.0, 0.6, 0.0)),

        // Tier 3 coins (30pts, darker orange)
        (-350.0, 225.0, 30, Color::srgb(1.0, 0.5, 0.0)),
        (-100.0, 265.0, 30, Color::srgb(1.0, 0.5, 0.0)),
        (150.0, 205.0, 30, Color::srgb(1.0, 0.5, 0.0)),
        (350.0, 285.0, 30, Color::srgb(1.0, 0.5, 0.0)),

        // Summit coins (50pts, cyan)
        (50.0, 385.0, 50, Color::srgb(0.0, 1.0, 1.0)),
        (100.0, 385.0, 50, Color::srgb(0.0, 1.0, 1.0)),
        (150.0, 385.0, 50, Color::srgb(0.0, 1.0, 1.0)),

        // Hidden coin in ground gap (10pts, gold)
        (-300.0, -180.0, 10, Color::srgb(1.0, 0.84, 0.0)),
    ];

    for &(x, y, value, color) in coins {
        commands.spawn((
            Collectible,
            CoinValue(value),
            Sprite {
                color,
                custom_size: Some(Vec2::new(16.0, 16.0)),
                ..default()
            },
            Transform::from_xyz(x, y, 0.0),
            Collider::rectangle(16.0, 16.0),
            Sensor,
        ));
    }
}

pub fn check_coin_collection(
    mut commands: Commands,
    player_query: Query<&CollidingEntities, With<Player>>,
    collectible_query: Query<(Entity, &CoinValue), With<Collectible>>,
    mut score_events: MessageWriter<ScoreChangedEvent>,
    game_data: Res<GameData>,
) {
    let Ok(colliding) = player_query.single() else {
        return;
    };

    for &entity in colliding.iter() {
        if let Ok((collectible_entity, coin_value)) = collectible_query.get(entity) {
            score_events.write(ScoreChangedEvent(game_data.score + coin_value.0));
            commands.entity(collectible_entity).despawn();
        }
    }
}

pub fn despawn_level(
    mut commands: Commands,
    platforms: Query<Entity, With<Platform>>,
    collectibles: Query<Entity, With<Collectible>>,
) {
    for entity in platforms.iter().chain(collectibles.iter()) {
        commands.entity(entity).despawn();
    }
}
