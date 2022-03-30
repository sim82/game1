use bevy::prelude::*;
use big_brain::prelude::*;

use crate::walk::VelocityWalker;

#[derive(Component, Debug, Clone)]
pub struct JiggleAround {
    left: f32,
    dir: Vec3,
}

impl Default for JiggleAround {
    fn default() -> Self {
        Self {
            left: 0.0,
            dir: Vec3::new(1.0, 0.0, 0.0),
        }
    }
}

pub fn jiggle_around_action_system(
    mut walkers: Query<&mut VelocityWalker>,
    time: Res<Time>,
    mut query: Query<(&Actor, &mut ActionState, &mut JiggleAround)>,
) {
    for (Actor(actor), mut state, mut jiggle_around) in query.iter_mut() {
        match *state {
            ActionState::Requested => *state = ActionState::Executing,
            ActionState::Executing => {
                jiggle_around.left -= time.delta_seconds();
                if jiggle_around.left <= 0.0 {
                    jiggle_around.left = 0.3;
                    jiggle_around.dir *= -1.0;
                    if let Ok(mut walker) = walkers.get_mut(*actor) {
                        walker.velocity = jiggle_around.dir * 0.5;
                    }
                }
            }
            ActionState::Cancelled => {
                // info!("jiggle around cancelled");
                *state = ActionState::Failure
            }
            _ => {}
        }
    }
}
