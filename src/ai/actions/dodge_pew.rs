use bevy::prelude::*;
use big_brain::prelude::*;

use crate::{walk::VelocityWalker, Pew};
use rand::Rng;

#[derive(Component, Debug, Clone)]
pub struct DodgePew {
    direction: f32,
}

impl DodgePew {
    pub fn build() -> DodgePewBuilder {
        DodgePewBuilder
    }
}

#[derive(Debug, Clone)]
pub struct DodgePewBuilder;

// impl DodgePewBuilder {

// }

impl ActionBuilder for DodgePewBuilder {
    fn build(&self, cmd: &mut Commands, scorer: Entity, _actor: Entity) {
        cmd.entity(scorer).insert(DodgePew { direction: 0.0 });
    }
}

// impl Default for DodgePew {
//     fn default() -> Self {
//         Self { direction: 0.0 }
//     }
// }

pub fn dodge_pew_action_system(
    mut walkers: Query<(&mut VelocityWalker, &Transform)>,
    time: Res<Time>,
    mut query: Query<(&Actor, &mut ActionState, &mut DodgePew)>,
    pew_query: Query<(&Transform, &Pew)>,
) {
    for (Actor(actor), mut state, mut dodge_pew) in query.iter_mut() {
        match *state {
            ActionState::Requested => {
                // if let Ok((_, Transform { translation, .. })) = walkers.get_mut(*actor) {
                //     for (
                //         Transform {
                //             translation: pew_translation,
                //             ..
                //         },
                //         Pew(pew_going_right),
                //     ) in pew_query.iter()
                //     {
                //         let dx = translation.x - pew_translation.x;
                //         let dy = translation.y - pew_translation.y;
                //         if (dx > 0.0) == *pew_going_right && dy.abs() < 64.0 {
                //             // found dangerous pew
                //             *state = ActionState::Executing;
                //             dodge_pew.direction = dy.signum();
                //             return;
                //         }
                //     }
                // }
                // *state = ActionState::Success;

                // more funny: just panic and run in random direction
                let mut rng = rand::thread_rng();
                if rng.gen_bool(0.5) {
                    dodge_pew.direction = 1.0;
                } else {
                    dodge_pew.direction = -1.0;
                }
                *state = ActionState::Executing;
            }
            ActionState::Executing => {
                if let Ok((mut walker, _)) = walkers.get_mut(*actor) {
                    walker.velocity = Vec3::new(0.0, dodge_pew.direction, 0.0);
                }
            }
            ActionState::Cancelled => *state = ActionState::Failure,
            _ => {}
        }
    }
}
