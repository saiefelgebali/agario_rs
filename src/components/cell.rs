use bevy::prelude::*;
use rand::{thread_rng, Rng};
use std::ops::Range;

#[derive(Default, Component, Clone)]
pub struct Cell {
    pub size: f32,
    pub value: f32,
}

impl Cell {
    pub fn new(value: f32) -> Cell {
        Cell { size: value, value }
    }

    pub fn rand_range(range: Range<f32>) -> Cell {
        let mut rand_gen = thread_rng();
        let size = rand_gen.gen_range(range);
        Cell::new(size)
    }
}
