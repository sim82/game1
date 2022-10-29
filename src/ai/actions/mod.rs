use bevy::prelude::*;
use big_brain::prelude::*;

pub mod dodge_pew;
pub mod follow;
pub mod goto_medikit;
pub mod jiggle_around;
pub mod run_away;
pub mod shoot;

pub mod pick_goto_pos {
    use bevy::prelude::*;
    use big_brain::prelude::*;
    use rand::prelude::*;

    use crate::{item::medikit::Medikit, movement::control::MovementGoToPoint, path::Waypoint};

    use super::DebugAction;

    #[derive(Debug, Clone)]
    pub enum TargetPos {
        Random,
        Medikit,
    }

    #[derive(Component, Debug, Clone)]
    pub struct ActionPickGotoPos {
        target: TargetPos,
    }

    impl ActionPickGotoPos {
        pub fn new(target: TargetPos) -> Self {
            Self { target }
        }
    }
    pub fn action_pick_goto_pos_system(
        mut commands: Commands,
        mut query: Query<(&Actor, &mut ActionState, &mut ActionPickGotoPos)>,
        actor_query: Query<&Transform>,
        // go_to_point_query: Query<(), With<MovementGoToPoint>>,
        waypoint_query: Query<&Transform, With<Waypoint>>,
        medikit_query: Query<&Transform, With<Medikit>>,
    ) {
        for (Actor(actor), mut state, mut pick_goto_pos) in query.iter_mut() {
            info!("pick: {:?} {:?}", state, pick_goto_pos.target);
            let Transform {
                translation: actor_pos,
                ..
            } = actor_query.get(*actor).unwrap();

            match *state {
                // let mut nearest_dist
                ActionState::Requested => {
                    commands
                        .entity(*actor)
                        .insert(DebugAction::new("pick goto pos", state.clone()));

                    *state = ActionState::Executing;
                }
                ActionState::Executing => {
                    let best_pos = match pick_goto_pos.target {
                        TargetPos::Random => {
                            let cand_pos = waypoint_query.iter().collect::<Vec<_>>(); // FIXME: this is horrible!
                            let mut rng = thread_rng();
                            cand_pos[rng.gen_range(0..cand_pos.len())].translation
                        }
                        TargetPos::Medikit => {
                            let best_pos = medikit_query
                                .iter()
                                .map(
                                    |Transform {
                                         translation: medikit_pos,
                                         ..
                                     }| *medikit_pos,
                                )
                                .min_by_key(|medikit_pos| {
                                    (*medikit_pos - *actor_pos).length() as i32
                                });

                            if let Some(best_pos) = best_pos {
                                best_pos
                            } else {
                                // no medikit found -> go directly to failure
                                *state = ActionState::Failure;
                                continue;
                            }
                        }
                    };
                    commands.entity(*actor).insert(MovementGoToPoint(best_pos));

                    *state = ActionState::Success;
                }
                ActionState::Cancelled => {
                    *state = ActionState::Failure;
                }
                _ => {}
            }
        }
    }
}

pub mod goto_pos {
    use bevy::prelude::*;
    use big_brain::prelude::*;
    use rand::prelude::*;

    use crate::{
        movement::{
            control::MovementGoToPoint,
            crab_controller::CrabFollowPath,
            crab_move::{CrabMoveDirection, CrabMoveWalker},
        },
        path::{PathQuery, Waypoint, WaypointPath},
    };

    use super::DebugAction;

    #[derive(Component, Debug, Clone)]
    pub struct ActionGotoPos {
        next_step_timeout: Timer,
        next_step: usize,
        check_timeout: Timer,
    }

