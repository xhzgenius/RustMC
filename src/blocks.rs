use bevy::prelude::*;

#[derive(Component)]
pub struct Block;

#[derive(Component)]
struct BlockPosition(Vec3);

#[derive(Component)]
struct BlockRotation(Vec3);

#[derive(Bundle)]
pub struct BlockStatus {
    position: BlockPosition,
    rotation: BlockRotation,
}
