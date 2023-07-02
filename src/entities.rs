use bevy::prelude::*;

#[derive(Component)]
pub struct Entity;

#[derive(Component)]
struct EntityHealth(i32);

#[derive(Component)]
struct EntityPosition(Vec3);

#[derive(Component)]
struct EntityRotation(Vec3);

#[derive(Component)]
struct EntityVelocity(Vec3);

#[derive(Bundle)]
pub struct EntityStatus {
    health: EntityHealth,
    position: EntityPosition,
    rotation: EntityRotation,
    velocity: EntityVelocity,
}
