use bevy::prelude::*;
use big_brain::prelude::*;
use rand::Rng;

use crate::movement::{
    control::MovementEvade,
    crab_move::{CrabMoveDirection, CrabMoveWalker},
};

use super::DebugAction;

#[derive(Component, Debug, Clone)]
pub struct DodgePew {
    timeout: Timer,
    direction_change: Timer,
}

impl DodgePew {
    pub fn build() -> DodgePewBuilder {
        DodgePewBuilder
    }
}

#[derive(Debug, Clone)]
pub struct DodgePewBuilder;

// }

impl ActionBuilder for DodgePewBuilder {
    fn build(&self, cmd: &mut Commands, scorer: Entity, _actor: Entity) {
        cmd.entity(scorer).insert(DodgePew {
            timeout: Timer::from_seconds(0.5, false),
            direction_change: Timer::from_seconds(0.2, true),
        });
    }
}

pub fn dodge_pew_action_system(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(&Actor, &mut ActionState, &mut DodgePew)>,
    mut walker_query: Query<&mut CrabMoveWalker>,
) {
    for (Actor(actor), mut state, mut dodge_pew) in query.iter_mut() {
        match *state {
            ActionState::Requested => {
                commands
                    .entity(*actor)
                    .insert(DebugAction::new("dodge pew", state.clone()));

                *state = ActionState::Executing;
            }
            ActionState::Executing => {
                dodge_pew.timeout.tick(time.delta());
                if dodge_pew.timeout.finished() {
                    *state = ActionState::Success;
                    continue;
                }
                dodge_pew.direction_change.tick(time.delta());

                if dodge_pew.direction_change.finished() {
                    // info!("dodging");
                    let mut walker = walker_query.get_mut(*actor).unwrap();
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
            ActionState::Cancelled => {
                *state = ActionState::Success;
            }
            _ => {}
        }
    }
}
