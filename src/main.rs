use bevy::{diagnostic::DiagnosticsPlugin, prelude::*};
// use bevy_aseprite::AsepritePlugin;
use bevy_aseprite::{anim::AsepriteAnimation, AsepriteBundle, AsepritePlugin};

use bevy_ecs_tilemap::TilemapPlugin;
use bevy_egui::EguiPlugin;
use big_brain::BigBrainPlugin;
use game1::{
    ai::{diagnostics::AiDiagnosticsPlugin, util::TargetDistanceProbe, AiPlugin, HealthPoints},
    brainy::spawn_brainy_ferris_system,
    die::die_system,
    exit_on_esc_system,
    hex::tilemap::HexTilemapPlugin,
    item::{ItemContactProbe, ItemPlugin},
    movement::{
        crab_move::{CrabMoveDirection, CrabMoveWalker},
        walk::VelocityWalker,
        zap::Zappable,
        MovementPlugin,
    },
    path::{PathPlugin, Waypoint},
    pointer::{ClickEvent, MousePointerFlag, PointerPlugin},
    sprites,
    tilemap::PlayfieldPlugin,
    tune,
    ui::IngameUiPlugin,
    Despawn, InputTarget, Pew, TargetFlag,
};
use rand::{thread_rng, Rng};

fn main() {
    let mut app = App::new();
    app.insert_resource(WindowDescriptor {
        ..Default::default()
    });
    //
    // external plugins
    //
    app.add_plugins(DefaultPlugins)
        .add_plugin(DiagnosticsPlugin)
        .add_plugin(TilemapPlugin)
        .add_plugin(AsepritePlugin)
        .add_plugin(BigBrainPlugin)
        .add_plugin(bevy_prototype_debug_lines::DebugLinesPlugin::with_depth_test(true))
        .add_plugin(EguiPlugin);
    //
    // internal plugins
    //
    app.add_plugin(PointerPlugin)
        .add_plugin(MovementPlugin)
        .add_plugin(AiPlugin)
        .add_plugin(PathPlugin)
        .add_plugin(AiDiagnosticsPlugin)
        .add_plugin(IngameUiPlugin)
        .add_plugin(PlayfieldPlugin)
        .add_plugin(ItemPlugin)
        .add_plugin(HexTilemapPlugin);

    //
    // startup systems
    //
    app.add_startup_system(setup);

    //
    // systems (mostly: TODO move to plugins)
    //
    app.add_system(setup_camera)
        .add_system(walk_to_target)
        .add_system(apply_input)
        .add_system(exit_on_esc_system)
        // .add_system(spawn_waypoint_on_click)
        .add_system(game1::pew_move_system)
        .add_system_to_stage(CoreStage::PostUpdate, game1::despawn_reaper_system)
        .add_system(die_system)
        .add_system(spawn_brainy_ferris_system)
        .add_system(spawn_player_system);
    //
    // type registrations
    //
    app.register_type::<VelocityWalker>();

    #[cfg(feature = "inspector")]
    {
        app.add_plugin(bevy_inspector_egui::WorldInspectorPlugin::new());
    }

    app.run();
}

