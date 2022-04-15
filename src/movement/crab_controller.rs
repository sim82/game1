use crate::{
    movement::{
        controller::MovementPriority,
        crab_move::{CrabMoveDirection, CrabMoveWalker},
    },
    path::{Waypoint, WaypointPath},
};

use bevy::prelude::*;
use rand::Rng;

mod tune {
    pub const CHANGE_DIRECTION_TIMEOUT: f32 = 0.5;
}

#[derive(Component, Default)]
#[component(storage = "SparseSet")]

pub struct CrabFollowPath {
    next_step: usize,
}

#[derive(Component, Default)]
#[component(storage = "SparseSet")]
pub struct CrabEvade {
    time_left_before_change: f32,
}

pub fn crab_override_direction_system(
    time: Res<Time>,
    mut query: Query<(&mut CrabMoveWalker, &mut CrabEvade)>,
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
        Without<CrabEvade>,
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
        // let translation = transform.translation;
        // if started {
        //     let start_point = path
        //         .waypoints
        //         .iter()
        //         .enumerate()
        //         .map(|(i, wp_entity)| {
        //             let Transform {
        //                 translation: wp_pos,
        //                 ..
        //             } = waypoint_query.get(*wp_entity).unwrap();
        //             (i, (translation - *wp_pos).length())
        //         })
        //         .min_by(|(_, l), (_, r)| l.partial_cmp(r).unwrap())
        //         .unwrap()
        //         .0;
        //     info!("follow path started at {}", start_point);
        //     *next = start_point;
        // }
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

// pub enum CrabMovement {
//     Directional(CrabMoveDirection),
//     FollowPath { next: usize, path: WaypointPath },
// }

// impl CrabMovement {
//     pub fn exec(
//         &mut self,
//         started: bool,
//         walker: &mut CrabMoveWalker,
//         transform: &Transform,
//         waypoint_query: &Query<&Transform, With<Waypoint>>,
//     ) -> bool {
//         match self {
//             CrabMovement::Directional(direction) => {
//                 walker.direction = *direction;
//                 false
//             }
//             CrabMovement::FollowPath { next, path } => {
//                 let translation = transform.translation;
//                 if started {
//                     let start_point = path
//                         .waypoints
//                         .iter()
//                         .enumerate()
//                         .map(|(i, wp_entity)| {
//                             let Transform {
//                                 translation: wp_pos,
//                                 ..
//                             } = waypoint_query.get(*wp_entity).unwrap();
//                             (i, (translation - *wp_pos).length())
//                         })
//                         .min_by(|(_, l), (_, r)| l.partial_cmp(r).unwrap())
//                         .unwrap()
//                         .0;
//                     info!("follow path started at {}", start_point);
//                     *next = start_point;
//                 }
//                 if *next >= path.waypoints.len() {
//                     return true;
//                 }
//                 let min_dist = 6.0;
//                 if let Ok(Transform {
//                     translation: waypoint_translation,
//                     ..
//                 }) = waypoint_query.get(path.waypoints[*next])
//                 {
//                     let d = *waypoint_translation - translation;
//                     let tv = d.normalize();
//                     walker.direction = CrabMoveDirection::find_nearest(tv);
//                     info!(
//                         "follow path progress: {} {} {:?}",
//                         next,
//                         d.length(),
//                         waypoint_translation
//                     );

//                     if d.length() < min_dist {
//                         *next += 1;
//                     }
//                 }
//                 false
//             }
//         }
//     }
// }

// #[derive(Component)]
// pub struct CrabController {
//     movements: Vec<(MovementPriority, Entity, CrabMovement)>,
//     active_movement: Option<Entity>,
// }

// impl CrabController {
//     pub fn add_movement(
//         &mut self,
//         priority: MovementPriority,
//         movement: CrabMovement,
//         owner: Entity,
//     ) {
//         self.movements.push((priority, owner, movement));
//     }
// }

// pub fn crab_controller_update_system(
//     mut query: Query<(&mut CrabController, &mut CrabMoveWalker, &Transform)>,
//     waypoints_query: Query<&Transform, With<Waypoint>>,
// ) {
//     for (mut controller, mut walker, transform) in query.iter_mut() {
//         controller.movements.sort_by_key(|m| m.0);
//         let next_movement = controller.movements.last().map(|(_, entity, _)| *entity);
//         let start = controller.active_movement != next_movement;

//         controller.active_movement = next_movement;

//         if let Some((_, _, movement)) = controller.movements.last_mut() {
//             movement.exec(start, &mut *walker, transform, &waypoints_query);
//         }
//     }
// }
