use bevy::{diagnostic::DiagnosticsPlugin, input::system::exit_on_esc_system, prelude::*};
// use bevy_aseprite::AsepritePlugin;
use bevy_aseprite::{AsepriteAnimation, AsepriteBundle, AsepritePlugin};

use bevy_ecs_tilemap::TilemapPlugin;
use big_brain::BigBrainPlugin;
use game1::{
    ai::{diagnostics::AiDiagnosticsPlugin, util::TargetDistanceProbe, AiPlugin},
    movement::{
        crab_move::{self, CrabMoveWalker},
        walk::VelocityWalker,
        MovementPlugin,
    },
    path::{PathPlugin, Waypoint},
    pointer::{ClickEvent, MousePointerFlag, PointerPlugin},
    sprites,
    tilemap::PlayfieldPlugin,
    ui::IngameUiPlugin,
    Pew, TargetFlag, TimeToLive,
};
use rand::{thread_rng, Rng};

fn main() {
    App::new()
        //
        // external plugins
        //
        .add_plugins(DefaultPlugins)
        .add_plugin(DiagnosticsPlugin)
        .add_plugin(TilemapPlugin)
        .add_plugin(AsepritePlugin)
        .add_plugin(BigBrainPlugin)
        .add_plugin(bevy_inspector_egui::WorldInspectorPlugin::new())
        .add_plugin(bevy_prototype_debug_lines::DebugLinesPlugin::default())
        //
        // internal plugins
        //
        .add_plugin(PointerPlugin)
        .add_plugin(MovementPlugin)
        .add_plugin(AiPlugin)
        .add_plugin(PathPlugin)
        .add_plugin(AiDiagnosticsPlugin)
        .add_plugin(IngameUiPlugin)
        .add_plugin(PlayfieldPlugin)
        //
        // startup systems
        //
        .add_startup_system(setup)
        //
        // systems (mostly: TODO move to plugins)
        //
        .add_system(setup_camera)
        .add_system(walk_to_target)
        .add_system(apply_input)
        .add_system(exit_on_esc_system)
        .add_system(spawn_waypoint_on_click)
        .add_system(game1::pew_move_system)
        .add_system(game1::time_to_live_reaper_system)
        //
        // type registrations
        //
        .register_type::<VelocityWalker>()
        .run();
    println!("Hello, world!");
}

