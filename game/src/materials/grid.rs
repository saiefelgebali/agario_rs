use bevy::prelude::*;
use bevy::{
    render::render_resource::{AsBindGroup, ShaderRef},
    sprite::Material2d,
};

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct GridMaterial {}

impl GridMaterial {
    pub fn new() -> GridMaterial {
        GridMaterial {}
    }
}

impl Material2d for GridMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/grid.wgsl".into()
    }
}
