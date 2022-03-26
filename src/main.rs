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

mod sprites {
    use bevy_aseprite::aseprite;
    aseprite!(pub Ferris, "assets/ferris2.0.aseprite");
    aseprite!(pub Pointer, "assets/pointer.aseprite");
}

struct MouseGrabState {
    shall_grab: bool,
    known_state: bool,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(AsepritePlugin)
        .add_plugin(WorldInspectorPlugin::new())
        .add_startup_system(setup)
        .add_system(walk_to_target)
        .add_system(apply_velocity)
        .add_system(apply_input)
        .add_system(exit_on_esc_system)
        .add_system(mouse_grab_system)
        .insert_resource(MouseGrabState {
            shall_grab: true,
            known_state: false,
        })
        .add_system(mouse_input_system)
        // .add_system(draw_mouse_pointer)
        .init_resource::<PrimaryPointerPos>()
        .register_type::<VelocityWalker>()
        .run();
    println!("Hello, world!");
}
fn mouse_grab_system(
    mut grab_state: ResMut<MouseGrabState>,
    mut windows: ResMut<Windows>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    let update = if keyboard_input.just_pressed(KeyCode::Grave) {
        grab_state.shall_grab = !grab_state.shall_grab;
        true
    } else {
        false
    };

    if update || !grab_state.known_state {
        grab_state.known_state = true;

        let window = windows.get_primary_mut().unwrap();

        if window.cursor_locked() != grab_state.shall_grab {
            window.set_cursor_lock_mode(grab_state.shall_grab);
            window.set_cursor_visibility(!grab_state.shall_grab);
        }
    }
}
pub fn setup(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

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
        .insert(VelocityWalker {
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
struct TargetFlag;

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
struct VelocityWalker {
    pub velocity: Vec3,
}

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
    mut query: Query<(&mut VelocityWalker), With<InputTarget>>,
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
    }
}

fn apply_velocity(
    mut query: Query<(
        Entity,
        &mut Transform,
        &mut AsepriteAnimation,
        &VelocityWalker,
    )>,
    mut grab_state: ResMut<MouseGrabState>,
) {
    if !grab_state.shall_grab {
        return;
    }

    for (entity, mut transform, mut animation, walk_velocity) in query.iter_mut() {
        let speed = walk_velocity.velocity.length();

        debug!(
            "walk: {:?} {:?} {:?}",
            entity, transform.translation, walk_velocity.velocity
        );
        if speed > 0.1 {
            let dir = walk_velocity.velocity.normalize();
            transform.translation += walk_velocity.velocity;
            // animation.
            if dir.x > 0.0 && !animation.is_tag(sprites::Ferris::tags::WALK_RIGHT) {
                *animation = AsepriteAnimation::from(sprites::Ferris::tags::WALK_RIGHT);
            } else if dir.x < 0.0 && !animation.is_tag(sprites::Ferris::tags::WALK_LEFT) {
                *animation = AsepriteAnimation::from(sprites::Ferris::tags::WALK_LEFT);
            } else if (dir.x == 0.0 && dir.y != 0.0)
                && !animation.is_tag(sprites::Ferris::tags::WALK_CENTER)
            {
                *animation = AsepriteAnimation::from(sprites::Ferris::tags::WALK_CENTER);
            }
        } else if !animation.is_tag(sprites::Ferris::tags::STAND) {
            *animation = AsepriteAnimation::from(sprites::Ferris::tags::STAND);
        }
    }
}

#[derive(Default)]
struct PrimaryPointerPos {
    pos: Vec3,
}

#[derive(Component)]
struct MousePointerFlag;

fn mouse_input_system(
    mut commands: Commands,
    mut query: Query<&mut Transform, With<MousePointerFlag>>,
    mut mouse_button_input_events: EventReader<MouseButtonInput>,
    mut mouse_motion_events: EventReader<MouseMotion>,
    mut _cursor_moved_events: EventReader<CursorMoved>,
    mut _mouse_wheel_events: EventReader<MouseWheel>,
    mut primary_pointer: ResMut<PrimaryPointerPos>,
    grab_state: Res<MouseGrabState>,
) {
    if !grab_state.shall_grab {
        return;
    }

    for mut transform in query.iter_mut() {
        for event in mouse_motion_events.iter() {
            let d = Vec3::new(event.delta.x, -event.delta.y, 0.0);
            transform.translation += d * 0.5;
        }
        primary_pointer.pos = transform.translation;
    }
    for event in mouse_button_input_events.iter() {
        if event.button == MouseButton::Left && event.state.is_pressed() {
            info!("pressed");
            commands
                .spawn_bundle(AsepriteBundle {
                    aseprite: sprites::Ferris::sprite(),
                    animation: AsepriteAnimation::from(sprites::Ferris::tags::WALK_RIGHT),
                    transform: Transform {
                        scale: Vec3::splat(4.),
                        translation: primary_pointer.pos,
                        ..Default::default()
                    },

                    ..Default::default()
                })
                .insert(WalkToTarget)
                .insert(VelocityWalker {
                    velocity: Vec3::ZERO,
                });
        }
    }
}
