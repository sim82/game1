use crate::{
    ai::util::TargetDistanceProbe,
    movement::crab_move::{CrabMoveDirection, CrabMoveWalker},
    TargetFlag,
};
use bevy::prelude::*;
use big_brain::prelude::*;

use super::DebugAction;

#[derive(Clone, Component, Debug)]
pub struct RunAway;
//  {
// pub until: f32,
// }

// Action systems execute according to a state machine, where the states are
// labeled by ActionState.
pub fn run_away_action_system(
    mut commands: Commands,
    mut walkers: Query<(&Transform, &TargetDistanceProbe, &mut CrabMoveWalker)>,
    target_query: Query<&Transform, With<TargetFlag>>,
    // We execute actions by querying for their associated Action Component
    // (Drink in this case). You'll always need both Actor and ActionState.
    mut query: Query<(&Actor, &mut ActionState, &RunAway)>,
) {
    let target_pos = target_query
        .iter()
        .next()
        .map(|t| t.translation)
        .unwrap_or_default();

    for (Actor(actor), mut state, _run_away) in query.iter_mut() {
        commands
            .entity(*actor)
            .insert(DebugAction::new("run away", state.clone()));

        // Use the drink_action's actor to look up the corresponding Thirst Component.
        if let Ok((transform, _target_distance, mut walker)) = walkers.get_mut(*actor) {
            match *state {
                ActionState::Requested => {
                    *state = ActionState::Executing;
                }
                ActionState::Executing => {
                    let tv = (transform.translation - target_pos).normalize();
                    walker.direction = CrabMoveDirection::find_nearest(tv);
                }
                // All Actions should make sure to handle cancellations!
                ActionState::Cancelled => {
                    *state = ActionState::Failure;
                }
                _ => {}
            }
        } else {
            // no VelocityWalker and/or TargetDistance -> fail
            *state = ActionState::Failure;
        }
    }
}
