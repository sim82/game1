use bevy::prelude::*;
use big_brain::{
    evaluators::{Evaluator, LinearEvaluator},
    prelude::*,
};

use crate::ai::util::TargetDistanceProbe;

#[derive(Component, Debug)]
pub struct Fear {
    fear: f32,
    evaluator: LinearEvaluator,
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
            fear: 0.0,
            evaluator: LinearEvaluator::new_ranged(self.within, 0.0),
        });
    }
}

// Looks familiar? It's a lot like Actions!
pub fn fear_scorer_system(
    time: Res<Time>,
    target_distance: Query<&TargetDistanceProbe>,
    // Same dance with the Actor here, but now we use look up Score instead of ActionState.
    mut query: Query<(&Actor, &mut Score, &mut Fear)>,
) {
    // info!("fear scorer {:?}", std::thread::current());

    for (Actor(actor), mut score, mut fear) in query.iter_mut() {
        debug!("fear_scorer {:?} {:?}", std::thread::current(), actor);
        if let Ok(target_distance) = target_distance.get(*actor) {
            fear.fear = (fear.fear + fear.evaluator.evaluate(target_distance.d)).clamp(0.0, 1.0);
            // info!("fear: {}", fear.fear);
        }
        score.set(fear.fear);
        fear.fear = (fear.fear - 0.2 * time.delta_seconds()).clamp(0.0, 1.0);
    }
}
