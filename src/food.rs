use crate::{
    components::{Food, Player, Size},
    events::EatFoodEvent,
    materials::cell::CellMaterial,
    WORLD_SIZE,
};
use bevy::{
    math::bounding::{Aabb2d, BoundingVolume, IntersectsVolume},
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};
use rand::{thread_rng, Rng};

pub struct FoodPlugin;

impl Plugin for FoodPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, setup_food_system);
        app.add_systems(FixedUpdate, check_food_collision_system);
    }
}

fn setup_food_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<CellMaterial>>,
) {
    for _ in 0..20000 {
        let cell_overflow_radius = 0.05;
        let mesh = Mesh::from(Circle::new(0.5 + cell_overflow_radius));
        let normalized_cell_overflow_radius = cell_overflow_radius / 0.5;

        commands
            .spawn(MaterialMesh2dBundle {
                mesh: Mesh2dHandle(meshes.add(mesh)),
                transform: Transform {
                    translation: random_position(),
                    ..default()
                },
                material: materials.add(CellMaterial {
                    normalized_cell_overflow_radius,
                    color: random_color(),
                }),
                ..default()
            })
            .insert(Food)
            .insert(Size::rand_range(40.0..50.0));
    }
}

fn check_food_collision_system(
    mut commands: Commands,
    player_query: Query<&Transform, With<Player>>,
    food_query: Query<(Entity, &Transform, &Size), With<Food>>,
    mut eat_food_event: EventWriter<EatFoodEvent>,
) {
    for player_transform in player_query.iter() {
        let player_box = Aabb2d::new(
            player_transform.translation.truncate(),
            player_transform.scale.truncate() / 2.0,
        );
        for (food_entity, food_transform, food_size) in food_query.iter() {
            let food_box = Aabb2d::new(
                food_transform.translation.truncate(),
                food_transform.scale.truncate() / 2.0,
            );
            if let Some(_) = food_collision(food_box, player_box) {
                commands.entity(food_entity).despawn();
                eat_food_event.send(EatFoodEvent::new(**food_size));
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

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum Collision {
    Left,
    Right,
    Top,
    Bottom,
}

fn food_collision(food_box: Aabb2d, player_box: Aabb2d) -> Option<Collision> {
    if !food_box.intersects(&player_box) {
        return None;
    }

    let closest = player_box.closest_point(food_box.center());
    let offset = food_box.center() - closest;
    let side = if offset.x.abs() > offset.y.abs() {
        if offset.x < 0. {
            Collision::Left
        } else {
            Collision::Right
        }
    } else if offset.y > 0. {
        Collision::Top
    } else {
        Collision::Bottom
    };

    Some(side)
}
