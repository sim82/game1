use bevy::prelude::*;

// the general idea is: those component are used for 'high level' or abstract movement control:
// * evade: try to do some evasive action (TODO: might need more fine grained info about what to evade.
//   currently it is easy as threats can only come horizontally, so vertical movement always works)
// * go to point: try to reach the point 'somehow'. How this is done (path finding, movement mechanics)
//   is up to lower levels.
//
// also it is a good idea to have some sensible priority ordering of the movements, e.g. evasion should
// override go to point.

// actual implementation strategies for those goals are currently in crab_controller.rs

#[derive(Component, Default)]
#[component(storage = "SparseSet")]
pub struct MovementEvade {
    pub time_left_before_change: f32,
}

#[derive(Component)]
#[component(storage = "SparseSet")]
pub struct MovementGoToPoint(pub Vec3);
