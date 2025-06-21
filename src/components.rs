use bevy::prelude::*;

#[derive(Component)] pub struct Player;
#[derive(Component)] pub struct HighScoreText;
#[derive(Component)] pub struct MenuUI;
#[derive(Component)] pub struct GameOverUI;
#[derive(Component)] pub struct Enemy;
#[derive(Component)] pub struct Boss;
#[derive(Component)] pub struct BossHealthText;
#[derive(Component)] pub struct Bullet;
#[derive(Component)] pub struct BulletDirection(pub Vec3);
#[derive(Component)] pub struct GameOverText;
#[derive(Component)] pub struct MenuText;
#[derive(Component)] pub struct MenuCamera;
#[derive(Component)] pub struct UpgradeUI;
#[derive(Component)] pub struct StatsText;
#[derive(Component)] pub struct ScoreText;
#[derive(Component)] pub struct HealthText;
#[derive(Component)] pub struct WaveText;
#[derive(Component, Copy, Clone)] pub struct Facing(pub crate::states::Direction);
#[derive(Component)] pub struct Health { pub current: i32, pub max: i32 }
#[derive(Component)] pub struct Collider { pub radius: f32 }
