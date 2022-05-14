use bevy::prelude::*;
use bevy_aseprite::{anim::AsepriteAnimation, Aseprite, AsepriteBundle};

use crate::{ai::HealthPoints, sprites, Despawn};

pub fn die_system(
    mut commands: Commands,
    query: Query<(Entity, &HealthPoints, &Transform, &Handle<Aseprite>)>,
    asset_server: Res<AssetServer>,
) {
    for (entity, health_points, transform, aseprite) in query.iter() {
        if health_points.health <= 0 {
            commands.entity(entity).insert(Despawn::ThisFrame);
            commands
                .spawn()
                .insert_bundle(AsepriteBundle {
                    aseprite: aseprite.clone(),
                    animation: AsepriteAnimation::from(sprites::Ferris::tags::DIE), // FIXME: this should be handled generic
                    transform: *transform,
                    ..Default::default()
                })
                .insert(Despawn::TimeToLive(3.0));
        }
    }
}
