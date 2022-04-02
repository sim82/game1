use bevy::prelude::*;

pub mod ai;
pub mod brainy;
pub mod movement;
pub mod path;
pub mod pointer;

pub mod sprites {
    use bevy_aseprite::aseprite;
    aseprite!(pub Ferris, "assets/ferris2.0.aseprite");
    aseprite!(pub Pointer, "assets/pointer.aseprite");
    aseprite!(pub Pew, "assets/pew.aseprite");
}

#[derive(Component)]
pub struct TargetFlag;

// stuff related to projectiles (Pew, Pew)
// TODO: move to proper package
#[derive(Component)]
pub struct Pew(pub bool);
#[derive(Component)]
pub struct TimeToLive(pub f32);

pub fn pew_move_system(time: Res<Time>, mut query: Query<(&Pew, &mut Transform)>) {
    for (Pew(right), mut transform) in query.iter_mut() {
        let dir = if *right {
            Vec3::new(1.0, 0.0, 0.0)
        } else {
            Vec3::new(-1.0, 0.0, 0.0)
        } * time.delta_seconds()
            * 100.0;
        transform.translation += dir;
    }
}

pub fn time_to_live_reaper_system(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut TimeToLive)>,
) {
    for (entity, mut ttl) in query.iter_mut() {
        ttl.0 -= time.delta_seconds();
        if ttl.0 <= 0.0 {
            commands.entity(entity).despawn();
        }
    }
}
