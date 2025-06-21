use bevy::asset::AssetServer;
use bevy::hierarchy::DespawnRecursiveExt;
use bevy::input::ButtonInput;
use bevy::math::Vec3;
use bevy::prelude::{default, AlignItems, Color, Commands, Entity, JustifyContent, KeyCode, NodeBundle, PositionType, Query, Res, ResMut, SpriteBundle, Style, TextBundle, TextStyle, Transform, UiRect, Val, With};
use crate::enemy::spawn_wave_enemies;
use bevy::prelude::BuildChildren;
use crate::components::*;
use crate::resources::*;
use crate::states::Direction;
use crate::constants::*;

pub fn game_over_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    game_over: Res<GameOver>,
    query: Query<Entity, With<GameOverText>>,
) {
    if game_over.is_over && query.is_empty() {
        commands.spawn((
            NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    top: Val::Percent(50.0),
                    left: Val::Percent(50.0),
                    width: Val::Px(700.0),
                    height: Val::Px(180.0),
                    margin: UiRect {
                        left: Val::Px(-350.0),
                        top: Val::Px(-90.0),
                        ..default()
                    },
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                background_color: Color::NONE.into(),
                ..default()
            },
            GameOverUI,
            GameOverText,
        ))
            .with_children(|parent| {
                parent.spawn(
                    TextBundle::from_section(
                        "游戏结束\n按下R键重新开始",
                        TextStyle {
                            font: asset_server.load("fonts/SourceHanSansSC-Bold.otf"),
                            font_size: 56.0,
                            color: Color::RED,
                        },
                    ),
                );
            });
    }
}
pub fn restart_game(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
    mut game_over: ResMut<GameOver>,
    player_query: Query<Entity, With<Player>>,
    enemy_query: Query<Entity, With<Enemy>>,
    bullet_query: Query<Entity, With<Bullet>>,
    asset_server: Res<AssetServer>,
    game_over_ui_query: Query<Entity, With<GameOverUI>>,
    textures: Res<GameTextures>,
    mut wave: ResMut<WaveState>,
    mut score: ResMut<Score>,
    mut stats: ResMut<PlayerStats>,
    mut upgrade: ResMut<UpgradeSelection>,
) {
    if game_over.is_over && keyboard.just_pressed(KeyCode::KeyR) {
        // 清理所有实体
        for e in player_query.iter() {
            commands.entity(e).despawn();
        }
        for e in enemy_query.iter() {
            commands.entity(e).despawn();
        }
        for e in bullet_query.iter() {
            commands.entity(e).despawn();
        }
        for e in game_over_ui_query.iter() {
            commands.entity(e).despawn_recursive();
        }
        // 重置游戏状态
        wave.current_wave = 1;
        wave.enemies_remaining = 0;
        game_over.is_over = false;
        score.0 = 0;
        // 重置玩家属性（回到初始状态）
        stats.damage = 2;
        stats.attack_speed = ATTACK_INTERVAL;
        stats.kills = 0;
        stats.level = 1;
        stats.bullet_count = 8;
        // 重置升级选项
        upgrade.options.clear();
        upgrade.show = false;
        // 重新生成玩家和敌人
        commands.spawn((
            SpriteBundle {
                texture: textures.player[Direction::Down as usize].clone(),
                transform: Transform::from_xyz(0.0, -200.0, 0.0).with_scale(Vec3::splat(1.0)),
                ..default()
            },
            Player,
            Health { current: 5, max: 5 },
            Facing(Direction::Down),
        ));
        spawn_wave_enemies(&mut commands, &mut *wave, &asset_server, &textures);
    }
}