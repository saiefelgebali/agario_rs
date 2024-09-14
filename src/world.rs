use bevy::prelude::*;
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};

use crate::materials::grid::GridMaterial;
use crate::WORLD_SIZE;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_world_system);
    }
}

// Setup a simple 2d scene
fn setup_world_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<GridMaterial>>,
) {
    commands.spawn(MaterialMesh2dBundle {
        mesh: Mesh2dHandle(meshes.add(Rectangle::new(WORLD_SIZE * 2.0, WORLD_SIZE * 2.0))),
        transform: Transform::from_xyz(0.0, 0.0, -100.0),
        material: materials.add(GridMaterial {}),
        ..default()
    });
}
