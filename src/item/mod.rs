use bevy::prelude::*;

pub mod medikit;

pub struct ItemPlugin;

impl Plugin for ItemPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(medikit::pick_medikit_system);
    }
}
