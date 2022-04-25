use std::collections::HashMap;

use bevy::{math::Vec3Swizzles, prelude::*};
use bevy_prototype_debug_lines::DebugLines;

use crate::{hex::Cube, path, pointer::ClickEvent};

use super::{
    editor::{background_on_click, tilemap_egui_ui_system, InteractionState},
    io, wavefunction, Hex,
};

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct HexTileAppearance {
    pub tile_type: usize,
}

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct HexTileCoord {
    pub cube: Cube,
}

pub struct Resources {
    pub base_entity: Entity,
    pub texture_atlas: Handle<TextureAtlas>,
    pub tile_size: Vec2,
}

impl Default for Resources {
    fn default() -> Self {
        Self {
            base_entity: Entity::from_raw(0), // FIXME: this is set in the init_system, but I'm too lazy for Option<>
            texture_atlas: Default::default(),
            tile_size: Default::default(),
        }
    }
}

fn init_system(
    mut commands: Commands,
    mut resources: ResMut<Resources>,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    //
    let texture_handle = asset_server.load("pointy_hex_tiles_18x20.png");
    resources.tile_size = Vec2::new(18.0, 20.0);
    let texture_atlas = TextureAtlas::from_grid(texture_handle, resources.tile_size, 7, 1);
    resources.texture_atlas = texture_atlases.add(texture_atlas);
    // commands.spawn_bundle(SpriteSheetBundle {
    //     texture_atlas: texture_atlas_handle,
    //     transform: Transform::from_translation(Vec3::new(128.0, 128.0, 0.0)),
    //     ..Default::default()
    // });

    resources.base_entity = commands
        .spawn()
        .insert(GlobalTransform::default())
        .insert(Transform::default())
        .id();

    if let Ok(init) = io::Tilemap::load("map.yaml") {
        let tiles: HashMap<Cube, usize> = init
            .tiles
            .iter()
            .map(|x| {
                let axial = Hex { q: x.x, r: x.y };
                (axial.into(), x.t)
            })
            .collect();

        for (cube, tile_type) in wavefunction::test(&tiles) {
            commands
                .entity(resources.base_entity)
                .with_children(|commands| {
                    commands
                        .spawn()
                        .insert(HexTileCoord { cube })
                        .insert(HexTileAppearance { tile_type });
                });
        }
    }
}

fn spawn_sprites_system(
    mut commands: Commands,
    resources: Res<Resources>,
    query: Query<(Entity, &HexTileCoord, &HexTileAppearance), Added<HexTileAppearance>>,
    mut query_changed: Query<
        (Entity, &HexTileCoord, &HexTileAppearance, &mut Transform),
        Changed<HexTileCoord>,
    >,
) {
    for (entity, coord, apperance) in query.iter() {
        let coord_screen = coord.cube.to_odd_r_screen() * resources.tile_size;
        info!("coord_screen: {:?}", coord_screen);
        let index = apperance.tile_type;
        // commands.entity(entity).with_children(|commands| {
        commands.entity(entity).insert_bundle(SpriteSheetBundle {
            texture_atlas: resources.texture_atlas.clone(),
            transform: Transform::from_translation(coord_screen.extend(0.0)),
            sprite: TextureAtlasSprite {
                index,
                ..Default::default()
            },
            ..Default::default()
        });
    }

    for (entity, coord, _apperance, mut transform) in query_changed.iter_mut() {
        if query.get(entity).is_ok() {
            continue;
        }

        let coord_screen = coord.cube.to_odd_r_screen() * resources.tile_size;
        transform.translation = coord_screen.extend(0.0);
        // info!("coord_screen: {:?}", coord_screen);
    }
}

pub fn pixel_to_pointy_hex(p: Vec3) -> Vec2 {
    let column_width = 18.0f32;
    let column_half_width = column_width / 2.0;

    let row_height = 20.0 * 0.75;
    let major_y = (p.y / row_height).floor();

    //   let qx = p.x - (major_y as f32) * column_half_width;
    let qx = p.x - (major_y as i32 & 1) as f32 * 0.5;
    let major_x = (qx / column_width).floor();
    // info!("major: {} {}", major_x, major_y);

    Vec2::new(major_x, major_y)
}

fn spawn_waypoints_system(
    query: Query<(Entity, &HexTileCoord, &HexTileAppearance), Added<HexTileAppearance>>,
    resources: Res<Resources>,
    mut commands: Commands,
) {
    for (_entity, tile_pos, tile) in query.iter() {
        if tile.tile_type == 0 {
            continue;
        }
        commands
            .spawn()
            .insert(path::Waypoint)
            .insert(Transform::from_translation(
                (tile_pos.cube.to_odd_r_screen() * resources.tile_size).extend(0.0),
            ));
    }
}

pub struct HexTilemapPlugin;

impl Plugin for HexTilemapPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Resources>()
            .register_type::<HexTileAppearance>()
            .register_type::<HexTileCoord>()
            .init_resource::<InteractionState>()
            .add_startup_system(init_system)
            .add_system(spawn_sprites_system)
            .add_system(spawn_waypoints_system)
            .add_system(background_on_click)
            .add_system(tilemap_egui_ui_system);
    }
}
