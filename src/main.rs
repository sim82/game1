use bevy::{
    input::{
        mouse::{MouseButtonInput, MouseMotion, MouseWheel},
        system::exit_on_esc_system,
    },
    prelude::*,
};
// use bevy_aseprite::AsepritePlugin;
use bevy_aseprite::{AsepriteAnimation, AsepriteBundle, AsepritePlugin};
use bevy_inspector_egui::WorldInspectorPlugin;
use game1::{
    brainy::BrainyPlugin,
    pointer::{ClickEvent, MouseGrabState, MousePointerFlag, PointerPlugin},
    sprites,
    walk::{VelocityWalker, WalkPlugin},
    TargetFlag,
};
use rand::{thread_rng, Rng};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(AsepritePlugin)
        // .add_plugin(WorldInspectorPlugin::new())
        .add_plugin(PointerPlugin)
        .add_plugin(WalkPlugin)
        .add_plugin(BrainyPlugin)
        .add_startup_system(setup)
        .add_system(walk_to_target)
        .add_system(apply_input)
        .add_system(exit_on_esc_system)
        .add_system(spawn_ferris_on_click)
        .register_type::<VelocityWalker>()
        .run();
    println!("Hello, world!");
}

fn spawn_stupid_ferris(commands: &mut Commands, pos: Vec3) {
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
        .insert(WalkToTarget)
        .insert(VelocityWalker {
            velocity: Vec3::ZERO,
        })
        .insert(game1::brainy::TargetDistanceProbe { d: 0.0 });
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
    let dist = rand_distr::Normal::new(0.0f32, 200.0f32).unwrap();
    for _ in 0..100000 {
        // spawn_stupid_ferris(
        //     &mut commands,
        //     Vec3::new(rng.sample(dist), rng.sample(dist), 0.0),
        // );
        game1::brainy::spawn_brainy_ferris(
            &mut commands,
            Vec3::new(rng.sample(dist), rng.sample(dist), 0.0),
        );
    }

    commands
        .spawn_bundle(AsepriteBundle {
            aseprite: sprites::Ferris::sprite(),
            animation: AsepriteAnimation::from(sprites::Ferris::tags::WALK_RIGHT),
            transform: Transform {
                scale: Vec3::splat(4.),
                translation: Vec3::new(0., -100., 0.),
                ..Default::default()
            },

            ..Default::default()
        })
        .insert(InputTarget)
        .insert(VelocityWalker {
            velocity: Vec3::ZERO,
        })
        .insert(TargetFlag);

    commands
        .spawn_bundle(AsepriteBundle {
            aseprite: sprites::Pointer::sprite(),
            // animation: AsepriteAnimation::from(sprites::Ferris::tags::WALK_RIGHT),
            transform: Transform {
                scale: Vec3::splat(4.),
                translation: Vec3::new(0., -100., 0.),
                ..Default::default()
            },

            ..Default::default()
        })
        .insert(MousePointerFlag);
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
    mut query: Query<&mut VelocityWalker, With<InputTarget>>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    for mut walk_velocity in query.iter_mut() {
        walk_velocity.velocity = Vec3::ZERO;
        if keyboard_input.pressed(KeyCode::A) {
            walk_velocity.velocity.x = -1.0;
        }
        if keyboard_input.pressed(KeyCode::D) {
            walk_velocity.velocity.x = 1.0;
        }
        if keyboard_input.pressed(KeyCode::W) {
            walk_velocity.velocity.y = 1.0;
        }
        if keyboard_input.pressed(KeyCode::S) {
            walk_velocity.velocity.y = -1.0;
        }
        walk_velocity.velocity = walk_velocity.velocity.normalize();
    }
}

pub fn spawn_ferris_on_click(mut commands: Commands, mut click_events: EventReader<ClickEvent>) {
    for event in click_events.iter() {
        game1::brainy::spawn_brainy_ferris(&mut commands, event.pos);
    }
}
