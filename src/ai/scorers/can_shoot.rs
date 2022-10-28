use bevy::prelude::*;
use big_brain::{
    evaluators::{Evaluator, LinearEvaluator},
    prelude::*,
};

use crate::{ai::util::Ammo, movement::crab_move::CrabMoveWalker, tune, TargetFlag};

#[derive(Component, Debug, Clone)]
pub struct CanShoot {
    evaluator: LinearEvaluator,
}

impl Default for CanShoot {
    fn default() -> Self {
        Self {
            evaluator: LinearEvaluator::new_ranged(
                tune::PEW_ZAP_DISTANCE * 1.5,
                tune::PEW_ZAP_DISTANCE * 0.5,
            ),
        }
    }
}

pub fn can_shoot_scorer_system(
    mut query: Query<(&Actor, &mut Score, &CanShoot)>,
    player_query: Query<&Transform, With<TargetFlag>>,
    my_query: Query<(&Transform, &CrabMoveWalker, &Ammo)>,
) {
    for (Actor(actor_entity), mut score, can_shoot) in query.iter_mut() {
        let (
            Transform {
                translation: my_pos,
                ..
            },
            CrabMoveWalker { direction: _ },
            ammo,
        ) = my_query.get(*actor_entity).unwrap(); // FIXME

        if let Ok(Transform {
            translation: target_pos,
            ..
        }) = player_query.get_single()
        {
            let _target_right = target_pos.x > my_pos.x;
            let s = if ammo.ammo > 0.0 {
                can_shoot
                    .evaluator
                    .evaluate((target_pos.y - my_pos.y).abs())
            } else {
                0.0
            };

            score.set(s);
        }
    }
}
