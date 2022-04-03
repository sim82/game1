use bevy::prelude::*;
use big_brain::prelude::*;

use crate::movement::{
    crab_move::{self, CrabMoveWalker},
    walk::VelocityWalker,
};

#[derive(Component, Debug, Clone)]
pub struct JiggleAround {
    left: f32,
    dir: crab_move::Direction,
}

impl Default for JiggleAround {
    fn default() -> Self {
        Self {
            left: 0.0,
            dir: crab_move::Direction::None,
        }
    }
}

pub fn jiggle_around_action_system(
    mut walkers: Query<&mut CrabMoveWalker>,
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
                    jiggle_around.dir = if jiggle_around.dir == crab_move::Direction::East {
                        crab_move::Direction::West
                    } else {
                        crab_move::Direction::East
                    };
                    if let Ok(mut walker) = walkers.get_mut(*actor) {
                        walker.direction = jiggle_around.dir;
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
