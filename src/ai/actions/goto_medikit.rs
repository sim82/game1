use bevy::prelude::*;
use big_brain::prelude::*;

use crate::{
    item::medikit::Medikit,
    movement::{control::MovementGoToPoint, crab_controller::CrabFollowPath},
};

use super::DebugAction;

#[derive(Component, Debug, Default, Clone)]
pub struct GotoMedikit {
    pos: Vec3,
}

pub fn goto_medikit_action_system(
    mut commands: Commands,
    mut query: Query<(&Actor, &mut ActionState, &mut GotoMedikit)>,
    actor_query: Query<&Transform>,
    medikit_query: Query<&Transform, With<Medikit>>,
    go_to_point_query: Query<(), With<MovementGoToPoint>>,
) {
    for (Actor(actor), mut state, mut goto_medikit) in query.iter_mut() {
        commands
            .entity(*actor)
            .insert(DebugAction::new("goto medikit", state.clone()));

        let Transform {
            translation: actor_pos,
            ..
        } = actor_query.get(*actor).unwrap();
        match *state {
            // let mut nearest_dist
            ActionState::Init => {
                let best_pos = medikit_query
                    .iter()
                    .map(
                        |Transform {
                             translation: medikit_pos,
                             ..
                         }| *medikit_pos,
                    )
                    .min_by_key(|medikit_pos| (*medikit_pos - *actor_pos).length() as i32);

                if let Some(best_pos) = best_pos {
                    goto_medikit.pos = best_pos;
                    info!("goto medikit: pos: {:?}", goto_medikit.pos);
                    // commands.entity(*actor).insert(PathQuery {
                    //     start: *actor_pos,
                    //     end: best_pos,
                    // });
                    commands.entity(*actor).insert(MovementGoToPoint(best_pos));
                    *state = ActionState::Executing
                } else {
                    info!("failed to find medikit");
                    *state = ActionState::Failure
                }
            }
            ActionState::Executing => {
                // let tv = (goto_medikit.pos - *actor_pos).normalize();
                // walker.direction = CrabMoveDirection::find_nearest(tv);
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
