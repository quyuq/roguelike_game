use bevy::asset::AssetServer;
use bevy::hierarchy::DespawnRecursiveExt;
use bevy::input::ButtonInput;
use bevy::prelude::{default, AlignItems, Color, Commands, Entity, FlexDirection, JustifyContent, KeyCode, NodeBundle, PositionType, Query, Res, ResMut, Style, TextBundle, TextStyle, UiRect, Val, With};
use crate::{Health, Player, PlayerStats, UpgradeOption, UpgradeSelection, UpgradeUI};
use bevy::prelude::BuildChildren;

pub fn show_upgrade_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    upgrade: Res<UpgradeSelection>,
    old_ui: Query<Entity, With<UpgradeUI>>,
) {
    for entity in old_ui.iter() {
        commands.entity(entity).despawn_recursive();
    }
    if !upgrade.show {
        return;
    }
    let panel_width = 480.0;
    let panel_height = 80.0 * upgrade.options.len() as f32;
    commands.spawn((
        NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                top: Val::Percent(50.0),
                left: Val::Percent(50.0),
                width: Val::Px(panel_width),
                height: Val::Px(panel_height),
                margin: UiRect {
                    left: Val::Px(-panel_width / 2.0),
                    top: Val::Px(-panel_height / 2.0),
                    ..default()
                },
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            background_color: Color::rgba(0.08,0.08,0.08,0.92).into(),
            ..default()
        },
        UpgradeUI,
    )).with_children(|parent| {
        for (i, option) in upgrade.options.iter().enumerate() {
            let label = match option {
                UpgradeOption::IncreaseDamage => "提升攻击力",
                UpgradeOption::IncreaseAttackSpeed => "提升攻速",
                UpgradeOption::IncreaseBulletCount => "增加弹幕数量",
            };
            parent.spawn(
                TextBundle::from_section(
                    format!("按下 {}：{}", i + 1, label),
                    TextStyle {
                        font: asset_server.load("fonts/SourceHanSansSC-Bold.otf"),
                        font_size: 36.0,
                        color: Color::YELLOW,
                    },
                )
                    .with_style(Style {
                        margin: UiRect::vertical(Val::Px(8.0)),
                        ..default()
                    }),
            );
        }
    });
}
pub fn handle_upgrade_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut stats: ResMut<PlayerStats>,
    mut upgrade: ResMut<UpgradeSelection>,
    mut commands: Commands,
    ui_query: Query<Entity, With<UpgradeUI>>,
    mut player_query: Query<&mut Health, With<Player>>,
) {
    if !upgrade.show {
        return;
    }
    if let Some(index) = [KeyCode::Digit1, KeyCode::Digit2, KeyCode::Digit3]
        .iter()
        .enumerate()
        .find_map(|(i, key)| if keyboard.just_pressed(*key) { Some(i) } else { None })
    {
        if index < upgrade.options.len() {
            match upgrade.options[index] {
                UpgradeOption::IncreaseDamage => {
                    stats.damage += 2;
                }
                UpgradeOption::IncreaseAttackSpeed => {
                    stats.attack_speed *= 0.8;
                }
                UpgradeOption::IncreaseBulletCount => {
                    stats.bullet_count += 2;
                }
            }
            // 升级回血
            if let Ok(mut health) = player_query.get_single_mut() {
                health.current = (health.current + 1).min(health.max);
            }
        }
        for e in ui_query.iter() {
            commands.entity(e).despawn_recursive();
        }
        upgrade.show = false;
        upgrade.options.clear();
    }
}