use bevy::prelude::*;

const TIME_STEP: f32 = 1.0 / 60.0;

pub struct EntityUpdatePlugin;

impl Plugin for EntityUpdatePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(FixedTime::new_from_secs(TIME_STEP));
        app.add_system(update_entity);
    }
}

#[derive(Component)]
pub struct Entity;

#[derive(Component)]
pub struct EntityStatus {
    pub health: i32,
    pub velocity: Vec3,
}

fn update_entity(
    mut query_entity_status: Query<(&mut EntityStatus, &mut Transform), With<Entity>>,
) {
    for (status, mut transform) in query_entity_status.iter_mut() {
        let local_y = transform.up().y;
        let movement = (transform.forward() * status.velocity.x + transform.right() * status.velocity.z)
            * TIME_STEP * Vec3::new(1., 0., 1.) / local_y;
        transform.translation += movement;
        println!("{:?}", movement)
    }
}
