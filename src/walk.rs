use crate::{pointer::MouseGrabState, sprites};
use bevy::{
    input::mouse::{MouseButtonInput, MouseMotion, MouseWheel},
    prelude::*,
};
use bevy_aseprite::{AsepriteAnimation, AsepriteBundle, AsepritePlugin};

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct VelocityWalker {
    pub velocity: Vec3,
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

pub struct WalkPlugin;

impl Plugin for WalkPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(apply_velocity);
    }
}
