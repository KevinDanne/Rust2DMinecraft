use bevy::prelude::*;

pub fn create_world(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let world_width_blocks = 5;
    let world_height_blocks = 5;
    let block_size: f32 = 128.0;

    for w in 0..world_width_blocks {
        for h in 0..world_height_blocks {
            let asset_path = if h == 0 { "sprites/tiles/dirt_grass.png" } else { "sprites/tiles/dirt.png" };
            let block_pos_x =  w as f32 * block_size;
            let block_pos_y =  -h as f32 * block_size;

            commands.spawn(SpriteBundle {
                transform: Transform::from_xyz(block_pos_x, block_pos_y, 0.0),
                texture: asset_server.load(asset_path),
                ..default()
            });
        }
    }
}