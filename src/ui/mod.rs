use bevy::prelude::*;

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
    mut overlays_query: Query<(&Link, &mut Style)>,
    target_query: Query<&GlobalTransform, Without<Link>>,
) {
    for (Link(link_entity), mut overlay_style) in overlays_query.iter_mut() {
        if let Ok(target_transform) = target_query.get(*link_entity) {
            info!("transform: {:?}", target_transform);
            // overlay_transform.translation = target_transform.translation;
            // overlay_style.position.(
            //     target_transform.translation.x,
            //     target_transform.translation.y,
            // )

            overlay_style.position.left = Val::Px(target_transform.translation.x + 600.0);
            overlay_style.position.bottom = Val::Px(target_transform.translation.y + 400.0);
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
