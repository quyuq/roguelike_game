use bevy::prelude::*;
use crate::states::{UpgradeOption, Direction};

#[derive(Resource, Default)] pub struct HighScore(pub i32);
#[derive(Resource)] pub struct AttackTimer { pub timer: Timer }
#[derive(Resource)] pub struct Score(pub i32);
#[derive(Resource)]
pub struct WaveState {
    pub current_wave: u32,
    pub enemies_remaining: u32,
    pub wave_timer: Timer,
}

impl Default for WaveState {
    fn default() -> Self {
        Self {
            current_wave: 1,
            enemies_remaining: 0,
            wave_timer: Timer::from_seconds(15.0, TimerMode::Repeating),
        }
    }
}
#[derive(Resource)]
pub struct GameTextures {
    pub player: [Handle<Image>; 4],
    pub enemy:  [Handle<Image>; 4],
    pub boss:   [Handle<Image>; 4],
}
#[derive(Resource, Default)] pub struct GameOver { pub is_over: bool }
#[derive(Resource)] pub struct EnemySpawnTimer(pub Timer);
#[derive(Resource, Default)] pub struct PlayerStats {
    pub damage: i32,
    pub attack_speed: f32,
    pub kills: u32,
    pub level: u32,
    pub bullet_count: usize,
}
#[derive(Resource, Default)] pub struct UpgradeSelection {
    pub options: Vec<UpgradeOption>,
    pub show: bool,
}
#[derive(Resource)] pub struct HurtTimer(pub Timer);
