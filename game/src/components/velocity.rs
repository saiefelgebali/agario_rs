use bevy::prelude::*;
use std::ops::{Deref, DerefMut};

#[derive(Default, Clone, Component)]
pub struct Velocity(Vec2);

impl Deref for Velocity {
    type Target = Vec2;

    fn deref(&self) -> &Self::Target {
        return &self.0;
    }
}

impl DerefMut for Velocity {
    fn deref_mut(&mut self) -> &mut Self::Target {
        return &mut self.0;
    }
}
