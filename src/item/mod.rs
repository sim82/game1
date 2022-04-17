use std::collections::VecDeque;

use bevy::prelude::*;

pub mod medikit;

#[derive(Component)]
pub struct Item(f32);

#[derive(Component, Default)]
pub struct ItemContactProbe {
    pub contacts: VecDeque<Entity>,
}

fn item_contact_system(
    item_query: Query<(Entity, &Transform, &Item)>,
    mut item_probe_query: Query<(&Transform, &mut ItemContactProbe)>,
) {
    'outer: for (
        item_entity,
        Transform {
            translation: item_pos,
            ..
        },
        Item(size),
    ) in item_query.iter()
    {
        for (
            Transform {
                translation: pos, ..
            },
            mut contacts,
        ) in item_probe_query.iter_mut()
        {
            if (*pos - *item_pos).length() <= *size {
                contacts.contacts.push_back(item_entity);
                continue 'outer;
            }
        }
    }
}

pub struct ItemPlugin;

impl Plugin for ItemPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(medikit::pick_medikit_system)
            .add_system(medikit::spawn_medikits_system)
            .add_system(item_contact_system);
    }
}
