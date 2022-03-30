use crate::ai::util::{TargetDistanceProbe, ThresholdEvaluator};
use bevy::prelude::*;
use big_brain::{evaluators::Evaluator, prelude::*};

#[derive(Component, Debug)]
pub struct Curiousity {
    // within: f32,
    alone_time: f32,
    evaluator: ThresholdEvaluator,
}

#[derive(Default, Clone, Debug)]
pub struct CuriousityBuilder {
    within: f32,
}
impl CuriousityBuilder {
    pub fn within(mut self, within: f32) -> Self {
        self.within = within;
        self
    }
}
impl ScorerBuilder for CuriousityBuilder {
    fn build(&self, cmd: &mut Commands, scorer: Entity, _actor: Entity) {
        cmd.entity(scorer).insert(Curiousity {
            alone_time: 0.0,
            evaluator: ThresholdEvaluator::new(self.within, true),
        });
    }
}

impl Curiousity {
    pub fn build() -> CuriousityBuilder {
        CuriousityBuilder::default()
    }
}
pub fn curiousity_scorer_system(
    target_distance: Query<&TargetDistanceProbe>,
    time: Res<Time>,
    // Same dance with the Actor here, but now we use look up Score instead of ActionState.
    mut query: Query<(&Actor, &mut Score, &mut Curiousity)>,
) {
    // info!("curious scorer {:?}", std::thread::current());

    for (Actor(actor), mut score, mut curious) in query.iter_mut() {
        // info!("curious_scorer {:?} {:?}", std::thread::current(), actor);
        if let Ok(target_distance) = target_distance.get(*actor) {
            if curious.evaluator.evaluate(target_distance.d) >= 0.8 {
                curious.alone_time += time.delta_seconds();
            } else {
                curious.alone_time = 0.0;
            }
            score.set((curious.alone_time / 5.0).clamp(0.0, 1.0));
            //     score.set(curious.evaluator.evaluate(target_distance.d))
            //     //     // become curious if more than 200 away from target
            //     //     if target_distance.d > curious.within {
            //     //         score.set(1.0)
            //     //     } else {
            //     //         score.set(0.0)
            //     //     }
        }
    }
}
