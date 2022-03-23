use bevy::prelude::*;
// use bevy_aseprite::AsepritePlugin;
use bevy_aseprite::{AsepriteAnimation, AsepriteAnimationState, AsepriteBundle, AsepritePlugin};

mod sprites {
    use bevy_aseprite::aseprite;
    aseprite!(pub Ferris, "assets/ferris2.0.aseprite");
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(AsepritePlugin)
        .add_startup_system(setup)
        .insert_resource(TargetPos {
            pos: Vec3::new(200.0, 100.0, 0.0),
        })
        .add_system(walk_to_target)
        .add_system(apply_velocity)
        .add_system(apply_input)
        .run();
    println!("Hello, world!");
}

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    // commands.spawn_bundle(SpriteBundle {
    //     texture: asset_server.load("ferris2.0.aseprite"),
    //     // texture: asset_server.load("ferris2.0.png"),
    //     transform: Transform::from_xyz(100., 0., 0.),
    //     ..Default::default()
    // });

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
        .insert(WalkToTarget)
        .insert(WalkVelocity {
            velocity: Vec3::ZERO,
        });

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
        .insert(WalkVelocity {
            velocity: Vec3::ZERO,
        });
}

#[derive(Component)]
struct WalkToTarget;

#[derive(Component)]
struct TargetPos {
    pub pos: Vec3,
}
#[derive(Component)]
struct WalkVelocity {
    pub velocity: Vec3,
}

#[derive(Component)]
struct InputTarget;
fn walk_to_target(
    mut query: Query<(&Transform, &mut WalkVelocity), With<WalkToTarget>>,
    target_pos: Res<TargetPos>,
) {
    for (transform, mut walk_velocity) in query.iter_mut() {
        let d = target_pos.pos - transform.translation;
        let dist = d.length();
        if dist > 0.1 {
            let dir = d.normalize();
            walk_velocity.velocity = dir;
        }
    }
}

fn apply_velocity(mut query: Query<(&mut Transform, &mut AsepriteAnimation, &WalkVelocity)>) {
    for (mut transform, mut animation, walk_velocity) in query.iter_mut() {
        let speed = walk_velocity.velocity.length();
        info!("speed: {}", speed);
        if speed > 0.1 {
            let dir = walk_velocity.velocity.normalize();
            transform.translation += dir;
            // animation.
            if dir.x > 0.0 && !animation.is_tag(sprites::Ferris::tags::WALK_RIGHT) {
                *animation = AsepriteAnimation::from(sprites::Ferris::tags::WALK_RIGHT);
            } else if dir.x < 0.0 && !animation.is_tag(sprites::Ferris::tags::WALK_LEFT) {
                *animation = AsepriteAnimation::from(sprites::Ferris::tags::WALK_LEFT);
            }
        } else if !animation.is_tag(sprites::Ferris::tags::STAND) {
            *animation = AsepriteAnimation::from(sprites::Ferris::tags::STAND);
        }
    }
}

fn apply_input(
    mut query: Query<(&mut WalkVelocity), With<InputTarget>>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    for mut walk_velocity in query.iter_mut() {
        if keyboard_input.pressed(KeyCode::A) {
            walk_velocity.velocity.x = -1.0;
        }
        if keyboard_input.pressed(KeyCode::D) {
            walk_velocity.velocity.x = 1.0;
        }
        if keyboard_input.pressed(KeyCode::W) {
            walk_velocity.velocity.y = -1.0;
        }
        if keyboard_input.pressed(KeyCode::S) {
            walk_velocity.velocity.y = 1.0;
        }
    }
}
