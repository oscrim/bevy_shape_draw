use bevy::{
    prelude::{EventReader, Local, Query, TouchInput},
    window::CursorMoved,
};
use bevy_input::touch::TouchPhase;
use bevy_mod_raycast::{RaycastMesh, RaycastMethod, RaycastSource};

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
    mut query: Query<&mut DrawShapeRaycastSource>,
    mut current_touch: Local<Option<u64>>,
) {
    for ev in touch.iter() {
        if let Some(id) = *current_touch {
            if id != ev.id {
                continue;
            }
        }
        #[cfg(target_arch = "wasm32")]
        let mut touch_position = ev.position;
        #[cfg(not(target_arch = "wasm32"))]
        let touch_position = ev.position;

        #[cfg(target_arch = "wasm32")]
        if let Some(window) = web_sys::window() {
            let y = window
                .document()
                .unwrap()
                .query_selector("canvas")
                .unwrap()
                .unwrap()
                .client_height();

            touch_position.y = y as f32 - touch_position.y;

            for mut pick_source in &mut query {
                pick_source.cast_method = RaycastMethod::Screenspace(touch_position);
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

        #[cfg(not(target_arch = "wasm32"))]
        {
            for mut pick_source in &mut query {
                pick_source.cast_method = RaycastMethod::Screenspace(touch_position);
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
}
