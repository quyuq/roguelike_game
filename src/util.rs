use bevy::prelude::*;
use crate::resources::*;

pub fn not_game_over(game_over: Res<GameOver>) -> bool {
    !game_over.is_over
}
