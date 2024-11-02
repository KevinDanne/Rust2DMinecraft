mod player;
mod systems;
mod world;
mod constants;

use bevy::prelude::*;
use player::PlayerPlugin;
use world::WorldPlugin;
use systems::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(WorldPlugin)
            .add_plugins(PlayerPlugin)
            .add_systems(Startup, spawn_camera);
    }
}