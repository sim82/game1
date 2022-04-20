use bevy::prelude::*;
use big_brain::prelude::*;

use crate::{
    ai::util::TargetDistanceProbe,
    movement::{
        control::MovementGoToPoint,
        crab_controller::CrabFollowPath,
        crab_move::{CrabMoveDirection, CrabMoveWalker},
    },
    TargetFlag,
};

use super::DebugAction;

#[derive(Clone, Component, Debug)]
pub struct Follow {
    pub until: f32,
}

pub fn follow_action_system(
    mut commands: Commands,
    mut walkers: Query<(&Transform, &TargetDistanceProbe, &mut CrabMoveWalker)>,
    target_query: Query<&Transform, With<TargetFlag>>,
    // We execute actions by querying for their associated Action Component
    // (Drink in this case). You'll always need both Actor and ActionState.
    mut query: Query<(&Actor, &mut ActionState, &Follow)>,
    go_to_point_query: Query<(), With<MovementGoToPoint>>,
) {
    let target_pos = target_query
        .iter()
        .next()
        .map(|t| t.translation)
        .unwrap_or_default();

    for (Actor(actor), mut state, go_to_target) in query.iter_mut() {
        commands
            .entity(*actor)
            .insert(DebugAction::new("follow", state.clone()));

        // Use the drink_action's actor to look up the corresponding Thirst Component.
        if let Ok((transform, target_distance, mut walker)) = walkers.get_mut(*actor) {
            match *state {
                ActionState::Requested => {
                    // println!("Time to follow the target!");
                    // let tv = (target_pos - transform.translation).normalize();
                    // walker.velocity = -0.5 * tv;
                    commands
                        .entity(*actor)
                        .insert(MovementGoToPoint(target_pos));
                    *state = ActionState::Executing;
                }
                ActionState::Executing => {
                    if go_to_point_query.get(*actor).is_err() {
                        *state = ActionState::Success;
                    }
                }
                // All Actions should make sure to handle cancellations!
                ActionState::Cancelled => {
                    // info!("follow target cancelled");
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
