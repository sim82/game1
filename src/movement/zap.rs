use bevy::prelude::*;

use crate::{ai::HealthPoints, tune, Despawn, Pew};

#[derive(Component)]
pub struct Zappable;

#[derive(Component)]
#[component(storage = "SparseSet")]
pub struct BeingZapped;

// FIXME: handling of zap damage is complete crap:
//  - decoupling between 'begin zapped' detection and applying damage is weird
//  - pews only lose power the first time they hit a 'non-zapped' entity
//  - but amount of damage is relative to the duration of the 'being zapped' state

#[allow(clippy::type_complexity)]
pub fn check_pew_intersection_system(
    _time: Res<Time>,
    mut commands: Commands,
    query_non_zapped: Query<(Entity, &mut Transform), (With<Zappable>, Without<BeingZapped>)>,
    query_zapped: Query<(Entity, &mut Transform), (With<Zappable>, With<BeingZapped>)>,
    mut query_pew: Query<(&Transform, &mut Pew, Entity), Without<Zappable>>,
) {
    let mut pew_pos = query_pew
        .iter_mut()
        .map(|(transform, pew, entity)| (transform.translation, pew, entity))
        .collect::<Vec<_>>();

    const ZAP_DIST: f32 = tune::PEW_ZAP_DISTANCE;

    // determine which non-zapped entities get hit this frame
    for (entity, Transform { translation, .. }) in query_non_zapped.iter() {
        if let Some((_, ref mut pew, pew_entity)) = pew_pos
            .iter_mut()
            .find(|(pos, _, _)| (*pos - *translation).length() < ZAP_DIST)
        {
            commands.entity(entity).insert(BeingZapped);
            // pew.1 -= time.delta_seconds() * 120.0;
            pew.1 -= 5.0; // time invariant!
            if pew.1 <= 0.0 {
                commands.entity(*pew_entity).insert(Despawn::ThisFrame);
            }
        }
    }

    // determine which being-zapped entities become non-zapped
    for (entity, Transform { translation, .. }) in query_zapped.iter() {
        if !pew_pos
            .iter()
            .any(|(pew, _, _)| (*pew - *translation).length() < ZAP_DIST)
        {
            commands.entity(entity).remove::<BeingZapped>();
        }
    }
}

pub fn apply_zap_damage(time: Res<Time>, mut query: Query<&mut HealthPoints, With<BeingZapped>>) {
    for mut health_points in query.iter_mut() {
        // info!("zap: {}", time.delta_seconds());
        health_points.health -= (time.delta_seconds() * 120.0) as i32;
    }
}
