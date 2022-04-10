use bevy::prelude::*;
use bevy_egui::{egui, EguiContext};
use bevy_prototype_debug_lines::DebugLines;
use petgraph::graphmap::UnGraphMap;

use crate::{
    debug::{debug_draw_cross, debug_draw_line},
    movement::zap::Zappable,
    InputTarget,
};

#[derive(Component)]
pub struct Waypoint;

fn debug_draw_system(
    mut debug_lines: ResMut<DebugLines>,
    query: Query<&Transform, With<Waypoint>>,
) {
    for transform in query.iter() {
        let p = transform.translation;
        debug_draw_cross(&mut debug_lines, p, None);
    }
}

fn update_graph_system(
    mut debug_lines: ResMut<DebugLines>,
    mut graph: ResMut<WaypointGraph>,
    query: Query<(Entity, &Transform), With<Waypoint>>,
    added: Query<Entity, Added<Waypoint>>,
) {
    use rtriangulate::{triangulate, TriangulationPoint};

    if added.is_empty() {
        return;
    }

    info!("waypoint added");

    let entities_and_points = query
        .iter()
        .map(|(entity, transform)| (entity, transform.translation))
        .collect::<Vec<_>>();

    let triangulate_points = entities_and_points
        .iter()
        .map(|(_entity, translation)| TriangulationPoint::new(translation.x, translation.y))
        .collect::<Vec<_>>();

    graph.graph_map.clear();
    if let Ok(triangles) = triangulate(&triangulate_points) {
        for triangle in triangles {
            for (istart, iend) in [
                (triangle.0, triangle.1),
                (triangle.1, triangle.2),
                (triangle.2, triangle.0),
            ] {
                let (start_entity, start) = entities_and_points[istart];
                let (end_entity, end) = entities_and_points[iend];
                let d = (start - end).length();
                if d > 18.0 {
                    continue;
                }
                debug_draw_line(&mut debug_lines, start, end, Some(20.0));
                graph.graph_map.add_edge(start_entity, end_entity, 1.0f32);
            }
        }
    }
    info!("{:?}", graph.graph_map);
    // for transform in query.iter() {}
}

#[derive(Component)]
struct PathQuery {
    start: Vec3,
    end: Vec3,
}

#[derive(Component, Debug)]
struct WaypointPath {
    waypoints: Vec<Entity>,
}

fn find_path_system(
    mut commands: Commands,
    _graph: Res<WaypointGraph>,
    query: Query<(Entity, &PathQuery), Added<PathQuery>>,
    waypoint_query: Query<(Entity, &Transform), With<Waypoint>>,
) {
    let mut start_entity = (f32::MAX, None);
    let mut end_entity = (f32::MAX, None);

    for (path_query_entity, path_query) in query.iter() {
        for (waypoint_entity, Transform { translation, .. }) in waypoint_query.iter() {
            let dstart = (*translation - path_query.start).length();
            let dend = (*translation - path_query.end).length();
            if dstart < start_entity.0 {
                start_entity = (dstart, Some(waypoint_entity));
            }
            if dend < end_entity.0 {
                end_entity = (dend, Some(waypoint_entity));
            }
        }
        if let ((_, Some(start_entity)), (_, Some(end_entity))) = (start_entity, end_entity) {
            commands.entity(path_query_entity).insert(WaypointPath {
                waypoints: vec![start_entity, end_entity],
            });
        }
    }
}

fn print_new_path_system(query: Query<&WaypointPath, Added<WaypointPath>>) {
    for path in query.iter() {
        info!("path: {:?}", path);
    }
}

#[derive(Default)]
struct WaypointGraph {
    graph_map: UnGraphMap<Entity, f32>,
}

fn path_egui_ui_system(
    mut commands: Commands,
    mut egui_context: ResMut<EguiContext>,
    player_query: Query<&Transform, With<InputTarget>>,
    ferris_query: Query<&Transform, With<Zappable>>,
) {
    let mut do_find_path = false;

    egui::Window::new("path").show(egui_context.ctx_mut(), |ui| {
        do_find_path = ui.button("find path").clicked();
    });

    if do_find_path {
        if let (
            Ok(Transform {
                translation: player_pos,
                ..
            }),
            Ok(Transform {
                translation: ferris_pos,
                ..
            }),
        ) = (player_query.get_single(), ferris_query.get_single())
        {
            commands.spawn().insert(PathQuery {
                start: *ferris_pos,
                end: *player_pos,
            });
        }
    }
}

pub struct PathPlugin;

impl Plugin for PathPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<WaypointGraph>()
            .add_system(debug_draw_system)
            .add_system(update_graph_system)
            .add_system(find_path_system)
            .add_system(print_new_path_system)
            .add_system(path_egui_ui_system);
    }
}
