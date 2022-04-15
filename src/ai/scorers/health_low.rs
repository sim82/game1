use bevy::prelude::*;
use big_brain::{
    evaluators::{Evaluator, LinearEvaluator},
    prelude::*,
};

use crate::ai::HealthPoints;

#[derive(Component, Debug)]
pub struct LowHealth {
    evaluator: LinearEvaluator,
}

impl LowHealth {
    pub fn build() -> LowHealthBuilder {
        LowHealthBuilder::default()
    }
}

#[derive(Clone, Default, Debug)]
pub struct LowHealthBuilder {}

impl ScorerBuilder for LowHealthBuilder {
    fn build(&self, cmd: &mut Commands, scorer: Entity, _actor: Entity) {
        cmd.entity(scorer).insert(LowHealth {
            evaluator: LinearEvaluator::new_ranged(75.0, 10.0),
        });
    }
}

pub fn low_health_scorer_system(
    health_query: Query<&HealthPoints>,
    // Same dance with the Actor here, but now we use look up Score instead of ActionState.
    mut query: Query<(&Actor, &mut Score, &LowHealth)>,
) {
    for (Actor(actor), mut score, low_health) in query.iter_mut() {
        if let Ok(health_points) = health_query.get(*actor) {
            let value = low_health.evaluator.evaluate(health_points.health as f32);
            // info!("health score: {}", value);
            score.set(value);
        }
    }
}
