use bevy::prelude::*;

pub mod crab_move;
pub mod walk;
pub mod zap;

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(crab_move::apply_velocity_system)
            .add_system(walk::apply_velocity_system)
            .add_system(zap::check_pew_intersection_system);
    }
}
