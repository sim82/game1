use bevy::prelude::*;
use bevy_aseprite::{AsepriteAnimation, AsepriteBundle};
use big_brain::prelude::*;
use rand::Rng;

use crate::{
    ai::{
        actions::{
            dodge_pew::DodgePew, follow::Follow, follow_path::FollowPath,
            goto_medikit::GotoMedikit, jiggle_around::JiggleAround, run_away::RunAway,
        },
        inspect::AiInspectTarget,
        scorers::{
            can_follow_path::CanFollowPath, curiosity::Curiousity, fear::Fear,
            health_low::LowHealth, pew_incoming::PewIncoming,
        },
        util::TargetDistanceProbe,
        HealthPoints,
    },
    movement::{crab_move::CrabMoveWalker, walk::VelocityWalker, zap::Zappable},
    sprites,
    ui::TrackingOverlayTarget,
};

mod tune {
    pub const FEAR_DISTANCE: f32 = 50.0;
    pub const CURIOSITY_DISTANCE: f32 = 150.0;
    pub const FOLLOW_MIN_DISTANCE: f32 = 16.0;
}

pub fn spawn_brainy_ferris(commands: &mut Commands, pos: Vec3, inspect_target: bool) {
    let mut rng = rand::thread_rng();
    let dist = rand_distr::Normal::new(0.8f32, 0.2f32).unwrap();

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
        .insert(
            Thinker::build()
                .picker(FirstToScore {
                    threshold: rng.sample(dist).clamp(0.0, 1.0),
                })
                .when(PewIncoming::build(), DodgePew::build())
                .when(LowHealth::build(), GotoMedikit::default())
                .when(CanFollowPath::default(), FollowPath::default())
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

    if inspect_target {
        entity_commands.insert(AiInspectTarget);
    }
}
