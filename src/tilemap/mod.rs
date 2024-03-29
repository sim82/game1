use bevy::{prelude::*, render::render_resource::TextureUsages};
use bevy_ecs_tilemap::prelude::*;
use bevy_egui::{egui, EguiContext};
use bevy_prototype_debug_lines::DebugLines;

use crate::{
    ai::inspect::AiInspectTarget,
    debug::debug_draw_cross,
    movement::control::MovementGoToPoint,
    path::{self},
    pointer::ClickEvent,
};
pub mod io;

// mod helpers;

fn startup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    info!("startup tilemap");

    let texture = TilemapTexture::Single(asset_server.load("pointy_hex_tiles_18x20.png"));

    let size = TilemapSize { x: 128, y: 128 };

    let storage = TileStorage::empty(size);
    // Create map entity and component:
    let map_entity = commands.spawn().id();
    let tile_size = TilemapTileSize { x: 18.0, y: 20.0 };
    let grid_size = TilemapGridSize { x: 18.0, y: 20.0 };

    let map_type = TilemapType::Hexagon(HexCoordSystem::Row);
    // let transform = Transform::from_xyz(-256.0, -256.0, 0.0).with_scale(Vec3::splat(1.0));

    let transform = get_tilemap_center_transform(&size, &grid_size, 0.0);
    commands.entity(map_entity).insert_bundle(TilemapBundle {
        grid_size,
        map_type,
        size,
        storage,
        texture,
        tile_size,
        transform,
        ..default()
    });
    // .insert(map)
    // .insert(Transform::from_xyz(-256.0, -256.0, 0.0).with_scale(Vec3::splat(1.0)))
    // .insert(GlobalTransform::default());
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

