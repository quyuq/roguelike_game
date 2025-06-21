mod components;
mod resources;
mod states;
mod setup;
mod player;
mod enemy;
mod bullet;
mod ui;
mod upgrade;
mod gameover;
mod util;
mod constants;

use bevy::prelude::*;
use components::*;
use resources::*;
use states::{Direction, GameState, UpgradeOption};
use constants::*;
use crate::enemy::spawn_wave_enemies;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Roguelike Rust".into(),
                ..default()
            }),
            ..default()
        }))
        .insert_resource(HighScore(0))
        .init_state::<GameState>()
        .insert_resource(GameOver::default())
        .insert_resource(Score(0))
        .insert_resource(UpgradeSelection::default())
        .insert_resource(EnemySpawnTimer(Timer::from_seconds(3.0, TimerMode::Repeating)))
        .insert_resource(WaveState::default())
        .insert_resource(PlayerStats::default())
        .insert_resource(HurtTimer(Timer::from_seconds(1.0, TimerMode::Repeating)))
        .insert_resource(AttackTimer {
            timer: Timer::from_seconds(ATTACK_INTERVAL, TimerMode::Repeating)
        })
        .insert_resource(ClearColor(Color::rgb(0.1, 0.1, 0.1)))
        // 各模块启动/系统注册
        .add_systems(Startup, setup::load_textures)
        .add_systems(Startup, setup::setup_menu)
        .add_systems(OnEnter(GameState::Playing), setup::setup)
        .add_systems(Update, setup::menu_input.run_if(in_state(GameState::Menu)))
        .add_systems(Update, (
            ui::camera_follow.run_if(in_state(GameState::Playing).and_then(util::not_game_over)),
            player::player_movement.run_if(in_state(GameState::Playing).and_then(util::not_game_over)),
            enemy::wave_timer_advance.run_if(in_state(GameState::Playing).and_then(util::not_game_over)),
            enemy::enemy_follow.run_if(in_state(GameState::Playing).and_then(util::not_game_over)),
            setup::update_entity_texture.run_if(in_state(GameState::Playing).and_then(util::not_game_over)),
            enemy::continuous_enemy_spawn.run_if(in_state(GameState::Playing).and_then(util::not_game_over)),
            player::player_attack.run_if(in_state(GameState::Playing).and_then(util::not_game_over)),
            bullet::bullet_movement.run_if(in_state(GameState::Playing).and_then(util::not_game_over)),
            bullet::bullet_hit_enemy.run_if(in_state(GameState::Playing).and_then(util::not_game_over)),
            enemy::enemy_damage_player.run_if(in_state(GameState::Playing).and_then(util::not_game_over)),
            player::update_attack_timer.run_if(in_state(GameState::Playing).and_then(util::not_game_over)),
            upgrade::show_upgrade_ui.run_if(in_state(GameState::Playing).and_then(util::not_game_over)),
            upgrade::handle_upgrade_input.run_if(in_state(GameState::Playing)),
            ui::update_ui.run_if(in_state(GameState::Playing).and_then(util::not_game_over)),
            player::check_health.run_if(in_state(GameState::Playing)),
            enemy::enemy_avoidance.run_if(in_state(GameState::Playing).and_then(util::not_game_over)),
        ))
        .add_systems(Update, gameover::game_over_ui)
        .add_systems(Update, gameover::restart_game)
        .run();
}