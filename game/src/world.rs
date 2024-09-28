use crate::prelude::*;
use bevy::prelude::*;
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};
use bevy::window::{PrimaryWindow, WindowResized};

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_world_system);
        app.add_systems(FixedUpdate, resize_grid_system);
    }
}

#[derive(Component)]
pub struct BackgroundGrid;

fn setup_world_system(
    window: Query<&Window, With<PrimaryWindow>>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<GridMaterial>>,
) {
    let window = window.single();

    let mesh = Rectangle::new(window.width(), window.height());

    commands
        .spawn(MaterialMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(mesh)),
            transform: Transform::from_xyz(0.0, 0.0, -100.0),
            material: materials.add(GridMaterial::new()),
            ..default()
        })
        .insert(BackgroundGrid);
}

fn resize_grid_system(
    mut ev_window_resize: EventReader<WindowResized>,
    mut query: Query<&mut Mesh2dHandle, With<BackgroundGrid>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    let mut grid = query.single_mut();

    for ev in ev_window_resize.read() {
        grid.0 = meshes.add(Rectangle::new(ev.width, ev.height));
    }
}
