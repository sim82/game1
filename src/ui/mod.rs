use bevy::{prelude::*, render::camera::CameraProjection};

#[derive(Component)]
pub struct TrackingOverlayTarget {
    pub text: String,
}

#[derive(Component)]
struct Link(Entity);

fn add_tracking_overlays(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    query: Query<Entity, Added<TrackingOverlayTarget>>,
) {
    for entity in query.iter() {
        let font = asset_server.load("fonts/FiraSans-Bold.ttf");

        commands
            .spawn_bundle(TextBundle {
                style: Style {
                    align_self: AlignSelf::FlexEnd,
                    position_type: PositionType::Absolute,
                    position: Rect {
                        top: Val::Px(5.0),
                        left: Val::Px(15.0),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                text: Text::with_section(
                    "Some text",
                    TextStyle {
                        font: font.clone(),
                        font_size: 16.0,
                        color: Color::WHITE,
                    },
                    Default::default(),
                ),
                ..Default::default()
            })
            .insert(Link(entity));
    }
}

fn update_tracking_overlays(
    windows: ResMut<Windows>,
    mut overlays_query: Query<(&Link, &mut Style)>,
    target_query: Query<&GlobalTransform, Without<Link>>,
    ortho_cam_query: Query<(&GlobalTransform, &Camera), With<Camera>>,
) {
    let mut mat_2d = None;
    let mut mat_ui = None;

    // get projection and transformation of both 2d and ui cameras
    for (transform, camera) in ortho_cam_query.iter() {
        let mat = camera.projection_matrix * transform.compute_matrix().inverse();
        match camera.name.as_deref() {
            Some("camera_2d") => mat_2d = Some(mat),
            Some("camera_ui") => mat_ui = Some(mat),
            _ => (),
        }
    }

    for (Link(link_entity), mut overlay_style) in overlays_query.iter_mut() {
        if let (Ok(target_transform), Some(mat_2d), Some(mat_ui)) =
            (target_query.get(*link_entity), &mat_2d, &mat_ui)
        {
            // project overlay target position from 2d camera world space to screenspace
            let screen_coord = *mat_2d * target_transform.translation.extend(1.0);

            // project screen space coordinate back to world space of UI camera
            let mat_ui_inv = mat_ui.inverse();
            debug!(
                "proj {:?} -> {:?} -> {:?}",
                target_transform.translation,
                screen_coord,
                mat_ui_inv * screen_coord
            );
            let ui_coord = mat_ui_inv * screen_coord;

            // this can directly be used for overlay ui element (+/- some offsets). neat
            overlay_style.position.left = Val::Px(ui_coord.x - 16.0);
            overlay_style.position.bottom = Val::Px(ui_coord.y + 24.0);
        }
    }
}

fn ui_startup_system(mut commands: Commands) {
    commands.spawn_bundle(UiCameraBundle::default());
}
pub struct IngameUiPlugin;

impl Plugin for IngameUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(ui_startup_system)
            .add_system(add_tracking_overlays)
            .add_system(update_tracking_overlays);
    }
}
