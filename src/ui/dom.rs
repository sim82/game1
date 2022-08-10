use std::collections::HashMap;

use bevy::{ecs::system::EntityCommands, prelude::*};

#[derive(Clone)]
pub enum Element {
    Div(Div),
    Text(String),
}

#[derive(Clone)]
pub struct Div {
    pub content: Vec<Element>,
    pub class: String,
}

pub struct MyStyle {
    pub color: Color,
    pub style: Style,
}

pub fn get_stylesheet() -> HashMap<String, MyStyle> {
    let mut colors = HashMap::<String, MyStyle>::new();
    colors.insert(
        "outer_class".into(),
        MyStyle {
            color: Color::BLUE,
            style: Style {
                flex_direction: FlexDirection::ColumnReverse,
                align_self: AlignSelf::Center,
                size: Size::new(Val::Px(300.0), Val::Px(200.0)),
                overflow: Overflow::Hidden,
                ..Default::default()
            },
        },
    );
    colors.insert(
        "inner_red_class".into(),
        MyStyle {
            color: Color::RED,
            style: Style {
                flex_direction: FlexDirection::ColumnReverse,
                align_self: AlignSelf::Center,
                size: Size::new(Val::Percent(50.0), Val::Auto),
                overflow: Overflow::Hidden,
                ..Default::default()
            },
        },
    );
    colors.insert(
        "inner_green_class".into(),
        MyStyle {
            color: Color::GREEN,
            style: Style {
                flex_direction: FlexDirection::ColumnReverse,
                align_self: AlignSelf::Center,
                size: Size::new(Val::Percent(50.0), Val::Auto),
                overflow: Overflow::Hidden,
                ..Default::default()
            },
        },
    );
    colors.insert(
        "inner_row_class".into(),
        MyStyle {
            color: Color::GREEN,
            style: Style {
                flex_direction: FlexDirection::Row,
                align_self: AlignSelf::Center,
                size: Size::new(Val::Percent(50.0), Val::Auto),
                overflow: Overflow::Hidden,
                ..Default::default()
            },
        },
    );
    colors
}

pub fn spawn_element(
    content: &Element,
    mut commands: EntityCommands,
    asset_server: &AssetServer,
    styles: &HashMap<String, MyStyle>,
    entity: Option<Entity>,
) {
    match content {
        Element::Text(content) => {
            commands.insert_bundle(TextBundle {
                style: Style {
                    flex_shrink: 0.,
                    size: Size::new(Val::Undefined, Val::Px(20.)),
                    margin: UiRect {
                        left: Val::Auto,
                        right: Val::Auto,
                        ..Default::default()
                    },
                    ..Default::default()
                },
                text: Text::from_section(
                    content.clone(),
                    TextStyle {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 20.,
                        color: Color::WHITE,
                    },
                ),
                ..Default::default()
            });
        }
        Element::Div(div) => {
            let my_style = styles.get(&div.class).unwrap();
            commands.insert_bundle(NodeBundle {
                style: my_style.style.clone(),
                color: my_style.color.into(),
                ..Default::default()
            });
            commands.with_children(|child_builder| {
                for content in div.content.iter() {
                    spawn_element(content, child_builder.spawn(), asset_server, styles, None);
                }
            });
        }
    }
    if let Some(entity) = entity {
        commands.insert(super::Link(entity));
    }
}
