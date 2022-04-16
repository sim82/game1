use bevy::prelude::*;
use big_brain::prelude::*;

use crate::{
    movement::crab_move::{CrabMoveDirection, CrabMoveWalker},
    path::{Waypoint, WaypointPath},
};

#[derive(Clone, Debug, Component, Default)]
pub struct FollowPath {
    progress: usize,
}

pub fn follow_path_action_system(
    mut commands: Commands,
    mut walkers: Query<(&Transform, &mut CrabMoveWalker, &WaypointPath)>,
    // We execute actions by querying for their associated Action Component
    // (Drink in this case). You'll always need both Actor and ActionState.
    mut query: Query<(Entity, &Actor, &mut ActionState, &mut FollowPath)>,
    waypoint_query: Query<&Transform, With<Waypoint>>,
) {
    for (action_entity, Actor(entity), mut action_state, mut follow_path) in query.iter_mut() {
        // commands
        if let Ok((Transform { translation, .. }, mut crab_move_walker, waypoint_path)) =
            walkers.get_mut(*entity)
        {
            match *action_state {
                ActionState::Requested => {
                    let start_point = waypoint_path
                        .waypoints
                        .iter()
                        .enumerate()
                        .map(|(i, wp_entity)| {
                            let Transform {
                                translation: wp_pos,
                                ..
                            } = waypoint_query.get(*wp_entity).unwrap();
                            (i, (*translation - *wp_pos).length())
                        })
                        .min_by(|(_, l), (_, r)| l.partial_cmp(r).unwrap())
                        .unwrap()
                        .0;
                    info!("follow path started at {}", start_point);
                    follow_path.progress = start_point;
                    *action_state = ActionState::Executing
                }
                ActionState::Executing => {
                    let waypoints = &waypoint_path.waypoints;
                    if follow_path.progress >= waypoints.len() {
                        *action_state = ActionState::Success;
                    } else {
                        let min_dist = 6.0;
                        if let Ok(Transform {
                            translation: waypoint_translation,
                            ..
                        }) = waypoint_query.get(waypoints[follow_path.progress])
                        {
                            let d = *waypoint_translation - *translation;
                            let tv = d.normalize();
                            crab_move_walker.direction = CrabMoveDirection::find_nearest(tv);
                            info!(
                                "follow path progress: {} {} {:?}",
                                follow_path.progress,
                                d.length(),
                                waypoint_translation
                            );

                            if d.length() < min_dist {
                                follow_path.progress += 1;
                            }
                        }
                    }
                }
                ActionState::Cancelled => *action_state = ActionState::Failure,
                _ => (),
            }
        }
    }
}
