use bevy::prelude::*;

#[derive(Component)]
pub struct Grow {
    pub ideal_size: f32,
}

impl Grow {
    pub fn new(ideal_size: f32) -> Grow {
        Grow { ideal_size }
    }
}
