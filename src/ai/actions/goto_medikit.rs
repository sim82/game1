use bevy::prelude::*;
use big_brain::prelude::*;

use crate::{
    item::medikit::Medikit,
    movement::{crab_controller::CrabFollowPath, crab_move::CrabMoveWalker},
    path::PathQuery,
};

use super::DebugAction;

#[derive(Component, Debug, Default, Clone)]
pub struct GotoMedikit {
    pos: Vec3,
}

pub fn goto_medikit_action_system(
    mut commands: Commands,
    mut query: Query<(&Actor, &mut ActionState, &mut GotoMedikit)>,
    mut actor_query: Query<(&Transform, &mut CrabMoveWalker)>,
    medikit_query: Query<&Transform, With<Medikit>>,
) {
    for (Actor(actor), mut state, mut goto_medikit) in query.iter_mut() {
        commands
            .entity(*actor)
            .insert(DebugAction::new("goto medikit", state.clone()));

        let (
            Transform {
                translation: actor_pos,
                ..
            },
            mut _walker,
        ) = actor_query.get_mut(*actor).unwrap();
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
                    commands.spawn().insert(PathQuery {
                        start: *actor_pos,
                        end: best_pos,
                        target: *actor,
                    });
                    *state = ActionState::Executing
                } else {
                    info!("failed to find medikit");
                    *state = ActionState::Failure
                }
            }
            ActionState::Executing => {
                // let tv = (goto_medikit.pos - *actor_pos).normalize();
                // walker.direction = CrabMoveDirection::find_nearest(tv);
            }
            ActionState::Cancelled => {
                commands.entity(*actor).remove::<CrabFollowPath>();
                *state = ActionState::Failure;
            }
            _ => {}
        }
    }
}
