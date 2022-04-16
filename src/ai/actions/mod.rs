use bevy::prelude::*;
use big_brain::prelude::*;

pub mod dodge_pew;
pub mod follow;
pub mod goto_medikit;
pub mod jiggle_around;
pub mod run_away;

// FIXME: this is a bit of a kludge to track the active action. It would be nice to get this
// automatically either from the ecs or big_brain. Maybe look into AnyOf
#[derive(Component, Clone, PartialEq)]
pub struct DebugAction {
    pub action: &'static str,
    pub state: ActionState,
}

impl DebugAction {
    pub fn new(action: &'static str, state: ActionState) -> Self {
        Self { action, state }
    }
}