pub fn spawn_stupid_ferris(commands: &mut Commands, asset_server: &AssetServer, pos: Vec3) {
    commands
        .spawn_bundle(AsepriteBundle {
            aseprite: asset_server.load(sprites::Ferris::PATH),
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

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(Camera2dBundle::default());

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
    #[allow(clippy::reversed_empty_ranges)]
    for i in 0..0 {
        // spawn_stupid_ferris(
        //     &mut commands,
        //     Vec3::new(rng.sample(dist), rng.sample(dist), 0.0),
        // );
        game1::brainy::spawn_brainy_ferris(
            &mut commands,
            &asset_server,
            Vec3::new(
                rng.sample(dist) + 600.0 / 4.0,
                rng.sample(dist) + 400.0 / 4.0,
                5.0,
            ),
            i == 0,
        );
    }

    // spawn_player(&mut commands, Vec3::new(40., 112., 5.));

    commands
        .spawn_bundle(AsepriteBundle {
            aseprite: asset_server.load(sprites::Pointer::PATH),
            // animation: AsepriteAnimation::from(sprites::Ferris::tags::WALK_RIGHT),
            transform: Transform {
                scale: Vec3::splat(1.),
                translation: Vec3::new(0., 100., 10.),
                ..Default::default()
            },

            ..Default::default()
        })
        .insert(MousePointerFlag);
}

fn spawn_player(commands: &mut Commands, asset_server: &AssetServer, translation: Vec3) {
    commands
        .spawn_bundle(AsepriteBundle {
            aseprite: asset_server.load(sprites::Ferris::PATH),
            animation: AsepriteAnimation::from(sprites::Ferris::tags::WALK_RIGHT),
            transform: Transform {
                scale: Vec3::splat(1.),
                translation,
                ..Default::default()
            },

            ..Default::default()
        })
        .insert(InputTarget)
        .insert(CrabMoveWalker::default())
        .insert(TargetFlag)
        .insert(HealthPoints { health: 60 })
        .insert(ItemContactProbe::default())
        .insert(Zappable);
}

fn setup_camera(
    windows: Res<Windows>,
    mut query: Query<(&mut Transform, &mut OrthographicProjection, &Camera), Added<Camera2d>>,
) {
    for (mut transform, _projection, camera) in query.iter_mut() {
        let window = windows.get_primary().unwrap();

        // let z = transform.translation.z;
        transform.translation.x = window.width() / 8.0;
        transform.translation.y = window.height() / 8.0;
        transform.scale.x = 0.25;
        transform.scale.y = 0.25;
    }
}

#[derive(Component)]
struct WalkToTarget;

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
    asset_server: Res<AssetServer>,
    mut query: Query<(&mut CrabMoveWalker, &Transform), With<InputTarget>>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    for (mut walk_velocity, transform) in query.iter_mut() {
        walk_velocity.direction = CrabMoveDirection::None;
        if keyboard_input.pressed(KeyCode::A) && keyboard_input.pressed(KeyCode::W) {
            walk_velocity.direction = CrabMoveDirection::NorthWest;
        } else if keyboard_input.pressed(KeyCode::A) && keyboard_input.pressed(KeyCode::S) {
            walk_velocity.direction = CrabMoveDirection::SouthWest;
        } else if keyboard_input.pressed(KeyCode::A) {
            walk_velocity.direction = CrabMoveDirection::West;
        } else if keyboard_input.pressed(KeyCode::D) && keyboard_input.pressed(KeyCode::W) {
            walk_velocity.direction = CrabMoveDirection::NorthEast;
        } else if keyboard_input.pressed(KeyCode::D) && keyboard_input.pressed(KeyCode::S) {
            walk_velocity.direction = CrabMoveDirection::SouthEast;
        } else if keyboard_input.pressed(KeyCode::D) {
            walk_velocity.direction = CrabMoveDirection::East;
        }

        if keyboard_input.just_pressed(KeyCode::J) && walk_velocity.direction.is_any() {
            let offset = if walk_velocity.direction.is_right() {
                Vec3::new(tune::PEW_ZAP_DISTANCE, 0.0, 0.0)
            } else {
                Vec3::new(-tune::PEW_ZAP_DISTANCE, 0.0, 0.0)
            };
            commands
                .spawn_bundle(AsepriteBundle {
                    aseprite: asset_server.load(sprites::Pew::PATH),
                    animation: AsepriteAnimation::from(sprites::Pew::tags::GLITTER),
                    transform: Transform {
                        scale: Vec3::splat(1.),
                        translation: transform.translation + offset,
                        ..Default::default()
                    },

                    ..Default::default()
                })
                .insert(Pew(walk_velocity.direction.is_right(), 15.0))
                .insert(Despawn::TimeToLive(10.0));
        }
        // else if keyboard_input.just_pressed(KeyCode::K) {
        //     commands
        //         .spawn_bundle(AsepriteBundle {
        //             aseprite: sprites::Pew::sprite(),
        //             animation: AsepriteAnimation::from(sprites::Pew::tags::GLITTER),
        //             transform: Transform {
        //                 scale: Vec3::splat(1.),
        //                 translation: transform.translation,
        //                 ..Default::default()
        //             },

        //             ..Default::default()
        //         })
        //         .insert(Pew(true))
        //         .insert(TimeToLive(10.0));
        // }
    }
}

pub fn spawn_ferris_on_click(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut click_events: EventReader<ClickEvent>,
) {
    for event in click_events.iter() {
        game1::brainy::spawn_brainy_ferris(&mut commands, &asset_server, event.pos, false);
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

pub fn spawn_player_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    query: Query<(), With<InputTarget>>,
) {
    if query.is_empty() {
        spawn_player(&mut commands, &asset_server, Vec3::new(40., 112., 5.));
    }
}
