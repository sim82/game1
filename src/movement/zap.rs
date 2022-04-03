use bevy::prelude::*;

use crate::{tune, Pew};

#[derive(Component)]
pub struct Zappable;

#[derive(Component)]
#[component(storage = "SparseSet")]
pub struct BeingZapped;

#[allow(clippy::type_complexity)]
pub fn check_pew_intersection_system(
    mut commands: Commands,
    query_non_zapped: Query<(Entity, &mut Transform), (With<Zappable>, Without<BeingZapped>)>,
    query_zapped: Query<(Entity, &mut Transform), (With<Zappable>, With<BeingZapped>)>,
    query_pew: Query<&Transform, (With<Pew>, Without<Zappable>)>,
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