use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use crate::game::constants::BLOCK_SIZE;
use crate::game::block::components::Block;

pub fn create_world(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let world_width_blocks = 10;
    let world_block_depth = 5;
    let block_size: f32 = 128.0;

    for w in -world_width_blocks / 2..world_width_blocks / 2 {
        for h in 0..world_block_depth {
            let asset_path = if h == 0 { "sprites/tiles/dirt_grass.png" } else { "sprites/tiles/dirt.png" };
            let block_pos_x = w as f32 * block_size;
            let block_pos_y = -h as f32 * block_size;

            commands.spawn(
                (
                    SpriteBundle {
                        transform: Transform::from_xyz(block_pos_x, block_pos_y, 0.0),
                        texture: asset_server.load(asset_path),
                        ..default()
                    },
                    Collider::cuboid(BLOCK_SIZE / 2.0, BLOCK_SIZE / 2.0),
                    Block {}
                )
            );
        }
    }
}