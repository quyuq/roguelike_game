use bevy::asset::{AssetServer, Handle};
use bevy::hierarchy::DespawnRecursiveExt;
use bevy::input::ButtonInput;
use bevy::math::Vec3;
use bevy::prelude::{default, AlignItems, Camera2dBundle, Color, Commands, Entity, Image, JustifyContent, KeyCode, NextState, NodeBundle, PositionType, Query, Res, ResMut, SpriteBundle, Style, TextBundle, TextSection, TextStyle, Transform, UiRect, Val, With};
use crate::enemy::spawn_wave_enemies;
use bevy::prelude::*;
use crate::constants::*;
use crate::resources::*;
use crate::components::*;
use crate::states::Direction;
use crate::states::GameState;

pub fn setup_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((Camera2dBundle::default(), MenuCamera));
    // 绝对居中Node + Text
    commands.spawn((
        NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                top: Val::Percent(50.0),
                left: Val::Percent(50.0),
                width: Val::Px(600.0),
                height: Val::Px(120.0),
                margin: UiRect {
                    left: Val::Px(-300.0),
                    top: Val::Px(-60.0),
                    ..default()
                },
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            background_color: Color::NONE.into(),
            ..default()
        },
        MenuUI,
        MenuText,
    ))
        .with_children(|parent| {
            parent.spawn(
                TextBundle::from_section(
                    "按下空格开始游戏",
                    TextStyle {
                        font: asset_server.load("fonts/SourceHanSansSC-Bold.otf"),
                        font_size: 44.0,
                        color: Color::WHITE,
                    },
                ),
            );
        });
}
pub fn menu_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
    mut commands: Commands,
    menu_ui_query: Query<Entity, With<MenuUI>>,
    menu_camera_query: Query<Entity, With<MenuCamera>>,
) {
    if keyboard.just_pressed(KeyCode::Space) {
        for e in menu_ui_query.iter() {
            commands.entity(e).despawn_recursive();
        }
        for cam in menu_camera_query.iter() {
            commands.entity(cam).despawn();
        }
        next_state.set(GameState::Playing);
        println!("Game Start!");
    }
}
// === 加载图片 ===
pub fn load_textures(mut commands: Commands, asset_server: Res<AssetServer>) {
    let player = [
        asset_server.load("textures/player_up.png"),
        asset_server.load("textures/player_down.png"),
        asset_server.load("textures/player_left.png"),
        asset_server.load("textures/player_right.png"),
    ];
    let enemy = [
        asset_server.load("textures/enemy_up.png"),
        asset_server.load("textures/enemy_down.png"),
        asset_server.load("textures/enemy_left.png"),
        asset_server.load("textures/enemy_right.png"),
    ];
    let boss = [
        asset_server.load("textures/boss_up.png"),
        asset_server.load("textures/boss_down.png"),
        asset_server.load("textures/boss_left.png"),
        asset_server.load("textures/boss_right.png"),
    ];
    commands.insert_resource(GameTextures { player, enemy, boss });
}
pub fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut wave: ResMut<WaveState>,
    textures: Res<GameTextures>,
    mut stats: ResMut<PlayerStats>,) {
    *stats = PlayerStats {
        damage: 2,
        attack_speed: ATTACK_INTERVAL,
        kills: 0,
        level: 1,
        bullet_count: 8,
    };
    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(0.0, 0.0, 999.9),
        ..default()
    });
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
    commands.spawn((
        TextBundle::from_sections([
            TextSection::new("属性: ", TextStyle {
                font: asset_server.load("fonts/SourceHanSansSC-Bold.otf"),
                font_size: 24.0,
                color: Color::WHITE,
            }),
            TextSection::new("攻击 2 | 攻速 1.00s | 弹幕数 8", TextStyle {
                font: asset_server.load("fonts/SourceHanSansSC-Bold.otf"),
                font_size: 24.0,
                color: Color::ORANGE,
            }),
        ]).with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(120.0), // <- 调整到更低
            left: Val::Px(10.0),
            ..default()
        }),
        StatsText,
    ));
    // 最高分
    commands.spawn((
        TextBundle::from_sections([
            TextSection::new("最高分: ", TextStyle {
                font: asset_server.load("fonts/SourceHanSansSC-Bold.otf"),
                font_size: 24.0,
                color: Color::WHITE,
            }),
            TextSection::new("0", TextStyle {
                font: asset_server.load("fonts/SourceHanSansSC-Bold.otf"),
                font_size: 24.0,
                color: Color::GOLD,
            }),
        ]).with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(10.0),     // 放到顶部
            left: Val::Px(400.0),   // 靠右
            ..default()
        }),
        HighScoreText,
    ));
    spawn_wave_enemies(&mut commands, &mut *wave, &asset_server, &textures);
    // UI
    commands.spawn((
        TextBundle::from_sections([
            TextSection::new("生命值: ", TextStyle {
                font: asset_server.load("fonts/SourceHanSansSC-Bold.otf"),
                font_size: 24.0,
                color: Color::WHITE,
            }),
            TextSection::new("5", TextStyle {
                font: asset_server.load("fonts/SourceHanSansSC-Bold.otf"),
                font_size: 24.0,
                color: Color::GREEN,
            }),
        ]).with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(10.0),
            left: Val::Px(10.0),
            ..default()
        }),
        HealthText,
    ));
    commands.spawn((
        TextBundle::from_sections([
            TextSection::new("得分: ", TextStyle {
                font: asset_server.load("fonts/SourceHanSansSC-Bold.otf"),
                font_size: 24.0,
                color: Color::WHITE,
            }),
            TextSection::new("0", TextStyle {
                font: asset_server.load("fonts/SourceHanSansSC-Bold.otf"),
                font_size: 24.0,
                color: Color::YELLOW,
            }),
        ]).with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(40.0),
            left: Val::Px(10.0),
            ..default()
        }),
        ScoreText,
    ));
    commands.spawn((
        TextBundle::from_sections([
            TextSection::new("波次: ", TextStyle {
                font: asset_server.load("fonts/SourceHanSansSC-Bold.otf"),
                font_size: 24.0,
                color: Color::CYAN,
            }),
            TextSection::new("1", TextStyle {
                font: asset_server.load("fonts/SourceHanSansSC-Bold.otf"),
                font_size: 24.0,
                color: Color::WHITE,
            }),
        ]).with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(70.0),
            left: Val::Px(10.0),
            ..default()
        }),
        WaveText,
    ));
    commands.spawn((
        TextBundle::from_sections([
            TextSection::new("首领生命值: ", TextStyle {
                font: asset_server.load("fonts/SourceHanSansSC-Bold.otf"),
                font_size: 28.0,
                color: Color::PURPLE,
            }),
            TextSection::new("0/0", TextStyle {
                font: asset_server.load("fonts/SourceHanSansSC-Bold.otf"),
                font_size: 28.0,
                color: Color::WHITE,
            }),
        ])
            .with_style(Style {
                position_type: PositionType::Absolute,
                top: Val::Px(10.0),
                left: Val::Percent(40.0),
                ..default()
            }),
        BossHealthText,
    ));

}
// === 定期同步方向 ===
pub fn update_entity_texture(
    textures: Res<GameTextures>,
    mut query: Query<(&mut Handle<Image>, &Facing, Option<&Player>, Option<&Boss>, Option<&Enemy>)>,
) {
    for (mut texture, facing, is_player, is_boss, is_enemy) in query.iter_mut() {
        let dir_idx = facing.0 as usize;
        if is_player.is_some() {
            *texture = textures.player[dir_idx].clone();
        } else if is_boss.is_some() {
            *texture = textures.boss[dir_idx].clone();
        } else if is_enemy.is_some() {
            *texture = textures.enemy[dir_idx].clone();
        }
    }
}