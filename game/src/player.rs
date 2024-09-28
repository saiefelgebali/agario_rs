use crate::prelude::*;
use crate::world::BackgroundGrid;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, player_spawn_system);
        app.add_systems(PreUpdate, player_mouse_input_system);
        app.add_systems(
            Update,
            (
                sync_camera_with_player_system,
                sync_camera_with_player_size_system,
            ),
        );
    }
}

fn player_spawn_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<CellMaterial>>,
) {
    let player_color = LinearRgba::new(0.2, 0.8, 0.1, 1.0);

    commands
        .spawn(CellBundle {
            mesh: CellBundle::add_cell_mesh(&mut meshes),
            transform: Transform::from_xyz(0.0, 0.0, -10.0),
            material: materials.add(CellMaterial::new(player_color)),
            cell: Cell::new(PLAYER_SIZE),
            ..default()
        })
        .insert(Velocity::default())
        .insert(Consumer)
        .insert(Player);
}

#[allow(unused)]
fn player_keyboard_input_system(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Velocity, With<Player>>,
) {
    for mut velocity in &mut query {
        if keyboard.pressed(KeyCode::KeyD) {
            velocity.x = 1.0;
        } else if keyboard.pressed(KeyCode::KeyA) {
            velocity.x = -1.0;
        } else {
            velocity.x = 0.0;
        }

        if keyboard.pressed(KeyCode::KeyW) {
            velocity.y = 1.0;
        } else if keyboard.pressed(KeyCode::KeyS) {
            velocity.y = -1.0;
        } else {
            velocity.y = 0.0;
        }
    }
}

fn player_mouse_input_system(
    q_windows: Query<&Window, With<PrimaryWindow>>,
    mut query: Query<&mut Velocity, With<Player>>,
) {
    let window = q_windows.single();
    let window_center = window.size().div_euclid(Vec2::new(2.0, 2.0));

    if let Some(position) = window.cursor_position() {
        let mouse_vector = position - window_center;
        let mouse_vector =
            mouse_vector.clamp_length_max(MOUSE_UNIT_VECTOR_SCALE) / MOUSE_UNIT_VECTOR_SCALE;

        for mut velocity in &mut query {
            velocity.x = mouse_vector.x;
            velocity.y = -mouse_vector.y;
        }
    }
}

fn sync_camera_with_player_system(
    player_transform: Query<&Transform, With<Player>>,
    mut grid_transform: Query<
        &mut Transform,
        (With<BackgroundGrid>, Without<Player>, Without<Camera>),
    >,
    mut camera_transform: Query<
        &mut Transform,
        (With<Camera>, Without<Player>, Without<BackgroundGrid>),
    >,
    mut materials: ResMut<Assets<GridMaterial>>,
    handle: Query<&Handle<GridMaterial>, With<BackgroundGrid>>,
) {
    let grid_material = materials.get_mut(handle.single()).unwrap();

    let player_transform = player_transform.single();
    let mut camera_transform = camera_transform.single_mut();
    let mut grid_transform = grid_transform.single_mut();

    camera_transform.translation.x = player_transform.translation.x;
    camera_transform.translation.y = player_transform.translation.y;

    grid_transform.translation.x = player_transform.translation.x;
    grid_transform.translation.y = player_transform.translation.y;

    grid_material.offset = player_transform.translation.xy();
}

fn sync_camera_with_player_size_system(
    player_cell: Query<&Cell, With<Player>>,
    mut camera_query: Query<&mut OrthographicProjection, With<Camera>>,
) {
    let player_cell = player_cell.single();
    for mut camera_projection in camera_query.iter_mut() {
        // camera_projection.scale = 0.5 + ((player_cell.size / PLAYER_SIZE) - 1.0);
    }
}
