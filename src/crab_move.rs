use crate::{pointer::MouseGrabState, sprites};
use bevy::prelude::*;
use bevy_aseprite::AsepriteAnimation;

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    None,
    West,
    NorthWest,
    NorthEast,
    East,
    SouthEast,
    SouthWest,
}
impl Default for Direction {
    fn default() -> Self {
        Direction::None
    }
}

#[derive(Component, Default)]
// #[reflect(Component)]
pub struct CrabMoveWalker {
    pub direction: Direction,
}

impl Direction {
    pub fn to_vec3(self) -> Vec3 {
        match self {
            Direction::None => Vec3::ZERO,
            Direction::West => Vec3::new(-1.0, 0.0, 0.0),
            Direction::NorthWest => Vec3::new(-1.0, 1.0, 0.0),
            Direction::NorthEast => Vec3::new(1.0, 1.0, 0.0),
            Direction::East => Vec3::new(1.0, 0.0, 0.0),
            Direction::SouthEast => Vec3::new(1.0, -1.0, 0.0),
            Direction::SouthWest => Vec3::new(-1.0, -1.0, 0.0),
        }
    }
}

fn apply_velocity(
    mut query: Query<(
        Entity,
        &mut Transform,
        &mut AsepriteAnimation,
        &CrabMoveWalker,
    )>,
    grab_state: ResMut<MouseGrabState>,
) {
    if !grab_state.shall_grab {
        return;
    }

    for (entity, mut transform, mut animation, walk_velocity) in query.iter_mut() {
        let velocity = walk_velocity.direction.to_vec3();
        let speed = velocity.length();

        debug!(
            "walk: {:?} {:?} {:?}",
            entity, transform.translation, velocity
        );
        if speed > 0.1 {
            let dir = velocity.normalize();
            transform.translation += velocity;
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

pub struct CrabMovePlugin;

impl Plugin for CrabMovePlugin {
    fn build(&self, app: &mut App) {
        app.add_system(apply_velocity);
    }
}
