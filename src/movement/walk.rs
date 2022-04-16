use crate::{pointer::MouseGrabState, sprites, tune};
use bevy::prelude::*;
use bevy_aseprite::AsepriteAnimation;

use super::zap::BeingZapped;

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct VelocityWalker {
    pub velocity: Vec3,
}

pub fn apply_velocity_system(
    time: Res<Time>,
    mut query: Query<(
        Entity,
        &mut Transform,
        &mut AsepriteAnimation,
        &VelocityWalker,
    )>,
    zapped_query: Query<Entity, With<BeingZapped>>,
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
