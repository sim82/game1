use bevy::prelude::*;
use big_brain::prelude::*;

#[derive(Default)]
pub struct DiagnosticsTarget(Vec<(Entity, Entity)>);

// mixed experiments for extracting diagnostic info from big_brain Actors (strictly for debug visualization etc.)

fn _show_scores_system(query: Query<(&Actor, &Name)>) {
    for (score, name) in query.iter() {
        info!("score: {:?} {:?}", score, name);
    }
}

fn _select_actions_system(mut targets: ResMut<DiagnosticsTarget>, query: Query<(Entity, &Actor)>) {
    targets.0.clear();
    for (entity, Actor(actor_entity)) in query.iter() {
        // entity
        info!("actor {:?} for {:?}", entity, actor_entity);
        targets.0.push((entity, *actor_entity));
    }
}

fn _show_actions_system(world: &mut World) {
    if let Some(DiagnosticsTarget(entities)) = world.get_resource::<DiagnosticsTarget>() {
        for (entity, _actor_entity) in entities.iter() {
            if let Some(entity_ref) = world.get_entity(*entity) {
                for component_id in entity_ref.archetype().table_components().iter() {
                    if let Some(info) = world.components().get_info(*component_id) {
                        if info.name().starts_with("game1::ai::actions::") {
                            if let Some((_, action_name)) = info.name().rsplit_once("::") {
                                println!("action: {}", action_name);
                                // TODO: how to continue from here? we want to set set action name as overlay text of the actor_entity...

                                // if let Some(actor_entity_ref) = world.get_entity(actor_entity) {
                                //     // actor_entity_ref.
                                //     // world.
                                // }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub struct AiDiagnosticsPlugin;
impl Plugin for AiDiagnosticsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<DiagnosticsTarget>()
        // .add_system(show_scores_system)
        // .add_system(select_actions_system)
        // .add_system(show_actions_system.exclusive_system())
        ;
    }
}
