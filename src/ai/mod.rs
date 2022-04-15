use bevy::prelude::*;
use big_brain::BigBrainStage;

pub mod actions;
pub mod diagnostics;
pub mod scorers;
pub mod util;
pub struct AiPlugin;

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct HealthPoints {
    pub health: i32,
}

impl Plugin for AiPlugin {
    fn build(&self, app: &mut App) {
        use self::{
            actions::{
                dodge_pew::dodge_pew_action_system, follow::follow_action_system,
                follow_path::follow_path_action_system, jiggle_around::jiggle_around_action_system,
                run_away::run_away_action_system,
            },
            scorers::{
                can_follow_path::can_follow_path_scorer_system,
                curiosity::curiousity_scorer_system, fear::fear_scorer_system,
                health_low::low_health_scorer_system, pew_incoming::pew_incoming_scorer_system,
            },
        };

        app.register_type::<util::TargetDistanceProbe>()
            .register_type::<HealthPoints>()
            .add_system_to_stage(CoreStage::PostUpdate, util::measure_target_distance_system)
            .add_system_to_stage(BigBrainStage::Actions, run_away_action_system)
            .add_system_to_stage(BigBrainStage::Actions, follow_action_system)
            .add_system_to_stage(BigBrainStage::Actions, jiggle_around_action_system)
            .add_system_to_stage(BigBrainStage::Actions, dodge_pew_action_system)
            // .add_system_to_stage(BigBrainStage::Actions, follow_path_action_system)
            .add_system_to_stage(BigBrainStage::Scorers, fear_scorer_system)
            .add_system_to_stage(BigBrainStage::Scorers, curiousity_scorer_system)
            .add_system_to_stage(BigBrainStage::Scorers, pew_incoming_scorer_system)
            .add_system_to_stage(BigBrainStage::Scorers, low_health_scorer_system)
            // .add_system_to_stage(BigBrainStage::Scorers, can_follow_path_scorer_system)
            ;
    }
}
