use bevy::prelude::*;
use avian2d::prelude::*;
use crate::common::*;
use super::components::{PatrolPath, PatrolDirection, EnemySpeed};

pub fn spawn_enemies(mut commands: Commands) {
    let enemies = [
        // Enemy on the ground (y=-200 ground top, enemy center at -200 + 16 = -184)
        (Vec2::new(-200.0, -184.0), Vec2::new(-50.0, -184.0)),
        // Enemy on the ground, right side
        (Vec2::new(100.0, -184.0), Vec2::new(250.0, -184.0)),
        // Enemy on a mid-height platform (assuming platform around y=-50)
        (Vec2::new(-100.0, -34.0), Vec2::new(50.0, -34.0)),
        // Enemy on a higher platform (assuming platform around y=100)
        (Vec2::new(150.0, 116.0), Vec2::new(300.0, 116.0)),
    ];

    for (start, end) in enemies {
        commands.spawn((
            Enemy,
            Sprite {
                color: Color::srgb(0.9, 0.2, 0.2),
                custom_size: Some(Vec2::new(32.0, 32.0)),
                ..default()
            },
            Transform::from_xyz(start.x, start.y, 0.0),
            RigidBody::Kinematic,
            Collider::rectangle(32.0, 32.0),
            PatrolPath { start, end },
            PatrolDirection(1.0),
            EnemySpeed(80.0),
            LinearVelocity::default(),
            CollidingEntities::default(),
        ));
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
    mut death_events: EventWriter<PlayerDiedEvent>,
    mut score_events: EventWriter<ScoreChangedEvent>,
    game_data: Res<GameData>,
) {
    let Ok((player_transform, colliding)) = player_query.get_single() else {
        return;
    };

    for &entity in colliding.iter() {
        let Ok((enemy_entity, enemy_transform)) = enemy_query.get(entity) else {
            continue;
        };

        let stomp_threshold = 16.0;
        if player_transform.translation.y > enemy_transform.translation.y + stomp_threshold {
            // Player stomped the enemy from above
            commands.entity(enemy_entity).despawn_recursive();
            score_events.send(ScoreChangedEvent(game_data.score + 100));
        } else {
            // Enemy hit the player
            death_events.send(PlayerDiedEvent);
        }
    }
}

pub fn despawn_enemies(mut commands: Commands, query: Query<Entity, With<Enemy>>) {
    for entity in &query {
        commands.entity(entity).despawn_recursive();
    }
}
