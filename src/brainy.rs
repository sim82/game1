use bevy::prelude::*;
use bevy_aseprite::{AsepriteAnimation, AsepriteBundle};
use big_brain::prelude::*;
use rand::Rng;

use crate::{
    ai::{
        actions::{follow::Follow, jiggle_around::JiggleAround, run_away::RunAway},
        scorers::{curiosity::Curiousity, fear::Fear},
        util::TargetDistanceProbe,
    },
    sprites,
    walk::VelocityWalker,
};

pub fn spawn_brainy_ferris(commands: &mut Commands, pos: Vec3) {
    let mut rng = rand::thread_rng();
    let dist = rand_distr::Normal::new(128.0f32, 32.0f32).unwrap();

    let curious_min = rng.sample(dist);

    let _curious_max = curious_min + rng.sample(dist);

    commands
        .spawn_bundle(AsepriteBundle {
            aseprite: sprites::Ferris::sprite(),
            animation: AsepriteAnimation::from(sprites::Ferris::tags::WALK_RIGHT),
            transform: Transform {
                scale: Vec3::splat(4.),
                translation: pos,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(VelocityWalker {
            velocity: Vec3::ZERO,
        })
        .insert(TargetDistanceProbe { d: 0.0 })
        .insert(
            Thinker::build()
                .picker(FirstToScore { threshold: 0.8 })
                // Technically these are supposed to be ActionBuilders and
                // ScorerBuilders, but our Clone impls simplify our code here.
                .when(
                    Fear::build().within(72.0),
                    RunAway {
                        until: rng.sample(dist),
                    },
                )
                .when(
                    Curiousity::build().within(256.0),
                    Follow { until: curious_min },
                )
                .otherwise(JiggleAround::default()),
        );
}
