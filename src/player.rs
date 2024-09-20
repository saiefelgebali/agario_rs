use crate::{
    components::{Player, Size, Velocity},
    events::EatFoodEvent,
    materials::cell::CellMaterial,
    BASE_SPEED, TIME_STEP,
};
use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};

pub struct PlayerPlugin;

const PLAYER_SIZE: f32 = 50.0;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, player_spawn_system);
        app.add_systems(
            Update,
            (
                (
                    player_keyboard_input_system,
                    player_move_sytem,
                    sync_camera_with_player_system,
                    sync_camera_with_player_size_system,
                )
                    .chain(),
                handle_eat_food_event
            ),
        );
    }
}

fn player_spawn_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<CellMaterial>>,
) {
    let cell_overflow_radius = 0.05;
    let mesh = Mesh::from(Circle::new(0.5 + cell_overflow_radius));

    let normalized_cell_overflow_radius = cell_overflow_radius / 0.5;

    commands
        .spawn(MaterialMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(mesh)),
            transform: Transform::from_xyz(0.0, 0.0, -10.0),
            material: materials.add(CellMaterial {
                normalized_cell_overflow_radius,
                color: LinearRgba::new(0.2, 0.8, 0.1, 1.0),
                colliders: Vec::new()
            }),
            ..default()
        })
        .insert(Player)
        .insert(Velocity::new())
        .insert(Size::new(PLAYER_SIZE));
}

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

fn player_move_sytem(mut query: Query<(&Velocity, &mut Transform), With<Player>>) {
    for (velocity, mut transform) in query.iter_mut() {
        let translation = &mut transform.translation;
        let vel = velocity;

        translation.x += vel.x * TIME_STEP * BASE_SPEED;
        translation.y += vel.y * TIME_STEP * BASE_SPEED;
    }
}

fn sync_camera_with_player_system(
    mut set: ParamSet<(
        Query<&Transform, With<Player>>,
        Query<&mut Transform, With<Camera>>,
    )>,
) {
    let player_transform = set.p0();
    let player_transform = player_transform.single();
    let player_x = player_transform.translation.x;
    let player_y = player_transform.translation.y;

    for mut camera_transform in set.p1().iter_mut() {
        camera_transform.translation.x = player_x;
        camera_transform.translation.y = player_y;
    }
}

fn sync_camera_with_player_size_system(
    player_query: Query<&Size, With<Player>>,
    mut camera_query: Query<&mut OrthographicProjection, With<Camera>>,
) {
    let player_size = player_query.single();
    for mut camera_projection in camera_query.iter_mut() {
        camera_projection.scale = **player_size / (PLAYER_SIZE * 1.5);
    }
}

fn handle_eat_food_event(
    mut eat_food_event: EventReader<EatFoodEvent>,
    mut query: Query<&mut Size, With<Player>>,
) {
    for event in eat_food_event.read() {
        for mut player_size in query.iter_mut() {
            let new_size = **player_size + (event.food_size / 50.0);
            *player_size = Size::new(new_size);
        }
    }
}
