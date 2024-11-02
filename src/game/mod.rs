mod player;
mod systems;
mod world;
mod constants;
mod block;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use player::PlayerPlugin;
use world::WorldPlugin;
use systems::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
            .add_plugins(WorldPlugin)
            .add_plugins(PlayerPlugin)
            .add_systems(Startup, spawn_camera);
    }
}