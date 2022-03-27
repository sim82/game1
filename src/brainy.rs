use bevy::prelude::*;
use bevy_aseprite::{AsepriteAnimation, AsepriteBundle};
use big_brain::prelude::*;

use crate::{sprites, walk::VelocityWalker, TargetFlag};

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
    let target_pos = target_query
        .iter()
        .next()
        .map(|t| t.translation)
        .unwrap_or_default();

    for (mut probe, transform) in query.iter_mut() {
        probe.d = (target_pos - transform.translation).length();
    }
}

#[derive(Clone, Component, Debug)]
pub struct RunAway {
    until: f32,
}

// Action systems execute according to a state machine, where the states are
// labeled by ActionState.
fn run_away_action_system(
    mut walkers: Query<(&Transform, &TargetDistanceProbe, &mut VelocityWalker)>,
    target_query: Query<&Transform, With<TargetFlag>>,
    // We execute actions by querying for their associated Action Component
    // (Drink in this case). You'll always need both Actor and ActionState.
    mut query: Query<(&Actor, &mut ActionState, &RunAway)>,
) {
    let target_pos = target_query
        .iter()
        .next()
        .map(|t| t.translation)
        .unwrap_or_default();

    for (Actor(actor), mut state, run_away) in query.iter_mut() {
        // Use the drink_action's actor to look up the corresponding Thirst Component.
        if let Ok((transform, target_distance, mut walker)) = walkers.get_mut(*actor) {
            match *state {
                ActionState::Requested => {
                    println!("Time to run away!");
                    // let tv = (target_pos - transform.translation).normalize();
                    // walker.velocity = -0.5 * tv;

                    *state = ActionState::Executing;
                }
                ActionState::Executing => {
                    if target_distance.d <= run_away.until {
                        let tv = (target_pos - transform.translation).normalize();
                        walker.velocity = -1.0 * tv;
                        info!("walk_velocity: {:?}", walker.velocity);
                    } else {
                        walker.velocity = Vec3::ZERO;
                        *state = ActionState::Success;
                    }
                }
                // All Actions should make sure to handle cancellations!
                ActionState::Cancelled => {
                    *state = ActionState::Failure;
                }
                _ => {}
            }
        }
    }
}

#[derive(Clone, Component, Debug)]
pub struct Fearful;

// Looks familiar? It's a lot like Actions!
pub fn fear_scorer_system(
    target_distance: Query<&TargetDistanceProbe>,
    // Same dance with the Actor here, but now we use look up Score instead of ActionState.
    mut query: Query<(&Actor, &mut Score), With<Fearful>>,
) {
    for (Actor(actor), mut score) in query.iter_mut() {
        if let Ok(target_distance) = target_distance.get(*actor) {
            // This is really what the job of a Scorer is. To calculate a
            // generic "Utility" score that the Big Brain engine will compare
            // against others, over time, and use to make decisions. This is
            // generally "the higher the better", and "first across the finish
            // line", but that's all configurable using Pickers!
            //
            // The score here must be between 0.0 and 1.0.
            if target_distance.d < 96.0 {
                score.set(1.0)
            } else {
                score.set(0.0)
            }
        }
    }
}

fn add_brainy_system(mut commands: Commands) {
    commands
        .spawn_bundle(AsepriteBundle {
            aseprite: sprites::Ferris::sprite(),
            animation: AsepriteAnimation::from(sprites::Ferris::tags::WALK_RIGHT),
            transform: Transform {
                scale: Vec3::splat(4.),
                translation: Vec3::new(0., -200., 0.),
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
                .when(Fearful, RunAway { until: 128.0 }),
        );
}

impl Plugin for BrainyPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(BigBrainPlugin)
            .register_type::<TargetDistanceProbe>()
            .add_system(measure_target_distance)
            .add_system_to_stage(BigBrainStage::Actions, run_away_action_system)
            .add_system_to_stage(BigBrainStage::Scorers, fear_scorer_system)
            .add_startup_system(add_brainy_system);
    }
}
