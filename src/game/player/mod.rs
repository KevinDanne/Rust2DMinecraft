mod systems;
mod components;
mod constants;

use bevy::prelude::*;
use systems::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, spawn_player)
            .add_systems(Update, player_movement)
            .add_systems(Update, move_camera)
            .add_systems(Update, block_mining);
    }
}
