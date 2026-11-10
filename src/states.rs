use bevy::ecs::schedule::States;

#[derive(Copy, Clone)]
pub enum Direction { Up = 0, Down = 1, Left = 2, Right = 3 }

#[derive(Clone)]
pub enum UpgradeOption {
    IncreaseDamage,
    IncreaseAttackSpeed,
    IncreaseBulletCount,
    IncreaseMaxHealth,
    HealFull,
}

#[derive(States, Clone, Eq, PartialEq, Debug, Hash, Default)]
pub enum GameState {
    #[default]
    Menu,
    Playing,
    GameOver,
}
