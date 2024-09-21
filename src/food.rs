use crate::{
    components::{Despawn, Food, Player, Size},
    events::EatFoodEvent,
    materials::cell::CellMaterial,
    WORLD_SIZE,
};
use bevy::{
    math::bounding::{BoundingCircle, BoundingVolume, IntersectsVolume},
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};
use rand::{thread_rng, Rng};

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
        let mesh = Mesh::from(Circle::new(0.5));

        commands
            .spawn(MaterialMesh2dBundle {
                mesh: Mesh2dHandle(meshes.add(mesh)),
                transform: Transform {
                    translation: random_position(),
                    ..default()
                },
                material: materials.add(CellMaterial {
                    color: random_color(),
                    colliders: Vec::new(),
                }),
                ..default()
            })
            .insert(Food)
            .insert(Size::rand_range(40.0..50.0));
    }
}

fn check_food_despawn(
    mut commands: Commands,
    mut eat_food_event: EventWriter<EatFoodEvent>,
    mut food_query: Query<
        (
            Entity,
            &mut Size,
            &Despawn,
            &Handle<CellMaterial>,
            &mut Transform,
        ),
        With<Food>,
    >,
    player_query: Query<&Transform, (With<Player>, Without<Food>)>,
    mut materials: ResMut<Assets<CellMaterial>>,
) {
    let player_transform = player_query.single();

    for (entity, mut size, despawn, material_handle, mut food_transform) in food_query.iter_mut() {
        // let material = materials.get_mut(material_handle).unwrap();
        // material.color = LinearRgba::new(0.0, 0.0, 0.0, 0.0);

        let new_position = player_transform.translation
            + Vec3::new(
                despawn.offset_from_player.x * player_transform.scale.x / 2.0,
                despawn.offset_from_player.y * -player_transform.scale.x / 2.0,
                0.0,
            );

        food_transform.translation.x = new_position.x;
        food_transform.translation.y = new_position.y;

        size.0 -= 3.;

        if size.0 <= 0.0 {
            commands.entity(entity).despawn();
            eat_food_event.send(EatFoodEvent::new(**size));
        }
    }
}

fn check_food_collision_system(
    mut commands: Commands,
    player_query: Query<&Transform, With<Player>>,
    mut food_query: Query<(Entity, &Transform, &Size, Option<&Despawn>), With<Food>>,
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
        for (food_entity, food_transform, _food_size, maybe_despawn) in food_query.iter_mut() {
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

fn random_position() -> Vec3 {
    let lower_bound = WORLD_SIZE * -1.0;
    let upper_bound = WORLD_SIZE;

    let mut rand_gen = thread_rng();
    let rand_x = rand_gen.gen_range(lower_bound..upper_bound);
    let rand_y = rand_gen.gen_range(lower_bound..upper_bound);
    let rand_z = rand_gen.gen_range(-50.0..-20.0);

    Vec3::new(rand_x, rand_y, rand_z)
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
