use bevy::prelude::*;
use big_brain::evaluators::Evaluator;

use crate::{path::Waypoint, tune, TargetFlag};

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

#[derive(Component, Reflect)]
pub struct Ammo {
    pub ammo: f32,
    pub reload_time: f32,
}

impl Default for Ammo {
    fn default() -> Self {
        Self {
            ammo: tune::AMMO_RELOAD_AMOUNT,
            reload_time: tune::AMMO_RELOAD_TIME,
        }
    }
}

pub fn ammo_reload_system(time: Res<Time>, mut query: Query<&mut Ammo>) {
    for mut ammo in query.iter_mut() {
        if ammo.ammo == 0.0 {
            ammo.reload_time -= time.delta_seconds();

            if ammo.reload_time <= 0.0 {
                ammo.ammo = tune::AMMO_RELOAD_AMOUNT;
                ammo.reload_time = tune::AMMO_RELOAD_TIME;
            }
        }
    }
}

// #[derive(Component)]
// struct WaypointAttributes {
//     attack: f32,
// }

pub fn waypoint_attribute_system(
    _query: Query<&Transform, With<Waypoint>>,
    _target_query: Query<&Transform, With<TargetFlag>>,
) {
}
