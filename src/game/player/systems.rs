use bevy::prelude::*;

use super::components::Player;
use super::constants::*;

pub fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.spawn(
        (
            TransformBundle {
                local: Transform::from_xyz(PLAYER_SPAWN_POSITION.x, PLAYER_SPAWN_POSITION.y, PLAYER_SPAWN_POSITION.z),
                ..default()
            },
            Player {},
        )
    )
        .with_children(|parent| {
            parent.spawn(SpriteBundle {
                transform: Transform::from_xyz(PLAYER_TOP_TRANSFORM_OFFSET.x, PLAYER_TOP_TRANSFORM_OFFSET.y, PLAYER_TOP_TRANSFORM_OFFSET.z),
                texture: asset_server.load(PLAYER_TOP_SPRITE_PATH),
                ..default()
            });
            parent.spawn(SpriteBundle {
                transform: Transform::from_xyz(PLAYER_BOTTOM_TRANSFORM_OFFSET.x, PLAYER_BOTTOM_TRANSFORM_OFFSET.y, PLAYER_BOTTOM_TRANSFORM_OFFSET.z),
                texture: asset_server.load(PLAYER_BOTTOM_SPRITE_PATH),
                ..default()
            });
        });
}

pub fn player_movement(
    mut player_query: Query<&mut Transform, With<Player>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    let Ok(mut player_transform) = player_query.get_single_mut() else { return };

    let mut direction = Vec3::ZERO;

    if keyboard_input.pressed(KeyCode::ArrowLeft) || keyboard_input.pressed(KeyCode::KeyA) {
        direction.x += -1.0;
    }
    if keyboard_input.pressed(KeyCode::ArrowRight) || keyboard_input.pressed(KeyCode::KeyD) {
        direction.x += 1.0;
    }

    player_transform.translation += direction * PLAYER_SPEED * time.delta_seconds();
}