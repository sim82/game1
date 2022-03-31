use crate::{ai::util::TargetDistanceProbe, walk::VelocityWalker, TargetFlag};
use bevy::prelude::*;
use big_brain::prelude::*;

#[derive(Clone, Component, Debug)]
pub struct RunAway;
//  {
// pub until: f32,
// }

// Action systems execute according to a state machine, where the states are
// labeled by ActionState.
pub fn run_away_action_system(
    mut walkers: Query<(&Transform, &TargetDistanceProbe, &mut VelocityWalker)>,
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

    for (Actor(actor), mut state, run_away) in query.iter_mut() {
        // Use the drink_action's actor to look up the corresponding Thirst Component.
        if let Ok((transform, target_distance, mut walker)) = walkers.get_mut(*actor) {
            match *state {
                ActionState::Requested => {
                    // println!("Time to run away!");
                    // let tv = (target_pos - transform.translation).normalize();
                    // walker.velocity = -0.5 * tv;

                    *state = ActionState::Executing;
                }
                ActionState::Executing => {
                    // if target_distance.d <= run_away.until {
                    let tv = (target_pos - transform.translation).normalize();
                    walker.velocity = -1.0 * tv;
                    // info!("walk_velocity: {:?}", walker.velocity);
                    // } else {
                    //     walker.velocity = Vec3::ZERO;
                    //     *state = ActionState::Success;
                    // }
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
