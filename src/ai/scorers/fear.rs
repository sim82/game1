use bevy::prelude::*;
use big_brain::{evaluators::Evaluator, prelude::*};

use crate::ai::util::{TargetDistanceProbe, ThresholdEvaluator};

#[derive(Component, Debug)]
pub struct Fear {
    evaluator: ThresholdEvaluator,
}

impl Fear {
    pub fn build() -> FearBuilder {
        FearBuilder::default()
    }
}

#[derive(Default, Clone, Debug)]
pub struct FearBuilder {
    within: f32,
}
impl FearBuilder {
    pub fn within(mut self, within: f32) -> Self {
        self.within = within;
        self
    }
}
impl ScorerBuilder for FearBuilder {
    fn build(&self, cmd: &mut Commands, scorer: Entity, _actor: Entity) {
        cmd.entity(scorer).insert(Fear {
            evaluator: ThresholdEvaluator::new(self.within, false),
        });
    }
}

// Looks familiar? It's a lot like Actions!
pub fn fear_scorer_system(
    target_distance: Query<&TargetDistanceProbe>,
    // Same dance with the Actor here, but now we use look up Score instead of ActionState.
    mut query: Query<(&Actor, &mut Score, &Fear)>,
) {
    // info!("fear scorer {:?}", std::thread::current());

    for (Actor(actor), mut score, fear) in query.iter_mut() {
        debug!("fear_scorer {:?} {:?}", std::thread::current(), actor);
        if let Ok(target_distance) = target_distance.get(*actor) {
            score.set(fear.evaluator.evaluate(target_distance.d))
        }
    }
}
