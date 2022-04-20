use bevy::prelude::*;
use big_brain::{
    evaluators::{Evaluator, LinearEvaluator},
    prelude::*,
};

use crate::{ai::util::Ammo, movement::crab_move::CrabMoveWalker, tune, TargetFlag};

#[derive(Component, Debug, Clone)]
pub struct Crowdiness {
    evaluator: LinearEvaluator,
}

impl Default for Crowdiness {
    fn default() -> Self {
        Self {
            evaluator: LinearEvaluator::new_ranged(0.0, 75.0),
        }
    }
}

pub fn crowdiness_scorer_system(
    mut query: Query<(&Actor, &mut Score, &Crowdiness)>,
    player_query: Query<&Transform, With<TargetFlag>>,
    my_query: Query<&Transform, With<CrabMoveWalker>>,
) {
    let positions = my_query
        .iter()
        .map(|Transform { translation, .. }| translation)
        .collect::<Vec<_>>();

    for (Actor(actor_entity), mut score, crowded) in query.iter_mut() {
        let Transform {
            translation: my_pos,
            ..
        } = my_query.get(*actor_entity).unwrap();
        let crowdiness = positions
            .iter()
            .map(|pos| (*my_pos - **pos).length())
            .filter(|d| *d < 24.0)
            .sum::<f32>();
        // info!("crowdiness: {}", crowdiness);
        score.set(crowded.evaluator.evaluate(crowdiness));
    }
}
