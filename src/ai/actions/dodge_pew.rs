use bevy::prelude::*;
use big_brain::prelude::*;

use crate::movement::crab_controller::CrabEvade;

use super::DebugAction;

#[derive(Component, Debug, Clone)]
pub struct DodgePew {}

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
        cmd.entity(scorer).insert(DodgePew {});
    }
}

pub fn dodge_pew_action_system(
    mut commands: Commands,
    mut query: Query<(&Actor, &mut ActionState, &mut DodgePew)>,
) {
    for (Actor(actor), mut state, mut _dodge_pew) in query.iter_mut() {
        commands
            .entity(*actor)
            .insert(DebugAction::new("dodge pew", state.clone()));
        match *state {
            ActionState::Requested => {
                commands.entity(*actor).insert(CrabEvade::default());
                *state = ActionState::Executing;
            }
            ActionState::Executing => {}
            ActionState::Cancelled => {
                commands.entity(*actor).remove::<CrabEvade>();
                *state = ActionState::Failure;
            }
            _ => {}
        }
    }
}
