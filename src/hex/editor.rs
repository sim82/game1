use bevy::{math::Vec3Swizzles, prelude::*};
use bevy_egui::{egui, EguiContext};

use crate::{hex::Cube, pointer::ClickEvent};

use super::{
    io,
    tilemap::{HexTileAppearance, HexTileCoord, Resources},
    Hex,
};

#[derive(PartialEq)]
enum ClickMode {
    Wall,
    Ground,
    Water,
    // Fill,
    // Probe,
    // GoThere,
}

impl Default for ClickMode {
    fn default() -> Self {
        ClickMode::Wall
    }
}

#[derive(Default)]
pub struct InteractionState {
    click_mode: ClickMode,
}

pub fn tilemap_egui_ui_system(
    mut egui_context: ResMut<EguiContext>,
    query: Query<(Entity, &HexTileCoord, &HexTileAppearance)>,
    mut interaction_state: ResMut<InteractionState>,
) {
    let mut do_save = false;
    let mut do_load = false;
    let mut do_clear = false;
    // let mut do_spawn_waypoints = false;

    egui::Window::new("tilemap").show(egui_context.ctx_mut(), |ui| {
        do_clear = ui.button("clear").clicked();
        do_load = ui.button("load").clicked();
        do_save = ui.button("save").clicked();
        // ui.checkbox(&mut interaction_state.fill, "fill");
        ui.radio_value(&mut interaction_state.click_mode, ClickMode::Wall, "Wall");
        ui.radio_value(
            &mut interaction_state.click_mode,
            ClickMode::Ground,
            "Ground",
        );
        ui.radio_value(&mut interaction_state.click_mode, ClickMode::Water, "Water");

        // do_spawn_waypoints = ui.button("-> waypoints").clicked();
    });

    // if do_clear {
    //     map_query.despawn_layer_tiles(&mut commands, 0u16, 0u16);
    //     do_notify_chunks = true;
    // }
    // if do_load {
    //     let tilemap = io::Tilemap::load("map.yaml").unwrap();
    //     do_notify_chunks = !tilemap.tiles.is_empty();
    //     spawn_tilemap(tilemap, &mut map_query, &mut commands);
    // }
    if do_save {
        let tilemap = io::Tilemap {
            tiles: query
                .iter()
                .map(|(_entity, pos, tile)| {
                    let axial: Hex = pos.cube.into();
                    io::Tile {
                        x: axial.q,
                        y: axial.r,
                        t: tile.tile_type,
                    }
                })
                .collect(),
        };
        tilemap.save("map.yaml").unwrap();
    }
    // if do_spawn_waypoints {
    //     spawn_waypoints(&query, &mut commands);
    // }
}

pub fn background_on_click(
    mut commands: Commands,
    mut click_events: EventReader<ClickEvent>,
    // mut debug_lines: ResMut<DebugLines>,
    resources: Res<Resources>,
    interaction_state: Res<InteractionState>,
    // mut map_query: MapQuery,
    // ai_inspect_query: Query<(&HexTileCoord)>,
) {
    for event in click_events.iter() {
        // let pos = pixel_to_pointy_hex(event.pos - resources.tile_size.extend(0.0) * -0.5);
        // let cube: Cube = Cube::from_odd_r(pos); // test: unnecessary trip over cube form

        let pos = (event.pos.xy() - resources.tile_size * -0.5)
            * Vec2::new(1.0 / resources.tile_size.x, 1.0 / resources.tile_size.y);
        let cube = Cube::from_odd_r_screen(pos);
        info!("{:?} -> {:?}", pos, cube);

        let tile_type = match interaction_state.click_mode {
            ClickMode::Wall => 0,
            ClickMode::Ground => 2,
            ClickMode::Water => 1,
        };

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
