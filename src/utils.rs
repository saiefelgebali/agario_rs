use crate::prelude::*;
use bevy::prelude::*;
use rand::*;

pub fn random_position() -> Vec3 {
    let lower_bound = WORLD_SIZE * -1.0;
    let upper_bound = WORLD_SIZE;

    let mut rand_gen = thread_rng();
    let rand_x = rand_gen.gen_range(lower_bound..upper_bound);
    let rand_y = rand_gen.gen_range(lower_bound..upper_bound);
    let rand_z = rand_gen.gen_range(-50.0..-20.0);

    Vec3::new(rand_x, rand_y, rand_z)
}
