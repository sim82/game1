use bevy::{prelude::*, render::render_resource::TextureUsages};
use bevy_ecs_tilemap::prelude::*;
use rand::{thread_rng, Rng};

use crate::pointer::ClickEvent;

// mod helpers;

fn startup(mut commands: Commands, asset_server: Res<AssetServer>, mut map_query: MapQuery) {
    // commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    let texture_handle = asset_server.load("pointy_hex_tiles.png");

    // Create map entity and component:
    let map_entity = commands.spawn().id();
    let mut map = Map::new(0u16, map_entity);

    let mut map_settings = LayerSettings::new(
        MapSize(1, 1),
        ChunkSize(16, 16),
        TileSize(15.0, 17.0),
        TextureSize(105.0, 17.0),
    );
    map_settings.mesh_type = TilemapMeshType::Hexagon(HexType::Row);

    let (mut layer_builder, layer_entity) =
        LayerBuilder::<TileBundle>::new(&mut commands, map_settings.clone(), 0u16, 0u16);
    map.add_layer(&mut commands, 0u16, layer_entity);

    layer_builder.fill(
        TilePos(0, 0),
        TilePos(4, 4),
        Tile {
            texture_index: 0,
            ..Default::default()
        }
        .into(),
    );
    // layer_builder.fill(
    //     TilePos(64, 0),
    //     TilePos(128, 64),
    //     Tile {
    //         texture_index: 1,
    //         ..Default::default()
    //     }
    //     .into(),
    // );
    // layer_builder.fill(
    //     TilePos(0, 64),
    //     TilePos(64, 128),
    //     Tile {
    //         texture_index: 2,
    //         ..Default::default()
    //     }
    //     .into(),
    // );
    // layer_builder.fill(
    //     TilePos(64, 64),
    //     TilePos(128, 128),
    //     Tile {
    //         texture_index: 3,
    //         ..Default::default()
    //     }
    //     .into(),
    // );

    map_query.build_layer(&mut commands, layer_builder, texture_handle.clone());

    for z in 0..2 {
        let (mut layer_builder, layer_entity) =
            LayerBuilder::<TileBundle>::new(&mut commands, map_settings.clone(), 0u16, z + 1);
        map.add_layer(&mut commands, z + 1, layer_entity);

        let mut random = thread_rng();

        for _ in 0..100 {
            let position = TilePos(random.gen_range(0..128), random.gen_range(0..128));
            // Ignore errors for demo sake.
            let _ = layer_builder.set_tile(
                position,
                Tile {
                    texture_index: z + 1,
                    ..Default::default()
                }
                .into(),
            );
        }

        map_query.build_layer(&mut commands, layer_builder, texture_handle.clone());
    }

    // Spawn Map
    // Required in order to use map_query to retrieve layers/tiles.
    commands
        .entity(map_entity)
        .insert(map)
        .insert(Transform::from_xyz(0.0, 0.0, 0.0).with_scale(Vec3::splat(1.0)))
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

pub fn background_on_click(
    mut commands: Commands,
    mut click_events: EventReader<ClickEvent>,
    mut map_query: MapQuery,
) {
    for event in click_events.iter() {
        let zidx = map_query.get_zindex_for_pixel_pos(event.pos, 0u16, 0u16);
        info!("clicked: {:?} {}", event, zidx);

        // map_query.
    }
}

pub struct PlayfieldPlugin;

impl Plugin for PlayfieldPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(startup)
            .add_system(background_on_click)
            .add_system(set_texture_filters_to_nearest);
    }
}
