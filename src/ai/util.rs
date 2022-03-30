use bevy::prelude::*;
use big_brain::evaluators::Evaluator;

use crate::TargetFlag;

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct TargetDistanceProbe {
    pub d: f32,
}

pub fn measure_target_distance_system(
    mut query: Query<(&mut TargetDistanceProbe, &Transform)>,
    target_query: Query<&Transform, With<TargetFlag>>,
) {
    // info!("measure target distance {:?}", std::thread::current());
    let target_pos = target_query
        .iter()
        .next()
        .map(|t| t.translation)
        .unwrap_or_default();

    for (mut probe, transform) in query.iter_mut() {
        probe.d = (target_pos - transform.translation).length();
    }
}

#[derive(Debug, Default)]
pub struct ThresholdEvaluator {
    threshold: f32,
    above: bool,
}

impl ThresholdEvaluator {
    pub fn new(threshold: f32, above: bool) -> Self {
        Self { threshold, above }
    }
}

impl Evaluator for ThresholdEvaluator {
    fn evaluate(&self, value: f32) -> f32 {
        if (value >= self.threshold) == self.above {
            1.0
        } else {
            0.0
        }
    }
}
