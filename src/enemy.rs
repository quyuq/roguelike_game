use bevy::asset::AssetServer;
use bevy::math::Vec3;
use bevy::prelude::{default, Commands, Query, Res, ResMut, SpriteBundle, Time, Transform, With, Without};
use rand::Rng;
use crate::components::*;
use crate::resources::*;
use crate::states::Direction;
use crate::constants::*;

pub fn continuous_enemy_spawn(
    mut commands: Commands,
    time: Res<Time>,
    mut timer: ResMut<EnemySpawnTimer>,
    mut wave: ResMut<WaveState>,
    _asset_server: Res<AssetServer>,
    player_query: Query<&Transform, With<Player>>,
    upgrade: Res<UpgradeSelection>,
    textures: Res<GameTextures>,
) {
    if upgrade.show { return; }
    timer.0.tick(time.delta());
    if !timer.0.just_finished() {
        return;
    }
    let Ok(player_tf) = player_query.get_single() else { return; };
    let mut rng = rand::thread_rng();
    let enemies_to_spawn = 2 + wave.current_wave ;
    for _ in 0..enemies_to_spawn {
        let angle = rng.gen_range(0.0..std::f32::consts::TAU);
        let radius = rng.gen_range(300.0..500.0);
        let offset = Vec3::new(radius * angle.cos(), radius * angle.sin(), 0.0);
        let spawn_pos = player_tf.translation + offset;
        commands.spawn((
            SpriteBundle {
                texture: textures.enemy[Direction::Down as usize].clone(),
                transform: Transform::from_translation(spawn_pos).with_scale(Vec3::splat(1.5)),
                ..default()
            },
            Enemy,
            Health { current: 4 + wave.current_wave as i32, max: 4 + wave.current_wave as i32 },
            Collider { radius: 16.0 },
            Facing(Direction::Down),
        ));
        wave.enemies_remaining += 1;
    }
}
pub fn enemy_follow(
    time: Res<Time>,
    upgrade: Res<UpgradeSelection>,
    player_query: Query<&Transform, With<Player>>,
    mut enemies: Query<(&mut Transform, &mut Facing), (With<Enemy>, Without<Player>)>,
) {
    if upgrade.show { return; }
    if let Ok(player) = player_query.get_single() {
        for (mut enemy, mut facing) in enemies.iter_mut() {
            let dir = player.translation - enemy.translation;
            if dir.length() > 10.0 {
                let v = dir.normalize();
                // 主轴为 Y
                if v.y.abs() > v.x.abs() {
                    facing.0 = if v.y > 0.0 { Direction::Up } else { Direction::Down };
                } else {
                    facing.0 = if v.x > 0.0 { Direction::Right } else { Direction::Left };
                }
                enemy.translation += v * ENEMY_SPEED * time.delta_seconds();
            }
        }
    }
}
pub fn enemy_damage_player(
    time: Res<Time>,
    mut timer: ResMut<HurtTimer>,
    mut player_query: Query<(&mut Health, &Transform), With<Player>>,
    enemies: Query<(&Transform, &Collider), With<Enemy>>,
    upgrade: Res<UpgradeSelection>,
) {
    if upgrade.show { return; }
    timer.0.tick(time.delta());
    if let Ok((mut health, player_tf)) = player_query.get_single_mut() {
        if !timer.0.finished() { return; }
        for (enemy_tf, collider) in enemies.iter() {
            let distance = player_tf.translation.distance(enemy_tf.translation);
            if distance < 20.0 + collider.radius { // 20.0是玩家半径
                health.current -= 1;
                timer.0.reset();
                break;
            }
        }
    }
}
pub fn wave_timer_advance(
    time: Res<Time>,
    mut wave: ResMut<WaveState>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    textures: Res<GameTextures>,
) {
    wave.wave_timer.tick(time.delta());
    if wave.wave_timer.just_finished() {
        wave.current_wave += 1;
        println!("Wave advanced to {}", wave.current_wave);
        spawn_wave_enemies(&mut commands, &mut *wave, &asset_server, &textures);
    }
}
pub fn enemy_avoidance(
    mut enemies: Query<(&mut Transform, &Collider), With<Enemy>>,
    time: Res<Time>,
    upgrade: Res<UpgradeSelection>,
) {
    if upgrade.show { return; }
    let mut positions: Vec<Vec3> = Vec::new();
    for (mut tf, collider) in enemies.iter_mut() {
        for pos in &positions {
            let diff = tf.translation - *pos;
            if diff.length() < collider.radius * 2.0 {
                // 让敌人互相“推开”
                tf.translation += diff.normalize() * 10.0 * time.delta_seconds();
            }
        }
        positions.push(tf.translation);
    }
}
pub fn spawn_wave_enemies(
    commands: &mut Commands,
    wave: &mut WaveState,
    _asset_server: &Res<AssetServer>,
    textures: &Res<GameTextures>,) {
    let mut rng = rand::thread_rng();
    let base = 5 + (wave.current_wave * wave.current_wave) / 2;
    wave.enemies_remaining = base;
    for _ in 0..base {
        let x = rng.gen_range(-300.0..300.0);
        let y = rng.gen_range(-200.0..200.0);
        commands.spawn((
            SpriteBundle {
                texture: textures.enemy[Direction::Down as usize].clone(),
                transform: Transform::from_xyz(x, y, 0.0).with_scale(Vec3::splat(1.5)),
                ..default()
            },
            Enemy,
            Health { current: 4 + wave.current_wave as i32, max: 4 + wave.current_wave as i32 },
            Collider { radius: 25.0 },
            Facing(Direction::Down),
        ));
    }
    if wave.current_wave % 5 == 0 {
        let x = rng.gen_range(-200.0..200.0);
        let y = rng.gen_range(100.0..300.0);
        commands.spawn((
            SpriteBundle {
                texture: textures.boss[Direction::Down as usize].clone(),
                transform: Transform::from_xyz(x, y, 0.0).with_scale(Vec3::splat(2.5)),
                ..default()
            },
            Enemy,
            Boss,
            Health { current: 100 + 10 * wave.current_wave as i32, max: 100 + 10 * wave.current_wave as i32 },
            Collider { radius: 48.0 },
            Facing(Direction::Down),
        ));
        println!(
            "Spawned Boss with HP {}/{}",
            100 + 10 * wave.current_wave as i32,
            100 + 10 * wave.current_wave as i32
        );
        wave.enemies_remaining += 1;
    }
    println!("Wave {} started with {} enemies", wave.current_wave, wave.enemies_remaining);
}
