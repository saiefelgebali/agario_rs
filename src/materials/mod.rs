use bevy::{app::Plugin, sprite::Material2dPlugin};
use cell::CellMaterial;
use grid::GridMaterial;

pub mod cell;
pub mod grid;

pub struct MaterialsPlugin;

impl Plugin for MaterialsPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugins(Material2dPlugin::<CellMaterial>::default());
        app.add_plugins(Material2dPlugin::<GridMaterial>::default());
    }
}
