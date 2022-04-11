use bevy::prelude::*;
use big_brain::prelude::*;

use crate::path::WaypointPath;

#[derive(Component, Debug, Clone, Default)]
pub struct CanFollowPath;

pub fn can_follow_path_scorer_system(
    waypoint_path_query: Query<&WaypointPath>,
    // Same dance with the Actor here, but now we use look up Score instead of ActionState.
    mut query: Query<(&Actor, &mut Score), With<CanFollowPath>>,
) {
    // info!("fear scorer {:?}", std::thread::current());

    for (Actor(actor), mut score) in query.iter_mut() {
        if let Ok(_waypoint_path) = waypoint_path_query.get(*actor) {
            // info!("fear: {}", fear.fear);
            score.set(1.0);
        } else {
            score.set(0.0);
        }
    }
}
