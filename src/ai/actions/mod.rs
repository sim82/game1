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

    use crate::{movement::control::MovementGoToPoint, path::Waypoint};

    use super::DebugAction;

    #[derive(Debug, Clone)]
    pub enum TargetPos {
        Random,
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
    ) {
        for (Actor(actor), mut state, mut pick_goto_pos) in query.iter_mut() {
            info!("pick: {:?}", state);

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
                            cand_pos[rng.gen_range(0..cand_pos.len())]
                        }
                    };
                    commands
                        .entity(*actor)
                        .insert(MovementGoToPoint(best_pos.translation));

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
            control::MovementGoToPoint, crab_controller::CrabFollowPath,
            crab_move::CrabMoveDirection,
        },
        path::Waypoint,
    };

    use super::DebugAction;

    #[derive(Component, Debug, Clone)]
    pub struct ActionGotoPos;

    pub fn action_goto_pos_system(
        mut commands: Commands,
        mut query: Query<(&Actor, &mut ActionState, &mut ActionGotoPos)>,
        actor_query: Query<&Transform>,
        go_to_point_query: Query<(), With<MovementGoToPoint>>,
    ) {
        for (Actor(actor), mut state, mut goto_pos) in query.iter_mut() {
            info!("goto: {:?}", state);

            match *state {
                // let mut nearest_dist
                ActionState::Requested => {
                    commands
                        .entity(*actor)
                        .insert(DebugAction::new("goto pos", state.clone()));
                    *state = ActionState::Executing
                }
                ActionState::Executing => {
                    if go_to_point_query.get(*actor).is_err() {
                        *state = ActionState::Success;
                    }
                }
                ActionState::Cancelled => {
                    commands
                        .entity(*actor)
                        .remove::<MovementGoToPoint>()
                        .remove::<CrabFollowPath>();
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
            info!("wait: {:?}", state);

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
