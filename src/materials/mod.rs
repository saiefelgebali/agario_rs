use bevy::{app::Plugin, sprite::Material2dPlugin};
pub use cell::CellMaterial;
pub use grid::GridMaterial;

mod cell;
mod grid;

pub struct MaterialsPlugin;

impl Plugin for MaterialsPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugins(Material2dPlugin::<CellMaterial>::default());
        app.add_plugins(Material2dPlugin::<GridMaterial>::default());
    }
}
