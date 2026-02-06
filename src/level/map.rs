use bevy::prelude::*;
use avian2d::prelude::*;
use crate::common::*;
use crate::level::components::CoinValue;

pub fn spawn_level(mut commands: Commands) {
    // (x, y, width, height)
    let platforms: &[(f32, f32, f32, f32)] = &[
        // Ground
        (0.0, -200.0, 2000.0, 32.0),
        // Floating platforms - left side
        (-400.0, -100.0, 160.0, 16.0),
        (-250.0, 0.0, 128.0, 16.0),
        (-500.0, 80.0, 128.0, 16.0),
        (-150.0, 150.0, 192.0, 16.0),
        // Floating platforms - center
        (50.0, -50.0, 128.0, 16.0),
        (0.0, 100.0, 160.0, 16.0),
        // Floating platforms - right side
        (250.0, -80.0, 128.0, 16.0),
        (400.0, 20.0, 160.0, 16.0),
        (300.0, 140.0, 128.0, 16.0),
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
    // (x, y) positions for coins, placed on or near platforms
    let coins: &[(f32, f32)] = &[
        (-400.0, -75.0),   // above left platform
        (-250.0, 25.0),    // above second left platform
        (-500.0, 105.0),   // above third left platform
        (50.0, -25.0),     // above center-low platform
        (0.0, 125.0),      // above center-high platform
        (250.0, -55.0),    // above right-low platform
        (400.0, 45.0),     // above right-mid platform
        (300.0, 165.0),    // above right-high platform
    ];

    for &(x, y) in coins {
        commands.spawn((
            Collectible,
            CoinValue(10),
            Sprite {
                color: Color::srgb(1.0, 0.84, 0.0),
                custom_size: Some(Vec2::new(16.0, 16.0)),
                ..default()
            },
            Transform::from_xyz(x, y, 0.0),
        ));
    }
}

pub fn despawn_level(
    mut commands: Commands,
    platforms: Query<Entity, With<Platform>>,
    collectibles: Query<Entity, With<Collectible>>,
) {
    for entity in platforms.iter().chain(collectibles.iter()) {
        commands.entity(entity).despawn_recursive();
    }
}
