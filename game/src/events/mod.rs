use bevy::prelude::*;

pub struct EventsPlugin;

impl Plugin for EventsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<EatFoodEvent>();
    }
}

#[derive(Event)]
pub struct EatFoodEvent {
    pub food_value: f32,
}

impl EatFoodEvent {
    pub fn new(food_value: f32) -> EatFoodEvent {
        EatFoodEvent { food_value }
    }
}
