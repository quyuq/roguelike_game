use bevy::prelude::{Camera, ParamSet, Query, Res, Text, Transform, With, Without};
use crate::{Boss, BossHealthText, Enemy, Health, HealthText, HighScore, HighScoreText, Player, PlayerStats, Score, ScoreText, StatsText, WaveState, WaveText, WINDOW_HEIGHT, WINDOW_WIDTH, WORLD_HEIGHT, WORLD_WIDTH};

pub fn camera_follow(
    mut camera_query: Query<&mut Transform, (With<Camera>, Without<Player>)>,
    player_query: Query<&Transform, With<Player>>,
) {
    let player_tf = match player_query.get_single() {
        Ok(transform) => transform,
        Err(_) => {
            return;
        }
    };
    let mut camera_tf = match camera_query.get_single_mut() {
        Ok(transform) => transform,
        Err(_) => {
            eprintln!("Camera entity not found!");
            return;
        }
    };
    let half_window_w = WINDOW_WIDTH / 2.0;
    let half_window_h = WINDOW_HEIGHT / 2.0;
    let mut target_x = player_tf.translation.x;
    let mut target_y = player_tf.translation.y;
    target_x = target_x.clamp(
        -WORLD_WIDTH / 2.0 + half_window_w,
        WORLD_WIDTH / 2.0 - half_window_w,
    );
    target_y = target_y.clamp(
        -WORLD_HEIGHT / 2.0 + half_window_h,
        WORLD_HEIGHT / 2.0 - half_window_h,
    );
    camera_tf.translation.x = target_x;
    camera_tf.translation.y = target_y;
}
pub fn update_ui(
    health_query: Query<&Health, With<Player>>,
    boss_query: Query<(&Health, &Transform), (With<Boss>, With<Enemy>)>,
    player_tf_query: Query<&Transform, With<Player>>,
    stats: Res<PlayerStats>,
    mut queries: ParamSet<(
        Query<&mut Text, With<ScoreText>>,
        Query<&mut Text, With<HealthText>>,
        Query<&mut Text, With<WaveText>>,
        Query<&mut Text, With<BossHealthText>>,
        Query<&mut Text, With<StatsText>>,
        Query<&mut Text, With<HighScoreText>>,
    )>,
    score: Res<Score>,
    wave: Res<WaveState>,
    high_score: Res<HighScore>,
){
    if let Ok(health) = health_query.get_single() {
        if let Ok(mut health_text) = queries.p1().get_single_mut() {
            health_text.sections[1].value = format!("{}/{}", health.current, health.max);
        }
    }
    if let Ok(mut score_text) = queries.p0().get_single_mut() {
        score_text.sections[1].value = format!("{}", score.0);
    }
    if let Ok(mut wave_text) = queries.p2().get_single_mut() {
        wave_text.sections[1].value = format!("{}", wave.current_wave);
    }
    if let Ok(mut boss_text) = queries.p3().get_single_mut() {
        if let Ok(player_tf) = player_tf_query.get_single() {
            let mut nearest_boss: Option<(&Health, f32)> = None;
            for (boss_health, boss_tf) in boss_query.iter() {
                let distance = boss_tf.translation.distance(player_tf.translation);
                println!("Found boss with HP: {}/{} at distance {}", boss_health.current, boss_health.max, distance);
                if nearest_boss.is_none() || distance < nearest_boss.unwrap().1 {
                    nearest_boss = Some((boss_health, distance));
                }
            }
            if let Some((health, _)) = nearest_boss {
                boss_text.sections[0].value = "Boss HP: ".to_string();
                boss_text.sections[1].value = format!("{}/{}", health.current, health.max);
            } else {
                boss_text.sections[0].value.clear();
                boss_text.sections[1].value.clear();
            }
        }
    }
    if let Ok(mut stats_text) = queries.p4().get_single_mut() {
        stats_text.sections[1].value = format!(
            "攻击 {} | 攻速 {:.2}s | 弹幕数 {}",
            stats.damage,
            stats.attack_speed,
            stats.bullet_count
        );
    }
    if let Ok(mut high_score_text) = queries.p5().get_single_mut() {
        high_score_text.sections[1].value = format!("{}", high_score.0);
    }
}