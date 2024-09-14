use std::ops::{Deref, DerefMut, Range};

use bevy::{
    math::Vec2,
    prelude::{Component, Deref, DerefMut},
};
use rand::{thread_rng, Rng};

#[derive(Component)]
pub struct Velocity(Vec2);

impl Velocity {
    pub fn new() -> Velocity {
        Velocity(Vec2::new(0.0, 0.0))
    }
}

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

#[derive(Component)]
pub struct Player;

#[derive(Deref, DerefMut, Component, Clone)]
pub struct Size(f32);

impl Size {
    pub fn new(value: f32) -> Size {
        Size(value)
    }

    pub fn rand_range(range: Range<f32>) -> Size {
        let mut rand_gen = thread_rng();
        let size = rand_gen.gen_range(range);
        Size(size)
    }
}

#[derive(Component)]
pub struct Food;
