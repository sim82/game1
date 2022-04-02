use bevy::prelude::*;
use big_brain::prelude::*;

#[derive(Default)]
pub struct DiagnosticsTarget(Vec<Entity>);

// mixed experiments for extracting diagnostic info from big_brain Actors (strictly for debug visualization etc.)

fn show_scores_system(query: Query<(&Actor, &Name)>) {
    for (score, name) in query.iter() {
        info!("score: {:?} {:?}", score, name);
    }
}

fn select_actions_system(mut targets: ResMut<DiagnosticsTarget>, query: Query<(Entity, &Actor)>) {
    targets.0.clear();
    for (entity, Actor(actor_entity)) in query.iter() {
        // entity
        info!("actor {:?} for {:?}", entity, actor_entity);
        targets.0.push(entity);
    }
}

fn show_actions_system(world: &mut World) {
    if let Some(DiagnosticsTarget(entities)) = world.get_resource::<DiagnosticsTarget>() {
        for entity in entities.iter() {
            if let Some(entity_ref) = world.get_entity(*entity) {
                let components = entity_ref
                    .archetype()
                    .table_components()
                    .iter()
                    .map(|component_id| world.components().get_info(*component_id).unwrap().name())
                    .collect::<Vec<_>>();

                info!("diagnostic entity: {:?}: {:?}", entity, components);
            }
        }
    }

    // world.get_entity(entity).
    // for (entity, Actor(actor_entity)) in query.iter() {
    //     // entity
    //     info!("actor {:?} for {:?}", entity, actor_entity);
    // }
}

pub struct AiDiagnosticsPlugin;
impl Plugin for AiDiagnosticsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<DiagnosticsTarget>()
            .add_system(show_scores_system)
            // .add_system(select_actions_system)
            .add_system(show_actions_system.exclusive_system());
    }
}
