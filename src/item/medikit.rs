use crate::ai::HealthPoints;
use bevy::prelude::*;

#[derive(Component)]
pub struct Medikit;

pub mod tune {
    pub const MEDIKIT_PICK_DIST: f32 = 8.0;
    pub const MEDIKIT_HEALTH: i32 = 25;
}

pub fn pick_medikit_system(
    mut commands: Commands,
    query: Query<(Entity, &Transform), With<Medikit>>,
    mut receiver_query: Query<(&Transform, &mut HealthPoints)>,
) {
    for (
        medikit_entity,
        Transform {
            translation: medikit_pos,
            ..
        },
    ) in query.iter()
    {
        for (
            Transform {
                translation: receiver_pos,
                ..
            },
            mut health_points,
        ) in receiver_query.iter_mut()
        {
            let dist = (*medikit_pos - *receiver_pos).length();
            info!("medikit {}", dist);

            if dist <= tune::MEDIKIT_PICK_DIST {
                health_points.health += tune::MEDIKIT_HEALTH;
                // FIXME: race condition between two recievers reaching it at the same time.
                // we probably need some centralized item pickup resolving.
                commands.entity(medikit_entity).despawn_recursive();
            }
        }
    }

    //blas
}
