#![feature(exclusive_range_pattern)]
use bevy::prelude::*;
use bevy_ecs_tilemap::{MapQuery, Tile};
use bevy_prototype_debug_lines::DebugLines;
use hex::tilemap::{HexTileAppearance, HexTileCoord};
use movement::crab_move::clip_movement;

pub mod ai;
pub mod brainy;
pub mod debug;
pub mod die;
pub mod hex;
pub mod item;
pub mod movement;
pub mod path;
pub mod pointer;
pub mod tilemap;
pub mod ui;

pub mod tune {
    pub const WALK_SPEED: f32 = 15.0;
    pub const PEW_SPEED: f32 = 50.0;
    pub const PEW_ZAP_DISTANCE: f32 = 8.0;
    pub const PEW_DETECT_FAR: f32 = 150.0;
    pub const PEW_DETECT_NEAR: f32 = 50.0;

    pub const AMMO_RELOAD_TIME: f32 = 0.5;
    pub const AMMO_RELOAD_AMOUNT: f32 = 3.0;
}
pub mod sprites {
    use bevy_aseprite::aseprite;
    aseprite!(pub Ferris, "assets/ferris2.0.aseprite");
    aseprite!(pub Pointer, "assets/pointer.aseprite");
    aseprite!(pub Pew, "assets/pew.aseprite");
    aseprite!(pub Medikit, "assets/medikit.aseprite");
}

#[derive(Component)]
pub struct InputTarget;

#[derive(Component)]
pub struct TargetFlag;

// stuff related to projectiles (Pew, Pew)
// TODO: move to proper package
#[derive(Component)]
pub struct Pew(pub bool, pub f32);
// #[derive(Component)]
// pub struct TimeToLive(pub f32);

#[derive(Component)]
#[component(storage = "SparseSet")]
pub enum Despawn {
    ThisFrame,
    TimeToLive(f32),
}

pub fn pew_move_system(
    mut commands: Commands,
    time: Res<Time>,
    mut debug_lines: ResMut<DebugLines>,
    mut query: Query<(Entity, &Pew, &mut Transform)>,
    tile_query: Query<(&HexTileCoord, &HexTileAppearance)>,
) {
    for (entity, Pew(right, _), mut transform) in query.iter_mut() {
        let dir = if *right {
            Vec3::new(1.0, 0.0, 0.0)
        } else {
            Vec3::new(-1.0, 0.0, 0.0)
        } * time.delta_seconds()
            * tune::PEW_SPEED;

        // // FIXME: it is not the smartest idea to use the clip_code to detect pew-wall collision, but it gets the job done quickly
        let d = clip_movement(
            &mut debug_lines,
            &tile_query,
            transform.translation,
            dir,
            0..1,
        );
        if d == Vec3::ZERO {
            commands.entity(entity).insert(Despawn::ThisFrame);
        } else {
            transform.translation += dir;
        }
    }
}

// pub fn time_to_live_reaper_system(
//     mut commands: Commands,
//     time: Res<Time>,
//     mut query: Query<(Entity, &mut TimeToLive)>,
// ) {
//     for (entity, mut ttl) in query.iter_mut() {
//         ttl.0 -= time.delta_seconds();
//         if ttl.0 <= 0.0 {
//             commands.entity(entity).despawn_recursive();
//         }
//     }
// }

pub fn despawn_reaper_system(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Despawn)>,
) {
    for (entity, mut despawn) in query.iter_mut() {
        let despawn = match *despawn {
            Despawn::ThisFrame => true,
            Despawn::TimeToLive(ref mut ttl) => {
                *ttl -= time.delta_seconds();
                *ttl <= 0.0
            }
        };
        if despawn {
            info!("despawn {:?}", entity);
            commands.entity(entity).despawn_recursive();
        }
    }
}
