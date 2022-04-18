use crate::{
    pointer::MouseGrabState,
    sprites,
    tilemap::{hex_neighbors, pixel_to_pointy_hex, pointy_hex_to_aabb},
    tune,
};
use bevy::{prelude::*, sprite::collide_aabb::collide};
use bevy_aseprite::AsepriteAnimation;
use bevy_ecs_tilemap::{MapQuery, Tile, TilePos};

use super::zap::BeingZapped;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CrabMoveDirection {
    None,
    West,
    NorthWest,
    NorthEast,
    East,
    SouthEast,
    SouthWest,
}
impl Default for CrabMoveDirection {
    fn default() -> Self {
        CrabMoveDirection::None
    }
}

#[derive(Component, Default)]
// #[reflect(Component)]
pub struct CrabMoveWalker {
    pub direction: CrabMoveDirection,
}

const HEX_DIAG_X: f32 = 0.5;
const HEX_DIAG_Y: f32 = 0.866; // sqrt(3) / 2 or sin(60)

impl CrabMoveDirection {
    pub fn to_vec3(self) -> Vec3 {
        // 'diagonal' directions on hex grid

        match self {
            // CrabMoveDirection::None => Vec3::ZERO,
            // CrabMoveDirection::West => Vec3::new(-1.0, 0.0, 0.0),
            // CrabMoveDirection::NorthWest => Vec3::new(-1.0, 1.0, 0.0),
            // CrabMoveDirection::NorthEast => Vec3::new(1.0, 1.0, 0.0),
            // CrabMoveDirection::East => Vec3::new(1.0, 0.0, 0.0),
            // CrabMoveDirection::SouthEast => Vec3::new(1.0, -1.0, 0.0),
            // CrabMoveDirection::SouthWest => Vec3::new(-1.0, -1.0, 0.0),
            CrabMoveDirection::None => Vec3::ZERO,
            CrabMoveDirection::West => Vec3::new(-1.0, 0.0, 0.0),
            CrabMoveDirection::NorthWest => Vec3::new(-HEX_DIAG_X, HEX_DIAG_Y, 0.0),
            CrabMoveDirection::NorthEast => Vec3::new(HEX_DIAG_X, HEX_DIAG_Y, 0.0),
            CrabMoveDirection::East => Vec3::new(1.0, 0.0, 0.0),
            CrabMoveDirection::SouthEast => Vec3::new(HEX_DIAG_X, -HEX_DIAG_Y, 0.0),
            CrabMoveDirection::SouthWest => Vec3::new(-HEX_DIAG_X, -HEX_DIAG_Y, 0.0),
        }
    }
    pub fn find_nearest(dir: Vec3) -> Self {
        fn to_positive_degrees(dir: Vec3) -> u32 {
            let deg = f32::atan2(dir.y, dir.x).to_degrees() as i32;
            if deg >= 0 {
                deg as u32
            } else {
                (360 + deg) as u32
            }
        }
        match to_positive_degrees(dir) {
            0..30 | 330..360 => CrabMoveDirection::East,
            30..90 => CrabMoveDirection::NorthEast,
            90..150 => CrabMoveDirection::NorthWest,
            150..210 => CrabMoveDirection::West,
            210..270 => CrabMoveDirection::SouthWest,
            270..330 => CrabMoveDirection::SouthEast,
            _ => CrabMoveDirection::None,
        }
    }
    pub fn is_none(&self) -> bool {
        *self == CrabMoveDirection::None
    }
    pub fn is_any(&self) -> bool {
        !self.is_none()
    }
    pub fn is_right(&self) -> bool {
        matches!(
            self,
            CrabMoveDirection::East | CrabMoveDirection::NorthEast | CrabMoveDirection::SouthEast
        )
    }
}

pub fn apply_velocity_system(
    time: Res<Time>,
    mut query: Query<(
        Entity,
        &mut Transform,
        &mut AsepriteAnimation,
        &CrabMoveWalker,
    )>,
    zapped_query: Query<Entity, With<BeingZapped>>,
    mut map_query: MapQuery,
    tile_query: Query<&Tile>,
    grab_state: ResMut<MouseGrabState>,
) {
    if !grab_state.shall_grab {
        return;
    }

    // map_query.

    for (entity, mut transform, mut animation, walk_velocity) in query.iter_mut() {
        if zapped_query.get(entity).is_ok() {
            if !animation.is_tag(sprites::Ferris::tags::ZAP) {
                *animation = AsepriteAnimation::from(sprites::Ferris::tags::ZAP)
            }
            continue;
        }

        let velocity = walk_velocity.direction.to_vec3();
        let speed = velocity.length();

        debug!(
            "walk: {:?} {:?} {:?}",
            entity, transform.translation, velocity
        );
        if speed > 0.1 {
            let dir = velocity.normalize();
            let delta = tune::WALK_SPEED * dir * time.delta_seconds();
            let x_delta = Vec3::new(delta.x, 0.0, 0.0);
            let y_delta = Vec3::new(0.0, delta.y, 0.0);

            let x_delta =
                clip_movement(&mut map_query, &tile_query, transform.translation, x_delta);
            let y_delta =
                clip_movement(&mut map_query, &tile_query, transform.translation, y_delta);

            transform.translation += x_delta;
            transform.translation += y_delta;
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

// FIXME: movement clipping is based on aabb of hex tile -> very crappy
fn clip_movement(
    map_query: &mut MapQuery,
    tile_query: &Query<&Tile>,
    translation: Vec3,
    delta: Vec3,
) -> Vec3 {
    // FIXME: the hard coded 256 256 offset is crap!

    let p = translation + Vec3::new(256.0, 256.0, 0.0);
    let (x, y) = pixel_to_pointy_hex(p);
    // info!("tile: {:?}", tile);

    let tile_pos = TilePos(x as u32, y as u32);

    // use very small player box to make clipping bearable
    let player_size = Vec2::new(6.0, 6.0);

    // check collision with (non walkable) neighbor tiles
    for neighbor_pos in hex_neighbors(tile_pos) {
        if let Ok(neighbor_entity) = map_query.get_tile_entity(neighbor_pos, 0u16, 0u16) {
            if let Ok(neighbor_tile) = tile_query.get(neighbor_entity) {
                // info!("tile entity: {:?} {}", neighbor_entity, tile.texture_index);
                if neighbor_tile.texture_index != 0 {
                    continue;
                }
                let neighbor_box = pointy_hex_to_aabb(neighbor_pos.0 as i32, neighbor_pos.1 as i32);
                // info!("neighbor: {:?} {} {}", neighbor_box, p + delta, player_size);

                if collide(p + delta, player_size, neighbor_box.0, neighbor_box.1).is_some() {
                    // info!("collision");
                    return Vec3::ZERO;
                }
            }
        }
    }

    delta
}
