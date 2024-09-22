use bevy::prelude::*;
use bevy::{
    render::render_resource::{AsBindGroup, ShaderRef},
    sprite::Material2d,
};

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct CellMaterial {
    #[uniform(1)]
    pub color: LinearRgba,

    #[storage(2, read_only)]
    pub colliders: Vec<Vec4>,
}

impl CellMaterial {
    pub fn new(color: LinearRgba) -> CellMaterial {
        CellMaterial {
            color,
            colliders: vec![],
        }
    }
}

impl Material2d for CellMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/cell.wgsl".into()
    }
}
