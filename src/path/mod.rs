use std::sync::Mutex;

use bevy::{prelude::*, tasks::ComputeTaskPool};
use bevy_egui::{egui, EguiContext};
use bevy_prototype_debug_lines::DebugLines;
use pathfinding::prelude::*;
use petgraph::graphmap::UnGraphMap;

use crate::{
    debug::{debug_draw_cross, debug_draw_line},
    movement::{crab_controller::CrabFollowPath, zap::Zappable},
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
#[component(storage = "SparseSet")]
pub struct PathQuery {
    pub start: Vec3,
    pub end: Vec3,
}

#[derive(Component, Debug, Reflect)]
pub struct WaypointPath {
    pub waypoints: Vec<Entity>,
}

// TODO: check if this is a useful pattern: spawn path finding as it's own entity (to make async path finding easier),
// and then have the finished path attached to a target entity (that would then react to this according to AI decisions).
fn _find_path_system(
    mut commands: Commands,
    graph: Res<WaypointGraph>,
    query: Query<(Entity, &PathQuery), Added<PathQuery>>,
    waypoint_query: Query<(Entity, &Transform), With<Waypoint>>,
) {
    let start = bevy::utils::Instant::now();
    for (path_query_entity, path_query) in query.iter() {
        let mut start_entity = (f32::MAX, None);
        let mut end_entity = (f32::MAX, None);

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
            // pathfinding::directed::astar::
            let res = astar(
                &start_entity,
                |e| graph.graph_map.neighbors(*e).map(|e| (e, 1)),
                |_e| 1, // NOTE: heuristic missing (parallel version has it)
                |e| *e == end_entity,
            );
            if let Some(res) = res {
                commands
                    .entity(path_query_entity)
                    .insert(WaypointPath { waypoints: res.0 });
            }
        }
        commands.entity(path_query_entity).despawn();
    }
    if !query.is_empty() {
        info!("path find: {:?}", start.elapsed());
    }
}

// TODO: check if this is a useful pattern: spawn path finding as it's own entity (to make async path finding easier),
// and then have the finished path attached to a target entity (that would then react to this according to AI decisions).
fn find_path_system_par(
    mut commands: Commands,
    pool: Res<ComputeTaskPool>,
    graph: Res<WaypointGraph>,
    query: Query<(Entity, &PathQuery), Added<PathQuery>>,
    waypoint_query: Query<(Entity, &Transform), With<Waypoint>>,
) {
    let out = Mutex::new(Vec::<(WaypointPath, Entity)>::new());
    let start = bevy::utils::Instant::now();
    // scatter: do actual path finding in parallel
    query.par_for_each(&pool, 16, |(path_query_entity, path_query)| {
        let mut start_entity = (f32::MAX, None);
        let mut end_entity = (f32::MAX, None);

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
            let res = astar(
                &start_entity,
                |e| graph.graph_map.neighbors(*e).map(|e| (e, 1)),
                |_e| {
                    // FIXME: this might be stupid: heuristic is based on actual 'pixel distance' but successor function
                    // always returns distance 1...
                    waypoint_query
                        .get(*_e)
                        .map(|(_, Transform { translation, .. })| {
                            (*translation - path_query.end).length() as i32
                        })
                        .unwrap_or(1)
                },
                |e| *e == end_entity,
            );
            if let Some(res) = res {
                let waypoint_path = WaypointPath { waypoints: res.0 };
                if let Ok(mut out) = out.lock() {
                    out.push((waypoint_path, path_query_entity));
                }
            }
        }
    });
    if !query.is_empty() {
        info!("path find (par): {:?}", start.elapsed());
    }
    // gather & distribute results to target entities (munching up the mutex along the way... sidenote: I love rust)
    if let Ok(out) = out.into_inner() {
        for (path, path_query_entity) in out {
            commands.entity(path_query_entity).insert(path);
            // cleanup path query in this loop as well. Not sure if this is better than iterating over query
            // again, but less code is more good.
            commands.entity(path_query_entity).remove::<PathQuery>();
        }
    }

    if !query.is_empty() {
        info!("path find: {:?}", start.elapsed());
    }
}

fn print_new_path_system(
    query: Query<&WaypointPath, Added<WaypointPath>>,
    waypoint_query: Query<&Transform, With<Waypoint>>,
    mut debug_lines: ResMut<DebugLines>,
) {
    for path in query.iter() {
        info!("path: {:?}", path);

        for p in path.waypoints.windows(2) {
            let start = waypoint_query.get(p[0]).unwrap();
            let end = waypoint_query.get(p[1]).unwrap();

            let offs = Vec3::new(0.0, 1.0, 0.0);
            debug_draw_line(
                &mut debug_lines,
                start.translation + offs,
                end.translation + offs,
                Some(10.0),
            );
        }
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
    ferris_query: Query<(Entity, &Transform), With<Zappable>>,
) {
    let mut do_find_path = false;

    egui::Window::new("path").show(egui_context.ctx_mut(), |ui| {
        do_find_path = ui.button("find path").clicked();
    });

    if do_find_path {
        // if let (
        //     Ok(Transform {
        //         translation: player_pos,
        //         ..
        //     }),
        //     Ok((
        //         ferris_entity,
        //         Transform {
        //             translation: ferris_pos,
        //             ..
        //         },
        //     )),
        // ) = (player_query.get_single(), ferris_query.get_single())
        // {
        //     commands.spawn().insert(PathQuery {
        //         start: *ferris_pos,
        //         end: *player_pos,
        //         target: ferris_entity,
        //     });
        // }

        if let Ok(Transform {
            translation: player_pos,
            ..
        }) = player_query.get_single()
        {
            for (
                ferris_entity,
                Transform {
                    translation: ferris_pos,
                    ..
                },
            ) in ferris_query.iter()
            {
                commands
                    .entity(ferris_entity)
                    .insert(PathQuery {
                        start: *ferris_pos,
                        end: *player_pos,
                    })
                    .insert(CrabFollowPath::default());
            }
        }
    }
}

pub struct PathPlugin;

impl Plugin for PathPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<WaypointGraph>()
            .add_system(debug_draw_system)
            .add_system(update_graph_system)
            .add_system(find_path_system_par)
            // .add_system(print_new_path_system)
            .add_system(path_egui_ui_system);
    }
}
