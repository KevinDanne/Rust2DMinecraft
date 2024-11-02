use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_rapier2d::prelude::*;
use crate::game::block::components::Block;
use crate::game::constants::BLOCK_SIZE;
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
            RigidBody::Dynamic,
            LockedAxes::ROTATION_LOCKED,
            Player {},
        )
    )
        .with_children(|parent| {
            parent.spawn(
                (
                    SpriteBundle {
                        transform: Transform::from_xyz(PLAYER_TOP_TRANSFORM_OFFSET.x, PLAYER_TOP_TRANSFORM_OFFSET.y, PLAYER_TOP_TRANSFORM_OFFSET.z),
                        texture: asset_server.load(PLAYER_TOP_SPRITE_PATH),
                        ..default()
                    },
                    Collider::cuboid(BLOCK_SIZE / 2.0, BLOCK_SIZE / 2.0),
                )
            );
            parent.spawn(
                (
                    SpriteBundle {
                        transform: Transform::from_xyz(PLAYER_BOTTOM_TRANSFORM_OFFSET.x, PLAYER_BOTTOM_TRANSFORM_OFFSET.y, PLAYER_BOTTOM_TRANSFORM_OFFSET.z),
                        texture: asset_server.load(PLAYER_BOTTOM_SPRITE_PATH),
                        ..default()
                    },
                    Collider::cuboid(BLOCK_SIZE / 2.0, BLOCK_SIZE / 2.0),
                )
            );
        });
}

pub fn player_movement(
    mut player_query: Query<&mut Transform, With<Player>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    let Ok(mut player_transform) = player_query.get_single_mut() else { return; };

    let mut direction = Vec3::ZERO;

    if keyboard_input.pressed(KeyCode::ArrowLeft) || keyboard_input.pressed(KeyCode::KeyA) {
        direction.x += -PLAYER_SPEED * time.delta_seconds();
    }
    if keyboard_input.pressed(KeyCode::ArrowRight) || keyboard_input.pressed(KeyCode::KeyD) {
        direction.x += PLAYER_SPEED * time.delta_seconds();
    }
    if keyboard_input.just_pressed(KeyCode::Space) {
        direction.y += PLAYER_JUMP_FORCE;
    }

    player_transform.translation += direction;
}

pub fn move_camera(
    mut camera_query: Query<&mut Transform, (With<Camera2d>, Without<Player>)>,
    player_query: Query<&Transform, (With<Player>, Without<Camera2d>)>,
) {
    let Ok(mut camera_transform) = camera_query.get_single_mut() else { return; };
    let Ok(player_transform) = player_query.get_single() else { return; };

    camera_transform.translation = Vec3::new(player_transform.translation.x, BLOCK_SIZE, 0.0)
}

pub fn block_mining(
    mut commands: Commands,
    mouse_input: Res<ButtonInput<MouseButton>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    block_query: Query<(Entity, &Transform), With<Block>>
) {
    let Ok(window) = window_query.get_single() else { return; };
    let Ok((camera, camera_global_transform)) = camera_query.get_single() else { return; };

    let Some(cursor_position) = window.cursor_position() else { return; };
    if !mouse_input.pressed(MouseButton::Left) {
        return;
    }

    let Some(mut world_pos) = camera.viewport_to_world_2d(camera_global_transform, cursor_position) else { return; };
    world_pos.x = (world_pos.x / BLOCK_SIZE).round() * BLOCK_SIZE;
    world_pos.y = (world_pos.y / BLOCK_SIZE).round() * BLOCK_SIZE;

    for (block_entity, block_transform) in block_query.iter() {
        let block_pos = Vec2::new(block_transform.translation.x, block_transform.translation.y);
        if block_pos == world_pos {
            commands.entity(block_entity).despawn();
        }
    }
}