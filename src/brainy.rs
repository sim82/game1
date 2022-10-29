use bevy::prelude::*;
use bevy_aseprite::{anim::AsepriteAnimation, AsepriteBundle};
use bevy_egui::{egui, EguiContext};
use big_brain::prelude::*;
use rand::{prelude::SliceRandom, Rng};

use crate::{
    ai::{
        actions::{
            dodge_pew::DodgePew,
            follow::Follow,
            go_direction::ActionGoDirection,
            go_script::ActionGoScript,
            goto_medikit::GotoMedikit,
            goto_pos::ActionGotoPos,
            jiggle_around::JiggleAround,
            pick_goto_pos::{ActionPickGotoPos, TargetPos},
            run_away::RunAway,
            shoot::Shoot,
            wait::ActionWait,
        },
        inspect::AiInspectTarget,
        scorers::{
            can_shoot::CanShoot,
            crowdiness::Crowdiness,
            curiosity::Curiousity,
            fear::Fear,
            health_low::{LowHealth, LowHealthBuilder},
            pew_incoming::PewIncoming,
        },
        util::{Ammo, TargetDistanceProbe},
        HealthPoints,
    },
    item::ItemContactProbe,
    movement::{
        crab_move::{CrabMoveDirection, CrabMoveWalker},
        zap::Zappable,
    },
    path::Waypoint,
    sprites,
};

mod tune {
    pub const FEAR_DISTANCE: f32 = 50.0;
    pub const CURIOSITY_DISTANCE: f32 = 100.0;
    pub const FOLLOW_MIN_DISTANCE: f32 = 16.0;
}

fn new_brainy_thinker() -> ThinkerBuilder {
    let idle_steps = Steps::build()
        .step(ActionPickGotoPos::new(TargetPos::Random))
        .step(ActionGotoPos::default())
        .step(ActionGoScript::repeat(
            &[
                (CrabMoveDirection::East, 0.2),
                (CrabMoveDirection::West, 0.2),
            ],
            5..10,
        ));

    let find_medikit_steps = Steps::build()
        .step(ActionPickGotoPos::new(TargetPos::Medikit))
        .step(ActionGotoPos::default());

    Thinker::build()
        .picker(Highest)
        .when(LowHealth::build(), find_medikit_steps)
        .when(PewIncoming::build(), DodgePew::build())
        .when(FixedScore(0.5), idle_steps)
}

pub fn spawn_brainy_ferris(
    commands: &mut Commands,
    asset_server: &AssetServer,
    pos: Vec3,
    inspect_target: bool,
) {
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
        aseprite: asset_server.load(sprites::Ferris::PATH),
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
    let entity = entity_commands.id();
    entity_commands
        .insert(Zappable)
        .insert(CrabMoveWalker::default())
        .insert(TargetDistanceProbe { d: 0.0 })
        .insert(ItemContactProbe::default())
        .insert(Ammo::default())
        .insert(Name::new(format!("brainy#{}", entity.id())));

    if true {
        entity_commands
            // .insert(
            //     Thinker::build()
            //         .picker(FirstToScore { threshold })
            //         .when(PewIncoming::build(), DodgePew::build())
            //         .when(CanShoot::default(), Shoot::default())
            //         .when(LowHealth::build(), GotoMedikit::default())
            //         // .when(Fear::build().within(tune::FEAR_DISTANCE), RunAway {})
            //         // .when(Crowdiness::default(), DodgePew::build())
            //         // .when(
            //         //     Curiousity::build().within(tune::CURIOSITY_DISTANCE),
            //         //     Follow {
            //         //         until: tune::FOLLOW_MIN_DISTANCE,
            //         //     },
            //         // )
            //         .otherwise(JiggleAround::default()),
            // )
            .insert(new_brainy_thinker())
            .insert(HealthPoints { health: 50 });
    }
    if inspect_target {
        entity_commands.insert(AiInspectTarget);
    }
}

pub struct SpawnFerrisState {
    ferris_count: usize,
    next_increase: f32,
}

impl Default for SpawnFerrisState {
    fn default() -> Self {
        Self {
            ferris_count: 1,
            next_increase: 15.0,
        }
    }
}

pub fn spawn_brainy_ferris_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    time: Res<Time>,
    mut egui_context: ResMut<EguiContext>,
    mut state: Local<SpawnFerrisState>,
    mut query: Query<(Entity, &mut HealthPoints), With<ThinkerBuilder>>,
    waypoints_query: Query<&Transform, With<Waypoint>>,
) {
    if false {
        state.next_increase -= time.delta_seconds();
        if state.next_increase <= 0.0 {
            state.next_increase = 15.0;
            state.ferris_count += 1;
        }
    }

    egui::Window::new("ferris").show(egui_context.ctx_mut(), |ui| {
        ui.add(egui::Slider::new(&mut state.ferris_count, 1..=100));
    });

    let count = query.iter().count();
    let mut first = count == 0;
    // info!("count: {} {}", count, state.ferris_count);
    #[allow(clippy::comparison_chain)]
    match count.cmp(&state.ferris_count) {
        std::cmp::Ordering::Less => {
            let num_create = state.ferris_count - count;
            let waypoint_pos = waypoints_query
                .iter()
                .map(|transform| transform.translation)
                .collect::<Vec<_>>();

            // if waypoint_pos.len() < num_create {
            //     return;
            // }

            let num_create = num_create.min(waypoint_pos.len());

            let mut rng = rand::thread_rng();

            for pos in waypoint_pos.choose_multiple(&mut rng, num_create) {
                // FIXME: hardcoded z offset is crap
                spawn_brainy_ferris(&mut commands, &asset_server, *pos + Vec3::Z * 5.0, first);
                first = false;
            }
        }

        std::cmp::Ordering::Greater => {
            let num_despawn = 1000.min(count - state.ferris_count);

            for (_entity, mut health_points) in query.iter_mut().take(num_despawn) {
                // commands.entity(entity).insert(Despawn::ThisFrame);
                health_points.health = 0;
            }
        }

        _ => (),
    }
}
