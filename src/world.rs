use crate::prelude::*;
use bevy::prelude::*;
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_world_system);
    }
}

fn setup_world_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<GridMaterial>>,
) {
    let mesh = Rectangle::new(WORLD_SIZE * 2.0, WORLD_SIZE * 2.0);

    commands.spawn(MaterialMesh2dBundle {
        mesh: Mesh2dHandle(meshes.add(mesh)),
        transform: Transform::from_xyz(0.0, 0.0, -100.0),
        material: materials.add(GridMaterial::new()),
        ..default()
    });
}