    impl Default for ActionGotoPos {
        fn default() -> Self {
            Self {
                next_step: 0,
                next_step_timeout: Timer::from_seconds(2.0, false),
                check_timeout: Timer::from_seconds(0.1, true),
            }
        }
    }
    pub fn action_goto_pos_system(
        mut commands: Commands,
        time: Res<Time>,
        mut query: Query<(&Actor, &mut ActionState, &mut ActionGotoPos)>,
        actor_query: Query<&Transform>,
        go_to_point_query: Query<&MovementGoToPoint>,
        mut path_query: Query<(&WaypointPath, &mut CrabMoveWalker)>,
        waypoint_query: Query<&Transform, With<Waypoint>>,
    ) {
        for (Actor(actor), mut state, mut goto_pos) in query.iter_mut() {
            // info!("goto pos: {:?}", state);
            let Transform {
                translation: actor_pos,
                ..
            } = actor_query.get(*actor).unwrap();

            match *state {
                // let mut nearest_dist
                ActionState::Requested => {
                    commands
                        .entity(*actor)
                        .insert(DebugAction::new("goto pos", state.clone()));

                    let goto_pos = go_to_point_query.get(*actor).unwrap();

                    commands.entity(*actor).insert(PathQuery {
                        start: *actor_pos,
                        end: goto_pos.0,
                    });
                    *state = ActionState::Executing
                }
                ActionState::Executing => {
                    if let Ok((WaypointPath { waypoints }, mut walker)) = path_query.get_mut(*actor)
                    {
                        if goto_pos.next_step >= waypoints.len() {
                            walker.direction = CrabMoveDirection::None;
                            commands.entity(*actor).remove::<MovementGoToPoint>();
                            commands.entity(*actor).remove::<WaypointPath>();
                            *state = ActionState::Success;
                            continue;
                        }

                        goto_pos.check_timeout.tick(time.delta());

                        if goto_pos.check_timeout.finished() {
                            let min_dist = 6.0;
                            if let Ok(Transform {
                                translation: waypoint_translation,
                                ..
                            }) = waypoint_query.get(waypoints[goto_pos.next_step])
                            {
                                let d = *waypoint_translation - *actor_pos;
                                let tv = d.normalize();
                                walker.direction = CrabMoveDirection::find_nearest(tv);
                                debug!(
                                    "follow path progress: {} {} {:?}",
                                    goto_pos.next_step,
                                    d.length(),
                                    waypoint_translation
                                );

                                if d.length() < min_dist {
                                    // info!("follow path next step: {}", follow_path.next_step);
                                    goto_pos.next_step += 1;
                                    goto_pos.next_step_timeout.reset();
                                }

                                goto_pos.next_step_timeout.tick(time.delta());
                                if goto_pos.next_step_timeout.finished() {
                                    warn!("timeout reaching next step -> failure");
                                    *state = ActionState::Failure;
                                    continue;
                                }
                            }
                        }
                    }

                    // if go_to_point_query.get(*actor).is_err() {
                    //     *state = ActionState::Success;
                    // }
                }
                ActionState::Cancelled => {
                    commands
                        .entity(*actor)
                        .remove::<MovementGoToPoint>()
                        .remove::<WaypointPath>();
                    *state = ActionState::Failure;
                }
                _ => {}
            }
        }
    }
}

pub mod wait {
    use std::ops::Range;

    use bevy::prelude::*;
    use big_brain::prelude::*;
    use rand::prelude::*;

    use crate::{
        movement::{control::MovementGoToPoint, crab_controller::CrabFollowPath},
        path::Waypoint,
    };

    use super::DebugAction;

    #[derive(Component, Debug, Clone)]
    pub struct ActionWait {
        timeout: Timer,
    }

    impl ActionWait {
        pub fn range(range: Range<f32>) -> Self {
            let mut rng = thread_rng();
            Self {
                timeout: Timer::from_seconds(rng.gen_range(range), false),
            }
        }
    }

    pub fn action_wait_system(
        mut commands: Commands,
        time: Res<Time>,
        mut query: Query<(&Actor, &mut ActionState, &mut ActionWait)>,
        actor_query: Query<&Transform>,
        go_to_point_query: Query<(), With<MovementGoToPoint>>,
    ) {
        for (Actor(actor), mut state, mut wait) in query.iter_mut() {
            // info!("wait: {:?}", state);

            match *state {
                ActionState::Requested => {
                    commands
                        .entity(*actor)
                        .insert(DebugAction::new("wait", state.clone()));
                    *state = ActionState::Executing;
                }
                ActionState::Executing => {
                    wait.timeout.tick(time.delta());
                    if wait.timeout.finished() {
                        *state = ActionState::Success;
                    }
                }
                ActionState::Cancelled => {
                    *state = ActionState::Failure;
                }
                _ => {}
            }
        }
    }
}

pub mod go_direction {
    use std::ops::Range;

    use bevy::prelude::*;
    use big_brain::prelude::*;
    use rand::prelude::*;

    use crate::{
        movement::{
            control::MovementGoToPoint,
            crab_controller::CrabFollowPath,
            crab_move::{CrabMoveDirection, CrabMoveWalker},
        },
        path::Waypoint,
    };

