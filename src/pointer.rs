use bevy::{
    input::mouse::{MouseButtonInput, MouseMotion},
    prelude::*,
};

pub struct MouseGrabState {
    pub shall_grab: bool,
    known_state: bool,
}
fn mouse_grab_system(
    mut grab_state: ResMut<MouseGrabState>,
    mut windows: ResMut<Windows>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    let update = if keyboard_input.just_pressed(KeyCode::Grave) {
        grab_state.shall_grab = !grab_state.shall_grab;
        true
    } else {
        false
    };

    if update || !grab_state.known_state {
        grab_state.known_state = true;

        let window = windows.get_primary_mut().unwrap();

        if window.cursor_locked() != grab_state.shall_grab {
            window.set_cursor_lock_mode(grab_state.shall_grab);
            window.set_cursor_visibility(!grab_state.shall_grab);
        }
    }
}
#[derive(Default)]
pub struct PrimaryPointerPos {
    pub pos: Vec3,
}

#[derive(Debug)]
pub struct ClickEvent {
    pub pos: Vec3,
}

#[derive(Component)]
pub struct MousePointerFlag;

pub fn mouse_input_system(
    mut query: Query<&mut Transform, With<MousePointerFlag>>,
    mut mouse_button_input_events: EventReader<MouseButtonInput>,
    mut mouse_motion_events: EventReader<MouseMotion>,
    mut primary_pointer: ResMut<PrimaryPointerPos>,
    grab_state: Res<MouseGrabState>,
    mut click_events: EventWriter<ClickEvent>,
) {
    if !grab_state.shall_grab {
        return;
    }

    for mut transform in query.iter_mut() {
        for event in mouse_motion_events.iter() {
            let d = Vec3::new(event.delta.x, -event.delta.y, 0.0);
            transform.translation += d * 0.5;
        }
        primary_pointer.pos = transform.translation;
    }
    for event in mouse_button_input_events.iter() {
        if event.button == MouseButton::Left && event.state.is_pressed() {
            // info!("pressed");
            click_events.send(ClickEvent {
                pos: primary_pointer.pos,
            })
        }
    }
}

pub struct PointerPlugin;

impl Plugin for PointerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(mouse_input_system)
            .add_system(mouse_grab_system)
            .init_resource::<PrimaryPointerPos>()
            .insert_resource(MouseGrabState {
                shall_grab: true,
                known_state: false,
            })
            .add_event::<ClickEvent>();
    }
}
