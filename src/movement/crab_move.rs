use std::ops::Range;

use crate::{
    debug::{debug_draw_box, debug_draw_cross},
    hex::tilemap::{HexTileAppearance, HexTileCoord},
    pointer::MouseGrabState,
    sprites,
    tilemap::{hex_neighbors, pixel_to_pointy_hex, pointy_hex_to_aabb},
    tune,
};
use bevy::{prelude::*, sprite::collide_aabb::collide};
use bevy_aseprite::AsepriteAnimation;
use bevy_ecs_tilemap::{MapQuery, Tile, TilePos};
use bevy_prototype_debug_lines::DebugLines;

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
    tile_query2: Query<(&HexTileCoord, &HexTileAppearance)>,
    grab_state: ResMut<MouseGrabState>,
    mut debug_lines: ResMut<DebugLines>,
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

            let x_delta = clip_movement(
                &mut debug_lines,
                &tile_query2,
                transform.translation,
                x_delta,
                0..1,
            );
            let y_delta = clip_movement(
                &mut debug_lines,
                &tile_query2,
                transform.translation,
                y_delta,
                0..1,
            );

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
pub fn clip_movement(
    debug_lines: &mut DebugLines,
    tile_query: &Query<(&HexTileCoord, &HexTileAppearance)>,
    translation: Vec3,
    delta: Vec3,
    solid_range: Range<usize>,
) -> Vec3 {
    // FIXME: the hard coded 256 256 offset is crap!
    let box_size = Vec2::new(18.0, 20.0);

    let aabbs: Vec<_> = tile_query
        .iter()
        .filter_map(|(coord, app)| {
            if solid_range.contains(&app.tile_type) {
                Some((
                    (coord.cube.to_odd_r_screen() * box_size).extend(0.0),
                    box_size * Vec2::new(1.0, 0.75),
                ))
            } else {
                None
            }
        })
        .collect();

    // info!("aabbs: {:?}", aabbs);

    // use very small player box to make clipping bearable
    let player_size = Vec2::new(6.0, 6.0);
    debug_draw_box(debug_lines, translation + delta, player_size, Some(0.2));

    // check collision with (non walkable) neighbor tiles
    for aabb in aabbs.iter() {
        if collide(translation + delta, player_size, aabb.0, aabb.1).is_some() {
            // info!("collision");
            debug_draw_box(debug_lines, aabb.0, aabb.1, Some(0.2));

            return Vec3::ZERO;
        }
    }

    delta
}
