use crate::Pew;
use bevy::prelude::*;
use big_brain::{
    evaluators::{Evaluator, LinearEvaluator},
    prelude::*,
};

#[derive(Component)]
pub struct PewIncoming;

impl PewIncoming {
    pub fn build() -> PewHitBuilder {
        PewHitBuilder::default()
    }
}

#[derive(Default, Clone, Debug)]
pub struct PewHitBuilder {
    within: f32,
}
impl PewHitBuilder {
    pub fn within(mut self, within: f32) -> Self {
        self.within = within;
        self
    }
}
impl ScorerBuilder for PewHitBuilder {
    fn build(&self, cmd: &mut Commands, scorer: Entity, _actor: Entity) {
        cmd.entity(scorer).insert(PewIncoming);
    }
}

// Looks familiar? It's a lot like Actions!
pub fn pew_incoming_scorer_system(
    // target_distance: Query<&TargetDistanceProbe>,
    // Same dance with the Actor here, but now we use look up Score instead of ActionState.
    mut query: Query<(&Actor, &mut Score), With<PewIncoming>>,
    pew_query: Query<(&Transform, &Pew)>,
    transform_query: Query<&Transform>,
) {
    for (Actor(actor), mut score) in query.iter_mut() {
        let mut pew_collision_score = 0.0;
        if let Ok(Transform { translation, .. }) = transform_query.get(*actor) {
            // check if we're about to be hit by any pew pew coming our way
            for (
                Transform {
                    translation: pew_translation,
                    ..
                },
                Pew(pew_going_right),
            ) in pew_query.iter()
            {
                let dx = translation.x - pew_translation.x;
                let dy = (translation.y - pew_translation.y).abs();
                if (dx > 0.0) == *pew_going_right && dy < 32.0 {
                    pew_collision_score +=
                        LinearEvaluator::new_ranged(300.0, 128.0).evaluate(dx.abs());
                }
            }
        }
        // info!("pew_collision_score: {}", pew_collision_score);
        score.set(pew_collision_score.clamp(0.0, 1.0));
        // fear.fear = (fear.fear - 0.2 * 0.016).clamp(0.0, 1.0);
    }
}
