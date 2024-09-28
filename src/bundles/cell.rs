use crate::prelude::*;
use bevy::prelude::*;
use bevy::sprite::Mesh2dHandle;

pub struct CellPlugin;

impl Plugin for CellPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (cell_growth_system, cell_size_system));
        app.add_systems(FixedUpdate, handle_eat_food_event);
    }
}

fn cell_size_system(mut query: Query<(&mut Transform, &Cell)>) {
    for (mut transform, cell) in &mut query {
        transform.scale = Vec3::new(cell.size, cell.size, cell.size);
    }
}

fn cell_growth_system(mut commands: Commands, mut query: Query<(Entity, &Grow, &mut Cell)>) {
    for (entity, grow, mut cell) in query.iter_mut() {
        cell.size += 0.1;
        if cell.size >= grow.ideal_size {
            cell.size = grow.ideal_size;
            commands.entity(entity).remove::<Grow>();
        }
    }
}

fn handle_eat_food_event(
    mut commands: Commands,
    mut eat_food_event: EventReader<EatFoodEvent>,
    query: Query<(Entity, &Cell), With<Player>>,
) {
    for event in eat_food_event.read() {
        for (entity, player_cell) in query.iter() {
            let new_size = player_cell.size + event.food_value;
            commands.entity(entity).insert(Grow::new(new_size));
        }
    }
}

/// Fork of [bevy::sprite::MaterialMesh2dBundle]
///
/// Should contain basic components for a game cell entity to function
#[derive(Bundle, Clone)]
pub struct CellBundle {
    pub cell: Cell,
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
