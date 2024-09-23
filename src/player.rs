use crate::prelude::*;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, player_spawn_system);
        app.add_systems(
            Update,
            (
                (
                    player_mouse_input_system,
                    player_move_sytem,
                    sync_camera_with_player_system,
                )
                    .chain(),
                (handle_eat_food_event, sync_camera_with_player_size_system).chain(),
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
        let mouse_vector = mouse_vector.clamp_length_max(5.0) / 5.0;

        for mut velocity in &mut query {
            velocity.x = mouse_vector.x;
            velocity.y = -mouse_vector.y;
        }
    }
}

fn player_move_sytem(mut query: Query<(&Velocity, &mut Transform), With<Player>>) {
    for (velocity, mut transform) in query.iter_mut() {
        let translation = &mut transform.translation;
        let vel = velocity;

        translation.x += vel.x * TIME_STEP * BASE_SPEED;
        translation.y += vel.y * TIME_STEP * BASE_SPEED;
    }
}

fn sync_camera_with_player_system(
    player_transform: Query<&Transform, With<Player>>,
    mut camera_transform: Query<&mut Transform, (With<Camera>, Without<Player>)>,
) {
    let player_transform = player_transform.single();

    for mut camera_transform in camera_transform.iter_mut() {
        camera_transform.translation.x = player_transform.translation.x;
        camera_transform.translation.y = player_transform.translation.y;
    }
}

fn sync_camera_with_player_size_system(
    player_cell: Query<&Cell, With<Player>>,
    mut camera_query: Query<&mut OrthographicProjection, With<Camera>>,
) {
    let player_cell = player_cell.single();
    for mut camera_projection in camera_query.iter_mut() {
        camera_projection.scale = 0.5 + ((player_cell.size / PLAYER_SIZE) - 1.0);
    }
}

fn handle_eat_food_event(
    mut commands: Commands,
    mut eat_food_event: EventReader<EatFoodEvent>,
    query: Query<(Entity, &Cell), With<Player>>,
) {
    for event in eat_food_event.read() {
        for (entity, player_cell) in query.iter() {
            let new_size = player_cell.size + event.food_value;
            commands.entity(entity).insert(Grow::new(new_size));
        }
    }
}
