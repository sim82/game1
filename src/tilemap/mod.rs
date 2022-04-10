use std::collections::HashSet;

use bevy::{prelude::*, render::render_resource::TextureUsages};
use bevy_ecs_tilemap::prelude::*;
use bevy_egui::{egui, EguiContext};
use bevy_prototype_debug_lines::DebugLines;
use rand::{thread_rng, Rng};

use crate::{debug::debug_draw_cross, path, pointer::ClickEvent};
pub mod io;

// mod helpers;

fn startup(mut commands: Commands, asset_server: Res<AssetServer>, mut map_query: MapQuery) {
    // commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    let texture_handle = asset_server.load("pointy_hex_tiles_18x20.png");

    // Create map entity and component:
    let map_entity = commands.spawn().id();
    let mut map = Map::new(0u16, map_entity);

    let mut map_settings = LayerSettings::new(
        MapSize(8, 8),
        ChunkSize(16, 16),
        TileSize(18.0, 20.0),
        TextureSize(128.0, 20.0),
    );
    map_settings.mesh_type = TilemapMeshType::Hexagon(HexType::Row);

    let (mut layer_builder, layer_entity) =
        LayerBuilder::<TileBundle>::new(&mut commands, map_settings.clone(), 0u16, 0u16);
    map.add_layer(&mut commands, 0u16, layer_entity);

    map_query.build_layer(&mut commands, layer_builder, texture_handle.clone());

    // Spawn Map
    // Required in order to use map_query to retrieve layers/tiles.
    commands
        .entity(map_entity)
        .insert(map)
        .insert(Transform::from_xyz(-256.0, -256.0, 0.0).with_scale(Vec3::splat(1.0)))
        .insert(GlobalTransform::default());
}

pub fn set_texture_filters_to_nearest(
    mut texture_events: EventReader<AssetEvent<Image>>,
    mut textures: ResMut<Assets<Image>>,
) {
    // quick and dirty, run this for all textures anytime a texture is created.
    for event in texture_events.iter() {
        if let AssetEvent::Created { handle } = event {
            if let Some(mut texture) = textures.get_mut(handle) {
                texture.texture_descriptor.usage = TextureUsages::TEXTURE_BINDING
                    | TextureUsages::COPY_SRC
                    | TextureUsages::COPY_DST;
            }
        }
    }
}

fn pointy_hex_to_pixel(x: i32, y: i32) -> Vec3 {
    let x = x as f32;
    let y = y as f32;
    let column_width = 18.0f32;
    let column_half_width = column_width / 2.0;

    let row_height = 20.0;
    let row_height_eff = row_height * 0.75;

    Vec3::new(
        (y + 1.0) * column_half_width + x * column_width,
        y * row_height_eff + row_height / 2.0,
        0.0,
    )
}

fn pixel_to_pointy_hex(p: Vec3) -> (i32, i32) {
    let column_width = 18.0f32;
    let column_half_width = column_width / 2.0;

    let row_height = 20.0 * 0.75;
    let major_y = (p.y / row_height).floor() as i32;

    let qx = p.x - (major_y as f32) * column_half_width;

    let major_x = (qx / column_width).floor() as i32;
    info!("major: {} {}", major_x, major_y);

    (major_x, major_y)
}

fn _pixel_to_pointy_hex2(p: Vec3) -> (i32, i32) {
    // adapted from https://gamedev.stackexchange.com/a/20753

    let y = p.y;
    let x = p.x;
    let b = 9.0;
    let a = 10.0;
    let c = 5.0;
    // Find out which major row and column we are on:
    let mx = (x / b).floor();
    let my = (y / (a + c)).floor();

    info!("major x {} y {}", mx, my);

    // Compute the offset into these row and column:
    let mut dx = x - mx * b;
    let dy = y - my * (a + c);
    info!("d x {} y {}", dx, dy);
    // Are we on the left of the hexagon edge, or on the right?
    let mut column = mx as i32;
    let mut row = my as i32;
    if ((column ^ row) & 1) == 0 {
        dx = b - dx;
    }
    let right = if dx * (a - c) < b * (dy - c) { 1 } else { 0 };

    // Now we have all the information we need, just fine-tune row and column.
    column += (row ^ column ^ right) & 1;
    row += right;
    // let row = (row - (row & 1)) / 2;
    // let column = (column - (column & 1)) / 2;
    (column, row)
}

