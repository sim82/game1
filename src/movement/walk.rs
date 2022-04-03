use crate::{pointer::MouseGrabState, sprites, tune, Pew};
use bevy::prelude::*;
use bevy_aseprite::AsepriteAnimation;

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct VelocityWalker {
    pub velocity: Vec3,
}

#[derive(Component)]
#[component(storage = "SparseSet")]
pub struct BeingZapped;

#[allow(clippy::type_complexity)]
fn check_pew_intersection_system(
    mut commands: Commands,
    query_non_zapped: Query<
        (Entity, &mut Transform),
        (With<VelocityWalker>, Without<BeingZapped>, Without<Pew>),
    >,
    query_zapped: Query<
        (Entity, &mut Transform),
        (With<VelocityWalker>, With<BeingZapped>, Without<Pew>),
    >,
    query_pew: Query<&Transform, With<Pew>>,
) {
    let pew_pos = query_pew
        .iter()
        .map(|transform| transform.translation)
        .collect::<Vec<_>>();

    const ZAP_DIST: f32 = tune::PEW_ZAP_DISTANCE;

    for (entity, Transform { translation, .. }) in query_non_zapped.iter() {
        if pew_pos
            .iter()
            .any(|pew| (*pew - *translation).length() < ZAP_DIST)
        {
            commands.entity(entity).insert(BeingZapped);
        }
    }
    for (entity, Transform { translation, .. }) in query_zapped.iter() {
        if !pew_pos
            .iter()
            .any(|pew| (*pew - *translation).length() < ZAP_DIST)
        {
            commands.entity(entity).remove::<BeingZapped>();
        }
    }
}

fn apply_velocity_system(
    time: Res<Time>,
    mut query: Query<(
        Entity,
        &mut Transform,
        &mut AsepriteAnimation,
        &VelocityWalker,
    )>,
    mut zapped_query: Query<Entity, With<BeingZapped>>,
    grab_state: ResMut<MouseGrabState>,
) {
    if !grab_state.shall_grab {
        return;
    }

    for (entity, mut transform, mut animation, walk_velocity) in query.iter_mut() {
        if zapped_query.get(entity).is_ok() {
            if !animation.is_tag(sprites::Ferris::tags::ZAP) {
                *animation = AsepriteAnimation::from(sprites::Ferris::tags::ZAP)
            }
            continue;
        }

        let speed = walk_velocity.velocity.length();

        debug!(
            "walk: {:?} {:?} {:?}",
            entity, transform.translation, walk_velocity.velocity
        );
        if speed > 0.1 {
            let dir = walk_velocity.velocity.normalize();
            transform.translation += tune::WALK_SPEED * dir * time.delta_seconds();
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

pub struct WalkPlugin;

impl Plugin for WalkPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(check_pew_intersection_system)
            .add_system(apply_velocity_system);
    }
}
