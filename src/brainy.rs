use bevy::prelude::*;
use big_brain::prelude::*;

use crate::TargetFlag;

pub struct BrainyPlugin;

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct TargetDistanceProbe {
    pub d: f32,
}

fn measure_target_distance(
    mut query: Query<(&mut TargetDistanceProbe, &Transform)>,
    target_query: Query<&Transform, With<TargetFlag>>,
) {
}

impl Plugin for BrainyPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(BigBrainPlugin)
            .register_type::<TargetDistanceProbe>()
            .add_system(measure_target_distance);
    }
}