pub fn pointy_hex_to_pixel(x: i32, y: i32) -> Vec3 {
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

pub fn pointy_hex_to_aabb(x: i32, y: i32) -> (Vec3, Vec2) {
    let column_width = 18.0f32;
    let row_height = 20.0;
    let row_height_eff = row_height * 0.75;
    (
        pointy_hex_to_pixel(x, y),
        Vec2::new(column_width, row_height_eff),
    )
}

pub fn pixel_to_pointy_hex(p: Vec3) -> (i32, i32) {
    let column_width = 18.0f32;
    let column_half_width = column_width / 2.0;

    let row_height = 20.0 * 0.75;
    let major_y = (p.y / row_height).floor() as i32;

    let qx = p.x - (major_y as f32) * column_half_width;

    let major_x = (qx / column_width).floor() as i32;
    // info!("major: {} {}", major_x, major_y);

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
    let right = i32::from(dx * (a - c) < b * (dy - c));

    // Now we have all the information we need, just fine-tune row and column.
    column += (row ^ column ^ right) & 1;
    row += right;
    // let row = (row - (row & 1)) / 2;
    // let column = (column - (column & 1)) / 2;
    (column, row)
}

fn _background_on_click(
    mut commands: Commands,
    mut click_events: EventReader<ClickEvent>,
    mut debug_lines: ResMut<DebugLines>,
    interaction_state: Res<InteractionState>,
    // mut map_query: MapQuery,
    mut tilemap_query: Query<(Entity, &mut TileStorage)>,
    mut _tile_query: Query<&mut TileTexture>,

    ai_inspect_query: Query<(Entity, &Transform), With<AiInspectTarget>>,
) {
    let (tilemap_entity, mut tile_storage) = tilemap_query.get_single_mut().unwrap();

    for event in click_events.iter() {
        info!("clicked: {:?} {}", event, event.pos.x / 15.0);

        let p = event.pos;
        debug_draw_cross(&mut debug_lines, p, None);
        let qp = event.pos + Vec3::new(256.0, 256.0, 0.0);

        let (tx, ty) = pixel_to_pointy_hex(qp);
        info!("tile: {} {}", tx, ty);

        let tile_pos = TilePos {
            x: tx as u32,
            y: ty as u32,
        };
        // Ignore errors for demo sake.

        match interaction_state.click_mode {
            ClickMode::Wall => {
                let tile_ent = if let Some(tile_ent) = tile_storage.get(&tile_pos) {
                    tile_ent
                } else {
                    let tile_ent = commands
                        .spawn()
                        .insert_bundle(TileBundle {
                            position: tile_pos,
                            tilemap_id: TilemapId(tilemap_entity),
                            ..Default::default()
                        })
                        .id();
                    tile_storage.set(&tile_pos, tile_ent);
                    tile_ent
                };

                commands.entity(tile_ent).insert(TileTexture(0));

                // let _ = tile_storage.set(
                //     &tile_pos,
                //     Tile {
                //         texture_index: 0,
                //         ..Default::default()
                //     },
                //     0u16,
                //     0u16,
                // );

                // map_query.notify_chunk_for_tile(tile_pos, map_id, layer_id);
            }
            ClickMode::Fill => {
                todo!()
                // let mut visited = HashSet::new();
                // let mut s = vec![tile_pos];
                // let mut steps_left = 1000;
                // while !s.is_empty() && steps_left > 0 {
                //     let cur_pos = s.pop().unwrap();

                //     visited.insert(cur_pos);
                //     let _ = map_query.set_tile(
                //         &mut commands,
                //         cur_pos,
                //         Tile {
                //             texture_index: 6,
                //             ..Default::default()
                //         },
                //         0u16,
                //         0u16,
                //     );
                //     map_query.notify_chunk_for_tile(cur_pos, map_id, layer_id);
                //     steps_left -= 1;
                //     // unsigned tile coords are quite annoying
                //     if cur_pos.0 == 0 || cur_pos.1 == 0 {
                //         continue;
                //     }

                //     for neighbor_pos in hex_neighbors(cur_pos) {
                //         if !visited.contains(&neighbor_pos)
                //             && map_query
                //                 .get_tile_entity(neighbor_pos, map_id, layer_id)
                //                 .is_err()
                //         {
                //             s.push(neighbor_pos);
                //         }
                //     }
                // }
            }
            ClickMode::Probe => {
                // map_query.get_tile_entity(tile_pos, map_id, layer_id);

                let p = pointy_hex_to_pixel(tile_pos.x as i32, tile_pos.y as i32)
                    - Vec3::new(256.0, 256.0, 0.0);
                debug_draw_cross(&mut debug_lines, p, Some(2.0));
            }
            ClickMode::GoThere => {
                if let Ok((target, Transform { translation: _, .. })) =
                    ai_inspect_query.get_single()
                {
                    commands.entity(target).insert(MovementGoToPoint(event.pos));
                }
            }
        }
        // map_query.get_
        // for n in map_query.get_tile_neighbors(position, 0u16, 0u16) {
        //     info!("{:?}", n);
        // }
    }
}

pub fn hex_neighbors(pos: TilePos) -> [TilePos; 6] {
    [
        TilePos {
            x: pos.x - 1,
            y: pos.y + 1,
        },
        TilePos {
            x: pos.x,
            y: pos.y + 1,
        },
        TilePos {
            x: pos.x - 1,
            y: pos.y,
        },
        TilePos {
            x: pos.x + 1,
            y: pos.y,
        },
        TilePos {
            x: pos.x,
            y: pos.y - 1,
        },
        TilePos {
            x: pos.x + 1,
            y: pos.y - 1,
        },
    ]
}

fn _new_tile_system(query: Query<&TilePos, Added<TilePos>>) {
    for pos in query.iter() {
        info!("new tile: {:?}", pos);
    }
}

#[derive(PartialEq)]
enum ClickMode {
    Wall,
    Fill,
    Probe,
    GoThere,
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

#[allow(unused)]
fn tilemap_egui_ui_system(
    mut commands: Commands,
    mut egui_context: ResMut<EguiContext>,
    query: Query<(Entity, &TilePos)>,
    mut tilemap_query: Query<(Entity, &mut TileStorage)>,

    // mut map_query: MapQuery,
    mut interaction_state: ResMut<InteractionState>,
    // chunk_query: Query<Entity, With<Chunk>>,
) {
    let mut do_save = false;
    let mut do_load = false;
    let mut do_clear = false;
    // let mut do_spawn_waypoints = false;

    let (tilemap_ent, mut tile_storage) = tilemap_query.get_single_mut().unwrap();

    egui::Window::new("tilemap").show(egui_context.ctx_mut(), |ui| {
        do_clear = ui.button("clear").clicked();
        do_load = ui.button("load").clicked();
        do_save = ui.button("save").clicked();
        // ui.checkbox(&mut interaction_state.fill, "fill");
        ui.radio_value(&mut interaction_state.click_mode, ClickMode::Wall, "Wall");
        ui.radio_value(&mut interaction_state.click_mode, ClickMode::Fill, "Fill");
        ui.radio_value(&mut interaction_state.click_mode, ClickMode::Probe, "Probe");
        ui.radio_value(
            &mut interaction_state.click_mode,
            ClickMode::GoThere,
            "Go there",
        );

        // do_spawn_waypoints = ui.button("-> waypoints").clicked();
    });

    let mut do_notify_chunks = false;
    if do_clear {
        todo!();
        // map_query.despawn_layer_tiles(&mut commands, 0u16, 0u16);
        // do_notify_chunks = true;
    }
    if do_load {
        let tilemap = io::Tilemap::load("map.yaml").unwrap();
        do_notify_chunks = !tilemap.tiles.is_empty();
        spawn_tilemap(tilemap, tilemap_ent, &mut tile_storage, &mut commands);
    }
    if do_save {
        todo!();
        // let tilemap = io::Tilemap {
        //     tiles: query
        //         .iter()
        //         .map(|(_entity, pos, tile)| io::Tile {
        //             x: pos.0,
        //             y: pos.1,
        //             t: tile.texture_index,
        //         })
        //         .collect(),
        // };
        // tilemap.save("map.yaml").unwrap();
    }
    // if do_notify_chunks {
    //     // re-meshing all chunks seems like the easiest approach since we cannot find out otherwise
    //     // which chunks are affected by a set_tile / depspawn_layer_tiles operation.
    //     notify_all_chunks(&chunk_query, &mut map_query);
    // }
    // if do_spawn_waypoints {
    //     spawn_waypoints(&query, &mut commands);
    // }
}

// fn notify_all_chunks(chunk_query: &Query<Entity, With<Chunk>>, map_query: &mut MapQuery) {
//     for chunk_entity in chunk_query.iter() {
//         map_query.notify_chunk(chunk_entity);
//     }
// }

fn spawn_waypoints_system(
    query: Query<(Entity, &TilePos, &TileTexture), Added<TilePos>>,
    mut commands: Commands,
) {
    for (_entity, tile_pos, tile_texture) in query.iter() {
        if tile_texture.0 == 0 {
            continue;
        }
        commands
            .spawn()
            .insert(path::Waypoint)
            .insert(Transform::from_translation(
                pointy_hex_to_pixel(tile_pos.x as i32, tile_pos.y as i32)
                    - Vec3::new(256.0, 256.0, 0.0),
            ));
    }
}

fn spawn_tilemap(
    tilemap: io::Tilemap,
    tilemap_entity: Entity,
    tile_storage: &mut TileStorage,
    commands: &mut Commands,
) {
    for io::Tile { x, y, t } in tilemap.tiles {
        info!("spawn tile: {} {} {}", x, y, t);
        let tile_pos = TilePos { x, y };

        let tile_ent = commands
            .spawn_bundle(TileBundle {
                position: tile_pos,
                texture: TileTexture(t as u32),
                tilemap_id: TilemapId(tilemap_entity),
                ..default()
            })
            .insert(Name::new("tile"))
            .id();
        tile_storage.set(&tile_pos, tile_ent);
        // map_query
        //     .set_tile(
        //         commands,
        //         tile_pos,
        //         Tile {
        //             texture_index: t,
        //             ..Default::default()
        //         },
        //         0u16,
        //         0u16,
        //     )
        //     .unwrap();
    }
}

fn autoload_startup_map_system(
    mut commands: Commands,
    mut tilemap_query: Query<(Entity, &mut TileStorage)>,
) {
    let (tilemap_entity, mut tile_storage) = tilemap_query.get_single_mut().unwrap();
    if let Ok(tilemap) = io::Tilemap::load("startup_map.yaml") {
        info!("spawn startup map");

        spawn_tilemap(tilemap, tilemap_entity, &mut tile_storage, &mut commands);
    }
}

pub struct PlayfieldPlugin;

impl Plugin for PlayfieldPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<InteractionState>()
            // using PreStartup feels a bit cheaty, but it works so that we can populate our map / layer id 0 tiles in
            // another startup system, so I guess this is what StartupStages are there for...
            .add_startup_system_to_stage(StartupStage::PreStartup, startup)
            .add_startup_system(autoload_startup_map_system)
            // .add_system(background_on_click)
            .add_system(set_texture_filters_to_nearest)
            // .add_system(new_tile_system)
            // .add_system(tilemap_egui_ui_system)
            .add_system(spawn_waypoints_system);
    }
}
