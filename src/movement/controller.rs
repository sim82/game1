use bevy::prelude::*;

#[derive(Debug, Clone, Component, Copy)]
pub struct Actor(pub Entity);

#[derive(Component)]
#[component(storage = "SparseSet")]
pub struct ActiveMovement;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Component, Reflect)]
pub enum MovementPriority {
    Background,
    Navigation,
    Evasion,
}

#[derive(Component, Default)]
pub struct MovementController {
    movements: Vec<(MovementPriority, Entity)>,
    active_movement: Option<Entity>,
}

impl MovementController {
    // pub fn spawn_movement<T>(
    //     &mut self,
    //     commands: &mut Commands,
    //     priority: MovementPriority,
    //     movement: T,
    //     owner: Entity,
    // ) where
    //     T: Component,
    // {
    //     commands.spawn().insert(movement).insert(Actor(owner));
    // }
}

pub fn movement_controller_update_system(
    commands: &mut Commands,
    added_query: Query<(Entity, &MovementPriority, &Actor), Added<MovementPriority>>,
    mut controller_query: Query<&mut MovementController>,
) {
    // attach new movements to controller
    for (entity, priority, actor) in added_query.iter() {
        if let Ok(mut controller) = controller_query.get_mut(actor.0) {
            controller.movements.push((*priority, entity));
        }
    }

    // decide on movement according to priority
    for mut controller in controller_query.iter_mut() {
        controller.movements.sort();

        let next_movement = controller.movements.last().map(|(_, entity)| *entity);
        if controller.active_movement == next_movement {
            // this can either mean that the same movement stays active or there is (and was) no active and next movement
            continue;
        }
        if let Some(entity) = controller.active_movement {
            commands.entity(entity).remove::<ActiveMovement>();
        }
        if let Some(entity) = next_movement {
            commands.entity(entity).insert(ActiveMovement);
            controller.active_movement = next_movement;
        }
    }
}
