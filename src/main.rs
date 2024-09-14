use bevy::prelude::*;
use events::EventsPlugin;
use food::FoodPlugin;
use materials::MaterialsPlugin;
use player::PlayerPlugin;
use size::SizePlugin;
use world::WorldPlugin;

mod components;
mod events;
mod food;
mod materials;
mod player;
mod size;
mod world;

const TIME_STEP: f32 = 1.0 / 60.0;
const BASE_SPEED: f32 = 100.0;
const WORLD_SIZE: f32 = 20000.0;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::srgb(0.0, 0.0, 0.0)))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: (800.0, 800.0).into(),
                title: "agar.io".to_string(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(MaterialsPlugin)
        .add_plugins(WorldPlugin)
        .add_plugins(PlayerPlugin)
        .add_plugins(SizePlugin)
        .add_plugins(FoodPlugin)
        .add_plugins(EventsPlugin)
        .add_systems(Startup, setup_system)
        .run();
}

fn setup_system(mut commands: Commands) {
    commands.spawn(Camera2dBundle {
        projection: OrthographicProjection {
            scale: 1.0,
            ..default()
        },
        ..default()
    });
}
