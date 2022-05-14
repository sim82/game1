use crate::{ai::HealthPoints, path::Waypoint, sprites, Despawn};
use bevy::prelude::*;
use bevy_aseprite::AsepriteBundle;
use rand::prelude::SliceRandom;

use super::{Item, ItemContactProbe};

#[derive(Component)]
pub struct Medikit;

pub mod tune {
    pub const MEDIKIT_PICK_DIST: f32 = 8.0;
    pub const MEDIKIT_HEALTH: i32 = 10;
    pub const MEDIKIT_COUNT: usize = 10;
}

pub fn pick_medikit_system(
    mut commands: Commands,
    medikit_query: Query<Entity, With<Medikit>>,
    mut receiver_query: Query<
        (&mut HealthPoints, &mut ItemContactProbe),
        Changed<ItemContactProbe>,
    >,
) {
    for (mut health_points, mut contacts) in receiver_query.iter_mut() {
        // info!("item contact");
        for item_entity in contacts.contacts.drain(..) {
            if medikit_query.get(item_entity).is_ok() {
                health_points.health += tune::MEDIKIT_HEALTH;
                commands.entity(item_entity).insert(Despawn::ThisFrame);
            }
        }
    }
}

// pub fn pick_medikit_system(
//     mut commands: Commands,
//     query: Query<(Entity, &Transform), With<Medikit>>,
//     mut receiver_query: Query<(&Transform, &mut HealthPoints)>,
// ) {
//     for (
//         medikit_entity,
//         Transform {
//             translation: medikit_pos,
//             ..
//         },
//     ) in query.iter()
//     {
//         for (
//             Transform {
//                 translation: receiver_pos,
//                 ..
//             },
//             mut health_points,
//         ) in receiver_query.iter_mut()
//         {
//             let dist = (*medikit_pos - *receiver_pos).length();
//             // info!("medikit {}", dist);

//             if dist <= tune::MEDIKIT_PICK_DIST {
//                 health_points.health += tune::MEDIKIT_HEALTH;
//                 // FIXME: race condition between two recievers reaching it at the same time.
//                 // we probably need some centralized item pickup resolving.
//                 commands.entity(medikit_entity).despawn_recursive();
//             }
//         }
//     }

//     //blas
// }

pub fn spawn_medikits_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    query: Query<Entity, With<Medikit>>,
    waypoints_query: Query<&Transform, With<Waypoint>>,
) {
    let count = query.iter().count();
    if count >= tune::MEDIKIT_COUNT {
        return;
    }

    let num_create = tune::MEDIKIT_COUNT - count;
    let waypoint_pos = waypoints_query
        .iter()
        .map(|transform| transform.translation)
        .collect::<Vec<_>>();

    if waypoint_pos.len() < num_create {
        return;
    }

    let mut rng = rand::thread_rng();

    for pos in waypoint_pos.choose_multiple(&mut rng, num_create) {
        commands
            .spawn_bundle(AsepriteBundle {
                aseprite: asset_server.load(sprites::Medikit::PATH),
                transform: Transform {
                    scale: Vec3::splat(1.),
                    translation: *pos + Vec3::new(0.0, 0.0, 5.0),
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(Medikit)
            .insert(Item(tune::MEDIKIT_PICK_DIST));
    }
}
