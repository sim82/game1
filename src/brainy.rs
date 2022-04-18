use bevy::prelude::*;
use bevy_aseprite::{AsepriteAnimation, AsepriteBundle};
use bevy_egui::{egui, EguiContext};
use big_brain::prelude::*;
use rand::{prelude::SliceRandom, Rng};

use crate::{
    ai::{
        actions::{
            dodge_pew::DodgePew, follow::Follow, goto_medikit::GotoMedikit,
            jiggle_around::JiggleAround, run_away::RunAway,
        },
        inspect::AiInspectTarget,
        scorers::{
            curiosity::Curiousity, fear::Fear, health_low::LowHealth, pew_incoming::PewIncoming,
        },
        util::TargetDistanceProbe,
        HealthPoints,
    },
    item::ItemContactProbe,
    movement::{crab_move::CrabMoveWalker, walk::VelocityWalker, zap::Zappable},
    path::Waypoint,
    sprites,
    ui::TrackingOverlayTarget,
    Despawn,
};

mod tune {
    pub const FEAR_DISTANCE: f32 = 50.0;
    pub const CURIOSITY_DISTANCE: f32 = 150.0;
    pub const FOLLOW_MIN_DISTANCE: f32 = 16.0;
    pub const BRAINY_FERRIS_COUNT: usize = 1;
}

pub fn spawn_brainy_ferris(commands: &mut Commands, pos: Vec3, inspect_target: bool) {
    let mut rng = rand::thread_rng();

    let threshold = if inspect_target {
        // make first brainy ferris deterministic
        0.8
    } else {
        let dist = rand_distr::Normal::new(0.8f32, 0.2f32).unwrap();
        rng.sample(dist).clamp(0.0, 1.0)
    };
    // let curious_min = rng.sample(dist);

    // let _curious_max = curious_min + rng.sample(dist);

    let mut entity_commands = commands.spawn_bundle(AsepriteBundle {
        aseprite: sprites::Ferris::sprite(),
        animation: AsepriteAnimation::from(sprites::Ferris::tags::WALK_RIGHT),
        transform: Transform {
            scale: Vec3::splat(1.),
            translation: pos,
            ..Default::default()
        },
        ..Default::default()
    });
    // .insert(VelocityWalker {
    //     velocity: Vec3::ZERO,
    // })
    entity_commands
        .insert(Zappable)
        .insert(CrabMoveWalker::default())
        .insert(TargetDistanceProbe { d: 0.0 })
        .insert(ItemContactProbe::default());

    if !false {
        entity_commands
            .insert(
                Thinker::build()
                    .picker(FirstToScore { threshold })
                    .when(PewIncoming::build(), DodgePew::build())
                    .when(LowHealth::build(), GotoMedikit::default())
                    .when(Fear::build().within(tune::FEAR_DISTANCE), RunAway {})
                    .when(
                        Curiousity::build().within(tune::CURIOSITY_DISTANCE),
                        Follow {
                            until: tune::FOLLOW_MIN_DISTANCE,
                        },
                    )
                    .otherwise(JiggleAround::default()),
            )
            .insert(HealthPoints { health: 50 });
    }
    if inspect_target {
        entity_commands.insert(AiInspectTarget);
    }
}

pub struct SpawnFerrisState {
    ferris_count: usize,
}

impl Default for SpawnFerrisState {
    fn default() -> Self {
        Self { ferris_count: 1 }
    }
}

pub fn spawn_brainy_ferris_system(
    mut commands: Commands,
    mut egui_context: ResMut<EguiContext>,
    mut state: Local<SpawnFerrisState>,
    mut query: Query<(Entity, &mut HealthPoints), With<Zappable>>,
    waypoints_query: Query<&Transform, With<Waypoint>>,
) {
    egui::Window::new("ferris").show(egui_context.ctx_mut(), |ui| {
        ui.add(egui::Slider::new(&mut state.ferris_count, 1..=100));
    });

    let count = query.iter().count();

    #[allow(clippy::comparison_chain)]
    match count.cmp(&state.ferris_count) {
        std::cmp::Ordering::Less => {
            let num_create = state.ferris_count - count;
            let waypoint_pos = waypoints_query
                .iter()
                .map(|transform| transform.translation)
                .collect::<Vec<_>>();

            if waypoint_pos.len() < num_create {
                return;
            }

            let mut rng = rand::thread_rng();

            for pos in waypoint_pos.choose_multiple(&mut rng, num_create) {
                // FIXME: hardcoded z offset is crap
                spawn_brainy_ferris(&mut commands, *pos + Vec3::Z * 5.0, false);
            }
        }

        std::cmp::Ordering::Greater => {
            let num_despawn = 10.min(count - state.ferris_count);

            for (entity, mut health_points) in query.iter_mut().take(num_despawn) {
                // commands.entity(entity).insert(Despawn::ThisFrame);
                health_points.health = 0;
            }
        }

        _ => (),
    }
}
