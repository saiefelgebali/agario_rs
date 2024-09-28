use crate::prelude::*;
use bevy::prelude::*;

pub struct VelocityPlugin;

impl Plugin for VelocityPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, velocity_system);
    }
}

fn velocity_system(mut query: Query<(&Velocity, &mut Transform)>) {
    for (velocity, mut transform) in query.iter_mut() {
        let translation = &mut transform.translation;
        let vel = velocity;

        translation.x += vel.x * TIME_STEP * BASE_SPEED;
        translation.y += vel.y * TIME_STEP * BASE_SPEED;
    }
}
