use bevy::prelude::*;
use bevy_aseprite::{anim::AsepriteAnimation, AsepriteBundle};
use big_brain::prelude::*;

use crate::{
    ai::util::Ammo,
    movement::crab_move::{CrabMoveDirection, CrabMoveWalker},
    sprites, tune, Despawn, Pew, TargetFlag,
};

use super::DebugAction;

#[derive(Component, Default, Debug, Clone)]
pub struct Shoot {
    shoot_right: bool,
    reload: f32,
}

pub fn shoot_action_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    time: Res<Time>,
    mut query: Query<(&Actor, &mut ActionState, &mut Shoot)>,
    player_query: Query<&Transform, With<TargetFlag>>,
    mut my_query: Query<(&Transform, &mut CrabMoveWalker, &mut Ammo)>,
) {
    for (Actor(actor_entity), mut state, mut shoot) in query.iter_mut() {
        commands
            .entity(*actor_entity)
            .insert(DebugAction::new("shoot", state.clone()));

        match *state {
            ActionState::Init => {
                let (
                    Transform {
                        translation: my_pos,
                        ..
                    },
                    mut walker,
                    _,
                ) = my_query.get_mut(*actor_entity).unwrap(); // FIXME

                if let Ok(Transform {
                    translation: target_pos,
                    ..
                }) = player_query.get_single()
                {
                    shoot.shoot_right = target_pos.x > my_pos.x;
                }

                walker.direction = CrabMoveDirection::None;
                *state = ActionState::Executing;
            }
            ActionState::Executing => {
                shoot.reload -= time.delta_seconds();
                if shoot.reload <= 0.0 {
                    let (Transform { translation, .. }, _, mut ammo) =
                        my_query.get_mut(*actor_entity).unwrap();

                    let offset = if shoot.shoot_right {
                        Vec3::new(tune::PEW_ZAP_DISTANCE, 0.0, 0.0)
                    } else {
                        Vec3::new(-tune::PEW_ZAP_DISTANCE, 0.0, 0.0)
                    };
                    commands
                        .spawn_bundle(AsepriteBundle {
                            aseprite: asset_server.load(sprites::Pew::PATH),
                            animation: AsepriteAnimation::from(sprites::Pew::tags::GLITTER),
                            transform: Transform {
                                scale: Vec3::splat(1.),
                                translation: *translation + offset,
                                ..Default::default()
                            },

                            ..Default::default()
                        })
                        .insert(Pew(shoot.shoot_right, 15.0))
                        .insert(Despawn::TimeToLive(10.0));
                    shoot.reload = 0.2;
                    ammo.ammo -= 1.0;
                }
            }
            ActionState::Cancelled => {
                *state = ActionState::Failure;
            }
            _ => {}
        }
    }
}
