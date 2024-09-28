use std::{io::Read, net::TcpListener};

use crate::prelude::*;
use bevy::{prelude::*, tasks::AsyncComputeTaskPool};

mod bundles;
mod components;
mod events;
mod food;
mod materials;
mod player;
mod prelude;
mod settings;
mod utils;
mod velocity;
mod world;

fn main() {
    let window_plugin = DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            resolution: (800.0, 800.0).into(),
            title: "agar.io".to_string(),
            ..default()
        }),
        ..default()
    });

    App::new()
        .insert_resource(ClearColor(Color::srgb(0.0, 0.0, 0.0)))
        .add_plugins(window_plugin)
        .add_plugins(MaterialsPlugin)
        .add_plugins(world::WorldPlugin)
        .add_plugins(player::PlayerPlugin)
        .add_plugins(food::FoodPlugin)
        .add_plugins(bundles::CellPlugin)
        .add_plugins(velocity::VelocityPlugin)
        .add_plugins(EventsPlugin)
        .add_systems(Startup, setup_system)
        .run();
}

fn setup_system(mut commands: Commands) {
    let thread_pool = AsyncComputeTaskPool::get();

    let task = thread_pool
        .spawn(async move {
            let client = TcpListener::bind("127.0.0.1:3000").unwrap();

            for stream in client.incoming() {
                let mut stream = stream.unwrap();

                let mut buf = String::new();

                let message = stream.read_to_string(&mut buf).unwrap();

                dbg!(message);
            }
        })
        .detach();

    dbg!(task);

    commands.spawn(Camera2dBundle {
        projection: OrthographicProjection {
            scale: 1.0,
            ..default()
        },
        ..default()
    });
}
