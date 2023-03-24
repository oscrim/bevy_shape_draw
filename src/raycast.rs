use bevy::{
    prelude::{Camera, EventReader, Local, Query, TouchInput},
    reflect::Reflect,
    window::CursorMoved,
};
use bevy_input::touch::TouchPhase;
use bevy_mod_raycast::{RaycastMesh, RaycastMethod, RaycastSource};

#[derive(Debug, Clone, Reflect)]
pub struct ShapeDrawRaycastSet;

pub type DrawShapeRaycastMesh = RaycastMesh<ShapeDrawRaycastSet>;
pub type DrawShapeRaycastSource = RaycastSource<ShapeDrawRaycastSet>;

pub(crate) fn update_raycast_with_cursor(
    mut cursor: EventReader<CursorMoved>,
    mut query: Query<&mut DrawShapeRaycastSource>,
) {
    let cursor_position = match cursor.iter().last() {
        Some(cursor_moved) => cursor_moved.position,
        None => return,
    };

    for mut pick_source in &mut query {
        pick_source.cast_method = RaycastMethod::Screenspace(cursor_position);
    }
}

pub(crate) fn update_raycast_with_touch(
    mut touch: EventReader<TouchInput>,
    mut query: Query<(&mut DrawShapeRaycastSource, &Camera)>,
    mut current_touch: Local<Option<u64>>,
) {
    'events: for ev in touch.iter() {
        if let Some(id) = *current_touch {
            if id != ev.id {
                continue;
            }
        }
        let mut touch_position = ev.position;
        for (mut pick_source, camera) in &mut query {
            if let Some(size) = camera.logical_target_size() {
                touch_position.y = size.y - touch_position.y;
                pick_source.cast_method = RaycastMethod::Screenspace(touch_position);
            } else {
                continue 'events;
            }
        }

        match ev.phase {
            TouchPhase::Moved => {}
            TouchPhase::Started => {
                *current_touch = Some(ev.id);
            }
            TouchPhase::Ended | TouchPhase::Cancelled => {
                *current_touch = None;
            }
        }
    }
}
