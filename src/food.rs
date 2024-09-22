use crate::prelude::*;
use bevy::math::bounding::{BoundingCircle, BoundingVolume, IntersectsVolume};
use bevy::prelude::*;
use rand::*;

pub struct FoodPlugin;

impl Plugin for FoodPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, setup_food_system);
        app.add_systems(Update, check_food_despawn);
        app.add_systems(FixedUpdate, check_food_collision_system);
    }
}

fn setup_food_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<CellMaterial>>,
) {
    for _ in 0..20000 {
        commands
            .spawn(CellBundle {
                mesh: CellBundle::add_cell_mesh(&mut meshes),
                transform: Transform::from_translation(random_position()),
                material: materials.add(CellMaterial::new(random_color())),
                cell: Cell::rand_range(40.0..50.0),
                ..default()
            })
            .insert(Food);
    }
}

fn check_food_despawn(
    mut commands: Commands,
    mut eat_food_event: EventWriter<EatFoodEvent>,
    mut food_query: Query<(Entity, &Despawn, &mut Cell, &mut Transform), With<Food>>,
    player_query: Query<&Transform, (With<Player>, Without<Food>)>,
) {
    let player_transform = player_query.single();

    for (entity, despawn, mut cell, mut transform) in food_query.iter_mut() {
        let offset = despawn.offset_from_player
            * Vec2::new(
                player_transform.scale.x / 2.0,
                -player_transform.scale.x / 2.0,
            );

        transform.translation.x = player_transform.translation.x + offset.x;
        transform.translation.y = player_transform.translation.y + offset.y;
        cell.size -= 3.0;

        if cell.size <= 0.0 {
            commands.entity(entity).despawn();
            eat_food_event.send(EatFoodEvent::new(cell.value / 20.0));
        }
    }
}

fn check_food_collision_system(
    mut commands: Commands,
    player_query: Query<&Transform, With<Player>>,
    mut food_query: Query<(Entity, &Transform, Option<&Despawn>), With<Food>>,
    mut materials: ResMut<Assets<CellMaterial>>,
    handle: Query<&Handle<CellMaterial>, With<Player>>,
) {
    let player_material_handle = handle.single();
    let player_material = materials.get_mut(player_material_handle).unwrap();

    player_material.colliders = vec![Vec4::new(0.0, 0.0, 0.0, 0.0)];

    for player_transform in player_query.iter() {
        let player_box = BoundingCircle::new(
            player_transform.translation.truncate(),
            player_transform.scale.x / 2.0,
        );
        for (food_entity, food_transform, maybe_despawn) in food_query.iter_mut() {
            let food_box = BoundingCircle::new(
                food_transform.translation.truncate(),
                food_transform.scale.x / 2.0,
            );
            if let Some(offset) = food_collision(food_box, player_box) {
                if offset.xy().length() < 1.0 && maybe_despawn.is_none() {
                    commands
                        .entity(food_entity)
                        .insert(Despawn::new(offset.xy()));
                    player_material
                        .colliders
                        .push(Vec4::new(offset.x, offset.y, offset.z, 0.0));
                } else {
                    player_material
                        .colliders
                        .push(Vec4::new(offset.x, offset.y, offset.z, 0.0));
                }
            }
        }
    }
}

fn random_color() -> LinearRgba {
    let mut rand_gen = thread_rng();
    let rand_red = rand_gen.gen_range(0.2..0.8);
    let rand_green = rand_gen.gen_range(0.2..0.8);
    let rand_blue = rand_gen.gen_range(0.2..0.8);
    return LinearRgba::new(rand_red, rand_green, rand_blue, 1.0);
}

fn food_collision(food_box: BoundingCircle, player_box: BoundingCircle) -> Option<Vec3> {
    if !food_box.grow(10.0).intersects(&player_box) {
        return None;
    }

    let raw_offset = food_box.center() - player_box.center();
    let normalized_offset = raw_offset / player_box.radius();
    let normalized_offset = normalized_offset * Vec2::new(1.0, -1.0);

    let food_radius = food_box.radius() / player_box.radius();

    Some(Vec3::new(
        normalized_offset.x,
        normalized_offset.y,
        food_radius,
    ))
}