pub fn spawn_stupid_ferris(commands: &mut Commands, pos: Vec3) {
    commands
        .spawn_bundle(AsepriteBundle {
            aseprite: sprites::Ferris::sprite(),
            animation: AsepriteAnimation::from(sprites::Ferris::tags::WALK_RIGHT),
            transform: Transform {
                scale: Vec3::splat(1.),
                translation: pos,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(WalkToTarget)
        .insert(VelocityWalker {
            velocity: Vec3::ZERO,
        })
        .insert(TargetDistanceProbe { d: 0.0 });
}

pub fn setup(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    // commands
    //     .spawn_bundle(AsepriteBundle {
    //         aseprite: sprites::Ferris::sprite(),
    //         animation: AsepriteAnimation::from(sprites::Ferris::tags::WALK_RIGHT),
    //         transform: Transform {
    //             scale: Vec3::splat(4.),
    //             translation: Vec3::new(0., -200., 0.),
    //             ..Default::default()
    //         },
    //         ..Default::default()
    //     })
    //     .insert(WalkToTarget)
    //     .insert(VelocityWalker {
    //         velocity: Vec3::ZERO,
    //     })
    //     .insert(game1::brainy::TargetDistanceProbe { d: 0.0 });

    let mut rng = thread_rng();
    let dist = rand_distr::Normal::new(0.0f32, 50.0f32).unwrap();
    for _ in 0..10 {
        // spawn_stupid_ferris(
        //     &mut commands,
        //     Vec3::new(rng.sample(dist), rng.sample(dist), 0.0),
        // );
        game1::brainy::spawn_brainy_ferris(
            &mut commands,
            Vec3::new(
                rng.sample(dist) + 600.0 / 4.0,
                rng.sample(dist) + 400.0 / 4.0,
                5.0,
            ),
        );
    }

    commands
        .spawn_bundle(AsepriteBundle {
            aseprite: sprites::Ferris::sprite(),
            animation: AsepriteAnimation::from(sprites::Ferris::tags::WALK_RIGHT),
            transform: Transform {
                scale: Vec3::splat(1.),
                translation: Vec3::new(0., 100., 5.),
                ..Default::default()
            },

            ..Default::default()
        })
        .insert(InputTarget)
        .insert(CrabMoveWalker::default())
        .insert(TargetFlag);

    commands
        .spawn_bundle(AsepriteBundle {
            aseprite: sprites::Pointer::sprite(),
            // animation: AsepriteAnimation::from(sprites::Ferris::tags::WALK_RIGHT),
            transform: Transform {
                scale: Vec3::splat(1.),
                translation: Vec3::new(0., 100., 0.),
                ..Default::default()
            },

            ..Default::default()
        })
        .insert(MousePointerFlag);
}

fn setup_camera(mut query: Query<(&mut Transform, &mut OrthographicProjection), Added<Camera>>) {
    for (mut transform, _projection) in query.iter_mut() {
        // let z = transform.translation.z;
        transform.translation.x = 600.0 / 4.0;
        transform.translation.y = 400.0 / 4.0;
        transform.scale.x = 0.25;
        transform.scale.y = 0.25;
    }
}

#[derive(Component)]
struct WalkToTarget;

#[derive(Component)]
struct InputTarget;
fn walk_to_target(
    target_query: Query<&Transform, With<TargetFlag>>,
    mut query: Query<(&Transform, &mut VelocityWalker), With<WalkToTarget>>,
) {
    let mut target_pos = None;
    for transform in target_query.iter() {
        target_pos = Some(transform.translation);
    }

    if let Some(target_pos) = target_pos {
        for (transform, mut walk_velocity) in query.iter_mut() {
            let d = target_pos - transform.translation;
            let dist = d.length();
            if dist > 0.1 {
                let dir = d.normalize();
                walk_velocity.velocity = dir * 0.5;
            }
        }
    }
}

fn apply_input(
    mut commands: Commands,
    mut query: Query<(&mut CrabMoveWalker, &Transform), With<InputTarget>>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    for (mut walk_velocity, transform) in query.iter_mut() {
        walk_velocity.direction = crab_move::Direction::None;
        if keyboard_input.pressed(KeyCode::A) && keyboard_input.pressed(KeyCode::W) {
            walk_velocity.direction = crab_move::Direction::NorthWest;
        } else if keyboard_input.pressed(KeyCode::A) && keyboard_input.pressed(KeyCode::S) {
            walk_velocity.direction = crab_move::Direction::SouthWest;
        } else if keyboard_input.pressed(KeyCode::A) {
            walk_velocity.direction = crab_move::Direction::West;
        } else if keyboard_input.pressed(KeyCode::D) && keyboard_input.pressed(KeyCode::W) {
            walk_velocity.direction = crab_move::Direction::NorthEast;
        } else if keyboard_input.pressed(KeyCode::D) && keyboard_input.pressed(KeyCode::S) {
            walk_velocity.direction = crab_move::Direction::SouthEast;
        } else if keyboard_input.pressed(KeyCode::D) {
            walk_velocity.direction = crab_move::Direction::East;
        }
        if keyboard_input.just_pressed(KeyCode::J) {
            commands
                .spawn_bundle(AsepriteBundle {
                    aseprite: sprites::Pew::sprite(),
                    animation: AsepriteAnimation::from(sprites::Pew::tags::GLITTER),
                    transform: Transform {
                        scale: Vec3::splat(1.),
                        translation: transform.translation,
                        ..Default::default()
                    },

                    ..Default::default()
                })
                .insert(Pew(false))
                .insert(TimeToLive(10.0));
        } else if keyboard_input.just_pressed(KeyCode::K) {
            commands
                .spawn_bundle(AsepriteBundle {
                    aseprite: sprites::Pew::sprite(),
                    animation: AsepriteAnimation::from(sprites::Pew::tags::GLITTER),
                    transform: Transform {
                        scale: Vec3::splat(1.),
                        translation: transform.translation,
                        ..Default::default()
                    },

                    ..Default::default()
                })
                .insert(Pew(true))
                .insert(TimeToLive(10.0));
        }
    }
}

pub fn spawn_ferris_on_click(mut commands: Commands, mut click_events: EventReader<ClickEvent>) {
    for event in click_events.iter() {
        game1::brainy::spawn_brainy_ferris(&mut commands, event.pos);
    }
}

pub fn spawn_waypoint_on_click(mut commands: Commands, mut click_events: EventReader<ClickEvent>) {
    for event in click_events.iter() {
        commands
            .spawn()
            .insert(Waypoint)
            .insert(Transform::from_translation(event.pos));
    }
}