    use super::DebugAction;

    #[derive(Component, Debug, Clone)]
    pub struct ActionGoDirection {
        timeout: Timer,
        direction: CrabMoveDirection,
    }

    impl ActionGoDirection {
        pub fn new(duration: f32, direction: CrabMoveDirection) -> Self {
            Self {
                timeout: Timer::from_seconds(duration, false),
                direction,
            }
        }
    }

    pub fn action_go_direction_system(
        mut commands: Commands,
        time: Res<Time>,
        mut query: Query<(&Actor, &mut ActionState, &mut ActionGoDirection)>,
        mut walker_query: Query<&mut CrabMoveWalker>,
    ) {
        for (Actor(actor), mut state, mut go_direction) in query.iter_mut() {
            // info!("wait: {:?}", state);

            match *state {
                ActionState::Requested => {
                    commands
                        .entity(*actor)
                        .insert(DebugAction::new("go direction", state.clone()));

                    if let Ok(mut walker) = walker_query.get_mut(*actor) {
                        walker.direction = go_direction.direction;
                    }
                    *state = ActionState::Executing;
                }
                ActionState::Executing => {
                    go_direction.timeout.tick(time.delta());
                    if go_direction.timeout.finished() {
                        *state = ActionState::Success;
                    }
                }
                ActionState::Cancelled => {
                    *state = ActionState::Failure;
                }
                _ => {}
            }
        }
    }
}

pub mod go_script {
    use std::{collections::VecDeque, ops::Range};

    use bevy::prelude::*;
    use big_brain::prelude::*;
    use rand::prelude::*;

    use crate::{
        movement::{
            control::MovementGoToPoint,
            crab_controller::CrabFollowPath,
            crab_move::{CrabMoveDirection, CrabMoveWalker},
        },
        path::Waypoint,
    };

    use super::DebugAction;

    #[derive(Component, Debug, Clone)]
    pub struct ActionGoScript {
        steps: VecDeque<(CrabMoveDirection, Timer)>,
    }

    impl ActionGoScript {
        pub fn repeat(steps: &[(CrabMoveDirection, f32)], repeat: Range<usize>) -> Self {
            let repeat = thread_rng().gen_range(repeat);
            Self {
                steps: std::iter::repeat(steps)
                    .take(repeat)
                    .flatten()
                    .map(|(direction, duration)| {
                        (*direction, Timer::from_seconds(*duration, false))
                    })
                    .collect(),
            }
        }
    }

    pub fn action_go_script_system(
        mut commands: Commands,
        time: Res<Time>,
        mut query: Query<(&Actor, &mut ActionState, &mut ActionGoScript)>,
        mut walker_query: Query<&mut CrabMoveWalker>,
    ) {
        for (Actor(actor), mut state, mut go_direction) in query.iter_mut() {
            // info!("wait: {:?}", state);

            match *state {
                ActionState::Requested => {
                    commands
                        .entity(*actor)
                        .insert(DebugAction::new("go script", state.clone()));

                    if let Ok(mut walker) = walker_query.get_mut(*actor) {
                        if let Some(step) = go_direction.steps.front() {
                            walker.direction = step.0;
                        }
                    }
                    *state = ActionState::Executing;
                }
                ActionState::Executing => {
                    if let Some(step) = go_direction.steps.front_mut() {
                        step.1.tick(time.delta());
                        if step.1.finished() {
                            go_direction.steps.pop_front();

                            if let Some(step) = go_direction.steps.front_mut() {
                                if let Ok(mut walker) = walker_query.get_mut(*actor) {
                                    walker.direction = step.0;
                                }
                            }
                        }
                    }
                    if go_direction.steps.is_empty() {
                        *state = ActionState::Success;
                    }
                }
                ActionState::Cancelled => {
                    *state = ActionState::Failure;
                }
                _ => {}
            }
        }
    }
}

// FIXME: this is a bit of a kludge to track the active action. It would be nice to get this
// automatically either from the ecs or big_brain. Maybe look into AnyOf
#[derive(Component, Clone, PartialEq, Eq)]
pub struct DebugAction {
    pub action: &'static str,
    pub state: ActionState,
}

impl DebugAction {
    pub fn new(action: &'static str, state: ActionState) -> Self {
        Self { action, state }
    }
}
