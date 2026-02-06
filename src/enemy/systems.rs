use bevy::prelude::*;
use avian2d::prelude::*;
use crate::common::*;
use super::components::{PatrolPath, PatrolDirection, EnemySpeed, Stationary};

pub fn spawn_enemies(mut commands: Commands) {
    // (start, end, speed, color, width, height, is_stationary)
    let enemies: &[(Vec2, Vec2, f32, Color, f32, f32, bool)] = &[
        // 2 slow patrols on ground (speed 60, lighter red)
        (Vec2::new(-500.0, -184.0), Vec2::new(-350.0, -184.0), 60.0, Color::srgb(1.0, 0.4, 0.4), 32.0, 32.0, false),
        (Vec2::new(350.0, -184.0), Vec2::new(500.0, -184.0), 60.0, Color::srgb(1.0, 0.4, 0.4), 32.0, 32.0, false),

        // 2 normal patrols on tier 1 (speed 80)
        (Vec2::new(-250.0, -64.0), Vec2::new(-150.0, -64.0), 80.0, Color::srgb(0.9, 0.2, 0.2), 32.0, 32.0, false),
        (Vec2::new(350.0, -44.0), Vec2::new(450.0, -44.0), 80.0, Color::srgb(0.9, 0.2, 0.2), 32.0, 32.0, false),

        // 2 fast patrols on tier 2 (speed 120, darker red)
        (Vec2::new(-300.0, 56.0), Vec2::new(-200.0, 56.0), 120.0, Color::srgb(0.7, 0.1, 0.1), 32.0, 32.0, false),
        (Vec2::new(400.0, 116.0), Vec2::new(500.0, 116.0), 120.0, Color::srgb(0.7, 0.1, 0.1), 32.0, 32.0, false),

        // 1 fast patrol on tier 3
        (Vec2::new(100.0, 196.0), Vec2::new(200.0, 196.0), 120.0, Color::srgb(0.7, 0.1, 0.1), 32.0, 32.0, false),

        // 3 stationary blockers (wider, darkest red, speed 0, start==end)
        (Vec2::new(0.0, -24.0), Vec2::new(0.0, -24.0), 0.0, Color::srgb(0.5, 0.0, 0.0), 48.0, 32.0, true),
        (Vec2::new(-100.0, 256.0), Vec2::new(-100.0, 256.0), 0.0, Color::srgb(0.5, 0.0, 0.0), 48.0, 32.0, true),
        (Vec2::new(100.0, 376.0), Vec2::new(100.0, 376.0), 0.0, Color::srgb(0.5, 0.0, 0.0), 48.0, 32.0, true),
    ];

    for &(start, end, speed, color, w, h, is_stationary) in enemies {
        let mut entity_commands = commands.spawn((
            Enemy,
            Sprite {
                color,
                custom_size: Some(Vec2::new(w, h)),
                ..default()
            },
            Transform::from_xyz(start.x, start.y, 0.0),
            RigidBody::Kinematic,
            Collider::rectangle(w, h),
            PatrolPath { start, end },
            PatrolDirection(1.0),
            EnemySpeed(speed),
            LinearVelocity::default(),
            CollidingEntities::default(),
        ));

        if is_stationary {
            entity_commands.insert(Stationary);
        }
    }
}

pub fn enemy_patrol(
    mut query: Query<(
        &Transform,
        &PatrolPath,
        &mut PatrolDirection,
        &EnemySpeed,
        &mut LinearVelocity,
    ), With<Enemy>>,
) {
    for (transform, path, mut direction, speed, mut velocity) in &mut query {
        velocity.x = direction.0 * speed.0;
        velocity.y = 0.0;

        // Flip direction when reaching patrol endpoints
        if direction.0 > 0.0 && transform.translation.x >= path.end.x {
            direction.0 = -1.0;
        } else if direction.0 < 0.0 && transform.translation.x <= path.start.x {
            direction.0 = 1.0;
        }
    }
}

pub fn enemy_player_collision(
    mut commands: Commands,
    player_query: Query<(&Transform, &CollidingEntities), With<Player>>,
    enemy_query: Query<(Entity, &Transform), With<Enemy>>,
    mut death_events: MessageWriter<PlayerDiedEvent>,
    mut score_events: MessageWriter<ScoreChangedEvent>,
    game_data: Res<GameData>,
) {
    let Ok((player_transform, colliding)) = player_query.single() else {
        return;
    };

    for &entity in colliding.iter() {
        let Ok((enemy_entity, enemy_transform)) = enemy_query.get(entity) else {
            continue;
        };

        let stomp_threshold = 16.0;
        if player_transform.translation.y > enemy_transform.translation.y + stomp_threshold {
            // Player stomped the enemy from above
            commands.entity(enemy_entity).despawn();
            score_events.write(ScoreChangedEvent(game_data.score + 100));
        } else {
            // Enemy hit the player
            death_events.write(PlayerDiedEvent);
        }
    }
}

pub fn despawn_enemies(mut commands: Commands, query: Query<Entity, With<Enemy>>) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}
