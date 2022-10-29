use bevy::prelude::*;
use big_brain::prelude::*;

use crate::movement::control::MovementEvade;

use super::DebugAction;

#[derive(Component, Debug, Clone)]
pub struct DodgePew {
    timeout: Timer,
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
        });
    }
}

pub fn dodge_pew_action_system(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(&Actor, &mut ActionState, &mut DodgePew)>,
) {
    for (Actor(actor), mut state, mut dodge_pew) in query.iter_mut() {
        commands
            .entity(*actor)
            .insert(DebugAction::new("dodge pew", state.clone()));
        match *state {
            ActionState::Requested => {
                commands.entity(*actor).insert(MovementEvade::default());
                *state = ActionState::Executing;
            }
            ActionState::Executing => {
                dodge_pew.timeout.tick(time.delta());
                if dodge_pew.timeout.finished() {
                    *state = ActionState::Cancelled
                }
            }
            ActionState::Cancelled => {
                commands.entity(*actor).remove::<MovementEvade>();
                *state = ActionState::Success;
            }
            _ => {}
        }
    }
}
