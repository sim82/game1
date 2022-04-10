use bevy::{
    input::{
        mouse::{MouseScrollUnit, MouseWheel},
        system::exit_on_esc_system,
    },
    prelude::*,
};
use game1::ui::dom::{get_stylesheet, spawn_element, Div, Element};

/// This example illustrates the various features of Bevy UI.
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(mouse_scroll)
        .add_system(exit_on_esc_system)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // ui camera
    commands.spawn_bundle(UiCameraBundle::default());

    let div = Element::Div(Div {
        class: "outer_class".into(),
        content: vec![
            Element::Div(Div {
                class: "inner_row_class".into(),
                content: vec![
                    Element::Div(Div {
                        class: "inner_green_class".into(),
                        content: vec![Element::Text("left".into())],
                    }),
                    Element::Div(Div {
                        class: "inner_red_class".into(),
                        content: vec![Element::Text("right".into())],
                    }),
                ],
            }),
            Element::Text("bla".into()),
            Element::Div(Div {
                class: "inner_red_class".into(),
                content: vec![Element::Text("lower".into())],
            }),
        ],
    });

    spawn_element(
        &div,
        commands.spawn(),
        &*asset_server,
        &get_stylesheet(),
        None,
    );
}

#[derive(Component, Default)]
struct ScrollingList {
    position: f32,
}

fn mouse_scroll(
    mut mouse_wheel_events: EventReader<MouseWheel>,
    mut query_list: Query<(&mut ScrollingList, &mut Style, &Children, &Node)>,
    query_item: Query<&Node>,
) {
    for mouse_wheel_event in mouse_wheel_events.iter() {
        for (mut scrolling_list, mut style, children, uinode) in query_list.iter_mut() {
            let items_height: f32 = children
                .iter()
                .map(|entity| query_item.get(*entity).unwrap().size.y)
                .sum();
            let panel_height = uinode.size.y;
            let max_scroll = (items_height - panel_height).max(0.);
            let dy = match mouse_wheel_event.unit {
                MouseScrollUnit::Line => mouse_wheel_event.y * 20.,
                MouseScrollUnit::Pixel => mouse_wheel_event.y,
            };
            scrolling_list.position += dy;
            scrolling_list.position = scrolling_list.position.clamp(-max_scroll, 0.);
            style.position.top = Val::Px(scrolling_list.position);
        }
    }
}
