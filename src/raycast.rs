use bevy::{
    prelude::{EventReader, Query},
    window::CursorMoved,
};
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