fn background_on_click(
    mut commands: Commands,
    mut click_events: EventReader<ClickEvent>,
    mut debug_lines: ResMut<DebugLines>,
    interaction_state: Res<InteractionState>,
    mut map_query: MapQuery,
) {
    let map_id = 0u16;
    let layer_id = 0u16;
    for event in click_events.iter() {
        info!("clicked: {:?} {}", event, event.pos.x / 15.0);

        let p = event.pos;
        debug_draw_cross(&mut debug_lines, p, None);
        let qp = event.pos + Vec3::new(256.0, 256.0, 0.0);

        let (tx, ty) = pixel_to_pointy_hex(qp);
        info!("tile: {} {}", tx, ty);

        let tile_pos = TilePos(tx as u32, ty as u32);
        // Ignore errors for demo sake.

        match interaction_state.click_mode {
            ClickMode::Wall => {
                let _ = map_query.set_tile(
                    &mut commands,
                    tile_pos,
                    Tile {
                        texture_index: 0,
                        ..Default::default()
                    },
                    0u16,
                    0u16,
                );

                map_query.notify_chunk_for_tile(tile_pos, map_id, layer_id);
            }
            ClickMode::Fill => {
                let mut visited = HashSet::new();
                let mut s = vec![tile_pos];
                let mut steps_left = 1000;
                while !s.is_empty() && steps_left > 0 {
                    let cur_pos = s.pop().unwrap();

                    visited.insert(cur_pos);
                    let _ = map_query.set_tile(
                        &mut commands,
                        cur_pos,
                        Tile {
                            texture_index: 6,
                            ..Default::default()
                        },
                        0u16,
                        0u16,
                    );
                    map_query.notify_chunk_for_tile(cur_pos, map_id, layer_id);
                    steps_left -= 1;
                    // unsigned tile coords are quite annoying
                    if cur_pos.0 == 0 || cur_pos.1 == 0 {
                        continue;
                    }

                    for neighbor_pos in hex_neighbors(cur_pos) {
                        if !visited.contains(&neighbor_pos)
                            && map_query
                                .get_tile_entity(neighbor_pos, map_id, layer_id)
                                .is_err()
                        {
                            s.push(neighbor_pos);
                        }
                    }
                }
            }
            ClickMode::Probe => {
                // map_query.get_tile_entity(tile_pos, map_id, layer_id);

                let p = pointy_hex_to_pixel(tile_pos.0 as i32, tile_pos.1 as i32)
                    - Vec3::new(256.0, 256.0, 0.0);
                debug_draw_cross(&mut debug_lines, p, Some(2.0));
            }
        }
        // map_query.get_
        // for n in map_query.get_tile_neighbors(position, 0u16, 0u16) {
        //     info!("{:?}", n);
        // }
    }
}

fn hex_neighbors(pos: TilePos) -> [TilePos; 6] {
    [
        TilePos(pos.0 - 1, pos.1 + 1),
        TilePos(pos.0, pos.1 + 1),
        TilePos(pos.0 - 1, pos.1),
        TilePos(pos.0 + 1, pos.1),
        TilePos(pos.0, pos.1 - 1),
        TilePos(pos.0 + 1, pos.1 - 1),
    ]
}

fn new_tile_system(query: Query<(&TilePos), Added<TilePos>>) {
    for pos in query.iter() {
        info!("new tile: {:?}", pos);
    }
}

#[derive(PartialEq)]
enum ClickMode {
    Wall,
    Fill,
    Probe,
}

impl Default for ClickMode {
    fn default() -> Self {
        ClickMode::Wall
    }
}

#[derive(Default)]
struct InteractionState {
    click_mode: ClickMode,
}

fn tilemap_egui_ui_system(
    mut commands: Commands,
    mut egui_context: ResMut<EguiContext>,
    query: Query<(Entity, &TilePos, &Tile)>,
    mut map_query: MapQuery,
    mut interaction_state: ResMut<InteractionState>,
    chunk_query: Query<Entity, With<Chunk>>,
) {
    let mut do_save = false;
    let mut do_load = false;
    let mut do_clear = false;
    let mut do_spawn_waypoints = false;

    egui::Window::new("tilemap").show(egui_context.ctx_mut(), |ui| {
        do_clear = ui.button("clear").clicked();
        do_load = ui.button("load").clicked();
        do_save = ui.button("save").clicked();
        // ui.checkbox(&mut interaction_state.fill, "fill");
        ui.radio_value(&mut interaction_state.click_mode, ClickMode::Wall, "Wall");
        ui.radio_value(&mut interaction_state.click_mode, ClickMode::Fill, "Fill");
        ui.radio_value(&mut interaction_state.click_mode, ClickMode::Probe, "Probe");

        do_spawn_waypoints = ui.button("-> waypoints").clicked();
    });

    let mut do_notify_chunks = false;
    if do_clear {
        map_query.despawn_layer_tiles(&mut commands, 0u16, 0u16);
        do_notify_chunks = true;
    }
    if do_load {
        let tilemap = io::Tilemap::load("map.yaml");
        for io::Tile { x, y, t } in tilemap.tiles {
            let tile_pos = TilePos(x, y);
            map_query
                .set_tile(
                    &mut commands,
                    tile_pos,
                    Tile {
                        texture_index: t,
                        ..Default::default()
                    },
                    0u16,
                    0u16,
                )
                .unwrap();
            do_notify_chunks = true;
        }
    }
    if do_save {
        let tilemap = io::Tilemap {
            tiles: query
                .iter()
                .map(|(_entity, pos, tile)| io::Tile {
                    x: pos.0,
                    y: pos.1,
                    t: tile.texture_index,
                })
                .collect(),
        };
        tilemap.save("map.yaml");
    }
    if do_notify_chunks {
        // re-meshing all chunks seems like the easiest approach since we cannot find out otherwise
        // which chunks are affected by a set_tile / depspawn_layer_tiles operation.
        for chunk_entity in chunk_query.iter() {
            map_query.notify_chunk(chunk_entity);
        }
    }
    if do_spawn_waypoints {
        for (_entity, tile_pos, tile) in query.iter() {
            if tile.texture_index == 0 {
                continue;
            }
            commands
                .spawn()
                .insert(path::Waypoint)
                .insert(Transform::from_translation(
                    pointy_hex_to_pixel(tile_pos.0 as i32, tile_pos.1 as i32)
                        - Vec3::new(256.0, 256.0, 0.0),
                ));
        }
    }
}

pub struct PlayfieldPlugin;

impl Plugin for PlayfieldPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<InteractionState>()
            .add_startup_system(startup)
            .add_system(background_on_click)
            .add_system(set_texture_filters_to_nearest)
            .add_system(new_tile_system)
            .add_system(tilemap_egui_ui_system);
    }
}
