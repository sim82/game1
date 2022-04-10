use bevy::prelude::*;
use bevy_prototype_debug_lines::DebugLines;
use petgraph::graphmap::{GraphMap, UnGraphMap};

use crate::debug::{debug_draw_cross, debug_draw_line};

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

#[derive(Default)]
struct WaypointGraph {
    graph_map: UnGraphMap<Entity, f32>,
}

pub struct PathPlugin;

impl Plugin for PathPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<WaypointGraph>()
            .add_system(debug_draw_system)
            .add_system(update_graph_system);
    }
}
