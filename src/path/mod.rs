use bevy::prelude::*;
use bevy_prototype_debug_lines::DebugLines;

#[derive(Component)]
pub struct Waypoint;

fn debug_draw_system(
    mut debug_lines: ResMut<DebugLines>,
    query: Query<&Transform, With<Waypoint>>,
) {
    for transform in query.iter() {
        let p = transform.translation;
        let c0 = Vec3::new(-10.0, 10.0, 0.0);
        let c1 = Vec3::new(10.0, 10.0, 0.0);

        debug_lines.line(p + c0, p - c0, 0f32);
        debug_lines.line(p + c1, p - c1, 0f32);
    }
}

fn update_graph_system(
    mut debug_lines: ResMut<DebugLines>,
    query: Query<&Transform, With<Waypoint>>,
    added: Query<Entity, Added<Waypoint>>,
) {
    use rtriangulate::{triangulate, TriangulationPoint};

    if added.is_empty() {
        return;
    }

    info!("waypoint added");

    let points = query
        .iter()
        .map(|transform| transform.translation)
        .collect::<Vec<_>>();

    let triangulate_points = points
        .iter()
        .map(|translation| TriangulationPoint::new(translation.x, translation.y))
        .collect::<Vec<_>>();

    if let Ok(triangles) = triangulate(&triangulate_points) {
        for triangle in triangles {
            debug_lines.line(points[triangle.0], points[triangle.1], 5.0);
            debug_lines.line(points[triangle.1], points[triangle.2], 5.0);
            debug_lines.line(points[triangle.2], points[triangle.0], 5.0);
        }
    }
    // for transform in query.iter() {}
}

pub struct PathPlugin;

impl Plugin for PathPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(debug_draw_system)
            .add_system(update_graph_system);
    }
}
