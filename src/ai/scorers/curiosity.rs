use crate::ai::util::{TargetDistanceProbe, ThresholdEvaluator};
use bevy::prelude::*;
use big_brain::{
    evaluators::{Evaluator, LinearEvaluator},
    prelude::*,
};

#[derive(Component, Debug)]
pub struct Curiousity {
    curiosity: f32,
    evaluator: LinearEvaluator,
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
            curiosity: 0.0,
            evaluator: LinearEvaluator::new_ranged(self.within, self.within * 2.0),
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

    // for (Actor(actor), mut score, mut curious) in query.iter_mut() {
    //     // info!("curious_scorer {:?} {:?}", std::thread::current(), actor);
    //     if let Ok(target_distance) = target_distance.get(*actor) {
    //         if curious.evaluator.evaluate(target_distance.d) >= 0.8 {
    //             curious.alone_time += time.delta_seconds();
    //         } else {
    //             curious.alone_time = 0.0;
    //         }

    //         let value = (curious.alone_time / 5.0).clamp(0.0, 1.0);
    //         info!("curiosity: {}", value);
    //         score.set(value);

    //         //     score.set(curious.evaluator.evaluate(target_distance.d))
    //         //     //     // become curious if more than 200 away from target
    //         //     //     if target_distance.d > curious.within {
    //         //     //         score.set(1.0)
    //         //     //     } else {
    //         //     //         score.set(0.0)
    //         //     //     }
    //     }
    // }
    for (Actor(actor), mut score, mut curiosity) in query.iter_mut() {
        debug!("curiosity_scorer {:?} {:?}", std::thread::current(), actor);
        if let Ok(target_distance) = target_distance.get(*actor) {
            curiosity.curiosity = (curiosity.curiosity
                + curiosity.evaluator.evaluate(target_distance.d))
            .clamp(0.0, 1.0);
            // info!("curiosity: {}", curiosity.curiosity);
        }
        score.set(curiosity.curiosity);
        curiosity.curiosity = (curiosity.curiosity - 0.2 * time.delta_seconds()).clamp(0.0, 1.0);
    }
}
