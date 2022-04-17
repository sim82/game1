use crate::{
    movement::crab_move::{CrabMoveDirection, CrabMoveWalker},
    path::{PathQuery, Waypoint, WaypointPath},
};

use bevy::prelude::*;
use rand::Rng;

use super::control::{MovementEvade, MovementGoToPoint};

// implementation of the abstract movement controls for CrabMoveWalker

mod tune {
    pub const CHANGE_DIRECTION_TIMEOUT: f32 = 0.5;
}

#[derive(Component, Default)]
#[component(storage = "SparseSet")]

pub struct CrabFollowPath {
    pub next_step: usize,
}

pub fn crab_evade_system(
    time: Res<Time>,
    mut query: Query<(&mut CrabMoveWalker, &mut MovementEvade)>,
) {
    for (mut walker, mut dodge_pew) in query.iter_mut() {
        dodge_pew.time_left_before_change -= time.delta_seconds();

        if dodge_pew.time_left_before_change <= 0.0 {
            dodge_pew.time_left_before_change = tune::CHANGE_DIRECTION_TIMEOUT;
            // info!("dodging");

            walker.direction = match walker.direction {
                CrabMoveDirection::NorthWest => CrabMoveDirection::NorthEast,
                CrabMoveDirection::NorthEast => CrabMoveDirection::NorthWest,
                CrabMoveDirection::SouthEast => CrabMoveDirection::SouthWest,
                CrabMoveDirection::SouthWest => CrabMoveDirection::SouthEast,
                _ => {
                    let mut rng = rand::thread_rng();
                    let choices = [
                        CrabMoveDirection::NorthEast,
                        CrabMoveDirection::NorthWest,
                        CrabMoveDirection::SouthEast,
                        CrabMoveDirection::SouthWest,
                    ];
                    choices[rng.gen_range(0..choices.len())]
                }
            }
        }
    }
}

#[allow(clippy::type_complexity)]
pub fn crab_update_path_system(
    mut commands: Commands,
    query: Query<
        (Entity, &Transform, &MovementGoToPoint),
        (
            Without<CrabFollowPath>,
            Without<PathQuery>,
            With<CrabMoveWalker>,
        ),
    >,
) {
    for (
        entity,
        Transform {
            translation: start, ..
        },
        MovementGoToPoint(end),
    ) in query.iter()
    {
        info!("goto point witout path. update");
        commands
            .entity(entity)
            .insert(PathQuery {
                start: *start,
                end: *end,
            })
            .insert(CrabFollowPath::default());
    }
}

pub fn crab_follow_path_system(
    mut commands: Commands,
    mut query: Query<
        (
            Entity,
            &mut CrabMoveWalker,
            &mut CrabFollowPath,
            &WaypointPath,
            &Transform,
        ),
        Without<MovementEvade>,
    >,
    waypoint_query: Query<&Transform, With<Waypoint>>,
) {
    for (
        entity,
        mut walker,
        mut follow_path,
        WaypointPath { waypoints },
        Transform { translation, .. },
    ) in query.iter_mut()
    {
        if follow_path.next_step >= waypoints.len() {
            commands.entity(entity).remove::<CrabFollowPath>();
            continue;
        }
        let min_dist = 6.0;
        if let Ok(Transform {
            translation: waypoint_translation,
            ..
        }) = waypoint_query.get(waypoints[follow_path.next_step])
        {
            let d = *waypoint_translation - *translation;
            let tv = d.normalize();
            walker.direction = CrabMoveDirection::find_nearest(tv);
            debug!(
                "follow path progress: {} {} {:?}",
                follow_path.next_step,
                d.length(),
                waypoint_translation
            );

            if d.length() < min_dist {
                // info!("follow path next step: {}", follow_path.next_step);
                follow_path.next_step += 1;
            }
        }
    }
}
