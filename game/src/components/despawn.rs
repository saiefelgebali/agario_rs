use bevy::prelude::*;

#[derive(Component)]
pub struct Despawn {
    pub offset_from_player: Vec2,
}

impl Despawn {
    pub fn new(offset: Vec2) -> Despawn {
        Despawn {
            offset_from_player: offset,
        }
    }
}
