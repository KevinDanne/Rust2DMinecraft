use bevy::prelude::*;
use crate::game::constants::BLOCK_SIZE;

pub const PLAYER_SPEED: f32 = BLOCK_SIZE * 3.0;
pub const PLAYER_JUMP_FORCE: f32 = BLOCK_SIZE * 1.5;
pub const PLAYER_SPAWN_POSITION: Vec3 = Vec3::new(0.0, BLOCK_SIZE, 0.0);
pub const PLAYER_TOP_SPRITE_PATH: &str = "sprites/characters/player/player_top.png";
pub const PLAYER_BOTTOM_SPRITE_PATH: &str = "sprites/characters/player/player_bottom.png";

pub const PLAYER_TOP_TRANSFORM_OFFSET: Vec3 = Vec3::new(0.0, BLOCK_SIZE, 0.0);
pub const PLAYER_BOTTOM_TRANSFORM_OFFSET: Vec3 = Vec3::new(0.0, 0.0, 0.0);
