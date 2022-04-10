use bevy::prelude::*;
use bevy_prototype_debug_lines::DebugLines;

pub fn debug_draw_cross(debug_lines: &mut DebugLines, p: Vec3, duration: Option<f32>) {
    let duration = duration.unwrap_or(0.0);
    let s = 2.0;
    let c0 = Vec3::new(-s, s, 0.0);
    let c1 = Vec3::new(s, s, 0.0);

    let mut start = p + c0;
    let mut end = p - c0;
    let zoff = 100.0;
    start.z = zoff;
    end.z = zoff;
    debug_lines.line(start, end, duration);

    let mut start = p + c1;
    let mut end = p - c1;
    start.z = zoff;
    end.z = zoff;
    debug_lines.line(start, end, duration);
}

pub fn debug_draw_line(
    debug_lines: &mut DebugLines,
    mut start: Vec3,
    mut end: Vec3,
    duration: Option<f32>,
) {
    let duration = duration.unwrap_or(0.0);
    let zoff = 100.0;
    start.z = zoff;
    end.z = zoff;
    debug_lines.line(start, end, duration);
}
