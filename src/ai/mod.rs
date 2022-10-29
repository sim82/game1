use bevy::prelude::*;
use big_brain::BigBrainStage;

use crate::ai::{
    actions::{
        go_direction::action_go_direction_system,
        go_script::action_go_script_system,
        goto_pos::action_goto_pos_system,
        pick_goto_pos::{self, action_pick_goto_pos_system},
        wait::action_wait_system,
    },
    inspect::AiInspectState,
    util::ammo_reload_system,
};

pub mod actions;
pub mod diagnostics;
pub mod inspect;
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
                goto_medikit::goto_medikit_action_system,
                jiggle_around::jiggle_around_action_system, run_away::run_away_action_system,
                shoot::shoot_action_system,
            },
            scorers::{
                can_shoot::can_shoot_scorer_system, crowdiness::crowdiness_scorer_system,
                curiosity::curiousity_scorer_system, fear::fear_scorer_system,
                health_low::low_health_scorer_system, pew_incoming::pew_incoming_scorer_system,
            },
        };

        app.register_type::<util::TargetDistanceProbe>()
            .register_type::<HealthPoints>()
            .add_system(inspect::ai_inspect_egui_system)
            .add_system(inspect::ai_inspect_pick_target)
            .init_resource::<AiInspectState>()
            .add_system_to_stage(CoreStage::PostUpdate, util::measure_target_distance_system)
            .add_system(ammo_reload_system)
            // actions
            .add_system_to_stage(BigBrainStage::Actions, run_away_action_system)
            .add_system_to_stage(BigBrainStage::Actions, follow_action_system)
            .add_system_to_stage(BigBrainStage::Actions, jiggle_around_action_system)
            .add_system_to_stage(BigBrainStage::Actions, dodge_pew_action_system)
            .add_system_to_stage(BigBrainStage::Actions, goto_medikit_action_system)
            .add_system_to_stage(BigBrainStage::Actions, shoot_action_system)
            .add_system_to_stage(BigBrainStage::Actions, action_pick_goto_pos_system)
            .add_system_to_stage(BigBrainStage::Actions, action_goto_pos_system)
            .add_system_to_stage(BigBrainStage::Actions, action_wait_system)
            .add_system_to_stage(BigBrainStage::Actions, action_go_direction_system)
            .add_system_to_stage(BigBrainStage::Actions, action_go_script_system)
            // scorers
            .add_system_to_stage(BigBrainStage::Scorers, fear_scorer_system)
            .add_system_to_stage(BigBrainStage::Scorers, curiousity_scorer_system)
            .add_system_to_stage(BigBrainStage::Scorers, pew_incoming_scorer_system)
            .add_system_to_stage(BigBrainStage::Scorers, low_health_scorer_system)
            .add_system_to_stage(BigBrainStage::Scorers, can_shoot_scorer_system)
            .add_system_to_stage(BigBrainStage::Scorers, crowdiness_scorer_system);
    }
}
