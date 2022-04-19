use std::collections::VecDeque;

use bevy::prelude::*;
use bevy_egui::{
    egui::{self, RichText},
    EguiContext,
};

use crate::{
    movement::{
        control::{MovementEvade, MovementGoToPoint},
        crab_controller::CrabFollowPath,
        zap::{BeingZapped, Zappable},
    },
    path::WaypointPath,
};

use super::{actions::DebugAction, util::Ammo, HealthPoints};

#[derive(Component)]
pub struct AiInspectTarget;

pub struct AiInspectState {
    debug_action: VecDeque<DebugAction>,
    timer: Timer,
}
impl Default for AiInspectState {
    fn default() -> Self {
        Self {
            debug_action: Default::default(),
            timer: Timer::from_seconds(1.0, true),
        }
    }
}

pub fn ai_inspect_pick_target(
    mut commands: Commands,
    target_query: Query<(), With<AiInspectTarget>>,
    candiate_query: Query<Entity, With<Zappable>>,
) {
    if target_query.is_empty() {
        if let Some(entity) = candiate_query.iter().next() {
            commands.entity(entity).insert(AiInspectTarget);
        }
    }
}

#[allow(clippy::too_many_arguments)]
pub fn ai_inspect_egui_system(
    // mut commands: Commands,
    mut egui_context: ResMut<EguiContext>,
    time: Res<Time>,
    mut state: ResMut<AiInspectState>,
    follow_path_query: Query<(&CrabFollowPath, &WaypointPath), With<AiInspectTarget>>,
    evade_query: Query<&MovementEvade>,
    health_query: Query<&HealthPoints, With<AiInspectTarget>>,
    action_query: Query<&DebugAction, (With<AiInspectTarget>, Changed<DebugAction>)>,
    zapped_query: Query<(), (With<BeingZapped>, With<AiInspectTarget>)>,
    goto_point_query: Query<&MovementGoToPoint>,
    ammo_query: Query<&Ammo, With<AiInspectTarget>>,
) {
    for action in action_query.iter() {
        if Some(action) != state.debug_action.back() {
            state.debug_action.push_back(action.clone());
        }
        if state.debug_action.len() > 6 {
            state.debug_action.pop_front();
        }
    }

    state.timer.tick(time.delta());
    if state.timer.just_finished() && state.debug_action.len() > 1 {
        state.debug_action.pop_front();
    }

    egui::Window::new("ai inspect").show(egui_context.ctx_mut(), |ui| {
        if let Ok(health_points) = health_query.get_single() {
            ui.label(format!("health: {}", health_points.health));
        }
        if let Ok(ammo) = ammo_query.get_single() {
            ui.label(format!("ammo: {} {}", ammo.ammo, ammo.reload_time));
        }

        if let Ok((follow, waypoint_path)) = follow_path_query.get_single() {
            ui.label(format!(
                "follow path: len: {} next: {}",
                waypoint_path.waypoints.len(),
                follow.next_step
            ));
        }

        if let Ok(MovementGoToPoint(pos)) = goto_point_query.get_single() {
            ui.label(format!("goto point: {:?}", pos));
        }

        if evade_query.get_single().is_ok() {
            ui.label(RichText::new("EVADE!").color(egui::Color32::RED));
        }

        if zapped_query.get_single().is_ok() {
            ui.label(RichText::new("ZAPPED!").color(egui::Color32::LIGHT_BLUE));
        }

        for action in state.debug_action.iter() {
            ui.label(format!("action: {} {:?}", action.action, action.state));
        }
    });
}
