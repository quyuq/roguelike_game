use bevy::input::ButtonInput;
use bevy::math::Vec3;
use bevy::prelude::{default, Color, Commands, Entity, KeyCode, Query, Res, ResMut, Sprite, SpriteBundle, Time, Transform, With};
use crate::{AttackTimer, Bullet, BulletDirection, Direction, Facing, GameOver, Health, HighScore, Player, PlayerStats, Score, UpgradeSelection, PLAYER_SPEED};

pub fn player_movement(
    keyboard: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut query: Query<(&mut Transform, &mut Facing), With<Player>>,
    upgrade: Res<UpgradeSelection>,
) {
    if upgrade.show { return; }
    if let Ok((mut transform, mut facing)) = query.get_single_mut() {
        let mut direction = Vec3::ZERO;
        let mut new_facing = facing.0;
        if keyboard.pressed(KeyCode::KeyW) { direction.y += 1.0; new_facing = Direction::Up; }
        if keyboard.pressed(KeyCode::KeyS) { direction.y -= 1.0; new_facing = Direction::Down; }
        if keyboard.pressed(KeyCode::KeyA) { direction.x -= 1.0; new_facing = Direction::Left; }
        if keyboard.pressed(KeyCode::KeyD) { direction.x += 1.0; new_facing = Direction::Right; }
        if direction != Vec3::ZERO {
            transform.translation += direction.normalize() * PLAYER_SPEED * time.delta_seconds();
            facing.0 = new_facing;
        }
    }
}
pub fn player_attack(
    mut commands: Commands,
    time: Res<Time>,
    mut timer: ResMut<AttackTimer>,
    player: Query<&Transform, With<Player>>,
    stats: Res<PlayerStats>,
    upgrade: Res<UpgradeSelection>,
) {
    if upgrade.show { return; }
    timer.timer.tick(time.delta());
    if timer.timer.just_finished() {
        if let Ok(tf) = player.get_single() {
            for i in 0..stats.bullet_count {
                let angle = i as f32 * std::f32::consts::TAU / stats.bullet_count as f32;
                let direction = Vec3::new(angle.cos(), angle.sin(), 0.0);

                commands.spawn((
                    SpriteBundle {
                        transform: Transform::from_translation(tf.translation)
                            .with_scale(Vec3::splat(10.0)),
                        sprite: Sprite { color: Color::GOLD, ..default() },
                        ..default()
                    },
                    Bullet,
                    BulletDirection(direction),
                ));
            }
        }
    }
}
pub fn update_attack_timer(
    mut attack_timer: ResMut<AttackTimer>,
    stats: Res<PlayerStats>,
) {
    attack_timer.timer.set_duration(std::time::Duration::from_secs_f32(stats.attack_speed));
}
pub fn check_health(
    mut commands: Commands,
    player: Query<(Entity, &Health), With<Player>>,
    mut game_over: ResMut<GameOver>,
    score: Res<Score>,
    mut high_score: ResMut<HighScore>,
) {
    if game_over.is_over {
        return;
    }
    if let Ok((entity, health)) = player.get_single() {
        if health.current <= 0 {
            if score.0 > high_score.0 {
                high_score.0 = score.0;
                println!("NEW HIGH SCORE: {}", score.0);
            }
            println!("Game Over!");
            game_over.is_over = true;
            if commands.get_entity(entity).is_some() {
                commands.entity(entity).despawn();
            }
        }
    }
}