use bevy::prelude::{Commands, Entity, Query, Res, ResMut, Time, Transform, With, Without};
use rand::prelude::SliceRandom;
use crate::{Boss, Bullet, BulletDirection, Collider, Enemy, Health, Player, PlayerStats, Score, UpgradeOption, UpgradeSelection, WaveState, BULLET_SPEED, WORLD_HEIGHT, WORLD_WIDTH};

pub fn bullet_movement(
    time: Res<Time>,
    mut commands: Commands,
    mut bullets: Query<(Entity, &mut Transform, &BulletDirection), With<Bullet>>,
    upgrade: Res<UpgradeSelection>,
) {
    if upgrade.show { return; }
    for (e, mut tf, dir) in bullets.iter_mut() {
        tf.translation += dir.0.normalize() * BULLET_SPEED * time.delta_seconds();

        if tf.translation.x < -WORLD_WIDTH / 2.0
            || tf.translation.x > WORLD_WIDTH / 2.0
            || tf.translation.y < -WORLD_HEIGHT / 2.0
            || tf.translation.y > WORLD_HEIGHT / 2.0
        {
            commands.entity(e).despawn();
        }
    }
}
pub fn bullet_hit_enemy(
    mut commands: Commands,
    mut bullets: Query<(Entity, &Transform), With<Bullet>>,
    mut enemies: Query<(Entity, &Transform, &mut Health, Option<&Boss>, &Collider), (With<Enemy>, Without<Player>)>,
    mut score: ResMut<Score>,
    mut wave: ResMut<WaveState>,
    mut stats: ResMut<PlayerStats>,
    mut upgrade_selection: ResMut<UpgradeSelection>,
) {
    if upgrade_selection.show {
        return;
    }
    let mut to_despawn: Vec<Entity> = vec![];
    for (b_entity, b_tf) in bullets.iter_mut() {
        for (e_entity, e_tf, mut health, is_boss, collider) in enemies.iter_mut() {
            let distance = b_tf.translation.distance(e_tf.translation);
            if distance < collider.radius + 8.0 { // 8.0为子弹半径
                health.current -= stats.damage;
                if health.current <= 0 {
                    commands.entity(e_entity).despawn();
                    wave.enemies_remaining = wave.enemies_remaining.saturating_sub(1);
                    score.0 += if is_boss.is_some() { 100 } else { 10 };
                    stats.kills += 1;
                    let required_kills = stats.level * stats.level * 5; // 可根据需要调整倍率
                    if stats.kills >= required_kills {
                        stats.level += 1;
                        let mut rng = rand::thread_rng();
                        let mut available = vec![
                            UpgradeOption::IncreaseDamage,
                            UpgradeOption::IncreaseAttackSpeed,
                            UpgradeOption::IncreaseBulletCount,
                        ];
                        available.shuffle(&mut rng);
                        upgrade_selection.options = available.into_iter().take(3).collect();
                        upgrade_selection.show = true;
                        println!("Level up! Choose one upgrade!");
                    }
                }
                // 使用临时列表统一处理 despawn，避免重复调用
                to_despawn.push(b_entity);
                break;
            }
        }
    }
    for entity in to_despawn {
        if commands.get_entity(entity).is_some() {
            commands.entity(entity).despawn();
        }
    }
}