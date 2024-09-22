use crate::prelude::*;
use bevy::prelude::*;
use bevy::sprite::Mesh2dHandle;

pub struct CellPlugin;

impl Plugin for CellPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, sync_cell_size);
    }
}

fn sync_cell_size(mut query: Query<(&mut Transform, &Cell)>) {
    for (mut transform, cell) in &mut query {
        transform.scale = Vec3::new(cell.size, cell.size, cell.size);
    }
}

/// Fork of [bevy::sprite::MaterialMesh2dBundle]
///
/// Should contain basic components for a game cell entity to function
#[derive(Bundle, Clone)]
pub struct CellBundle {
    pub cell: Cell,
    pub velocity: Velocity,
    pub mesh: Mesh2dHandle,
    pub material: Handle<CellMaterial>,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    // User indication of whether an entity is visible
    pub visibility: Visibility,
    // Inherited visibility of an entity.
    pub inherited_visibility: InheritedVisibility,
    // Indication of whether an entity is visible in any view.
    pub view_visibility: ViewVisibility,
}

impl CellBundle {
    pub fn add_cell_mesh(meshes: &mut ResMut<Assets<Mesh>>) -> Mesh2dHandle {
        Mesh2dHandle(meshes.add(Mesh::from(Circle::default())))
    }
}

impl Default for CellBundle {
    fn default() -> Self {
        Self {
            cell: Default::default(),
            velocity: Default::default(),
            mesh: Default::default(),
            material: Default::default(),
            transform: Default::default(),
            global_transform: Default::default(),
            visibility: Default::default(),
            inherited_visibility: Default::default(),
            view_visibility: Default::default(),
        }
    }
}
