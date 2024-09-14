use bevy::prelude::*;
use crate::components::Size;

pub struct SizePlugin;

impl Plugin for SizePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(FixedUpdate, sync_size_system);
    }
}

fn sync_size_system(
    mut query: Query<(&mut Transform, &Size)>
) {
    for (mut transform, size) in &mut query {
        let scale = **size as f32;
        transform.scale = Vec3::new(scale, scale, scale);
    }
}

