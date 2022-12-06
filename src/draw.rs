use bevy::prelude::{
    debug, shape, Assets, Commands, Component, Entity, Handle, Mesh, MouseButton, PbrBundle, Query,
    Res, ResMut, Transform, Vec3, With,
};
use bevy_input::Input;
use bevy_mod_raycast::Intersection;

use crate::{BoxDrawResources, ShapeDrawRaycastSet};

#[derive(Component)]
pub(crate) struct Editing(pub Vec3);

pub(crate) fn draw_box(
    mut meshes: ResMut<Assets<Mesh>>,
    query: Query<&Intersection<ShapeDrawRaycastSet>>,
    keys: Res<Input<MouseButton>>,
    resources: Res<BoxDrawResources>,
    mut commands: Commands,
    edit_box: Query<Entity, With<Editing>>,
) {
    if keys.just_pressed(MouseButton::Left) {
        let mut transform = Transform::default();
        let mut origin = Vec3::default();

        for intersection in &query {
            debug!(
                "Distance {:?}, Position {:?}",
                intersection.distance(),
                intersection.position()
            );

            if let Some(pos) = intersection.position() {
                transform.translation = *pos
                    + Vec3::new(
                        resources.initial_size / 2.,
                        resources.initial_height / 2.,
                        resources.initial_size / 2.,
                    );
                origin = *pos;
            }
        }

        let mesh = meshes.add(Mesh::from(shape::Box::new(
            resources.initial_size,
            resources.initial_height,
            resources.initial_size,
        )));

        commands
            .spawn(PbrBundle {
                mesh: mesh.clone(),
                material: resources.material.clone(),
                transform,
                ..Default::default()
            })
            .insert(Editing(origin));
    } else if keys.just_released(MouseButton::Left) {
        let e = edit_box.get_single().unwrap();
        commands.entity(e).remove::<Editing>();
    }
}

pub(crate) fn edit_box(
    mut e_box: Query<(&Handle<Mesh>, &mut Transform, &Editing)>,
    query: Query<&Intersection<ShapeDrawRaycastSet>>,
    keys: Res<Input<MouseButton>>,
    mut meshes: ResMut<Assets<Mesh>>,
    resources: Res<BoxDrawResources>,
) {
    if keys.pressed(MouseButton::Left) {
        if let Ok((handle, mut transform, edit_origin)) = e_box.get_single_mut() {
            if let Some(mesh) = meshes.get_mut(handle) {
                let mut opposite = Vec3::default();

                for intersection in &query {
                    if let Some(pos) = intersection.position() {
                        opposite = *pos;
                    }
                }

                if opposite == Vec3::ZERO || opposite == edit_origin.0 {
                    return;
                }

                let p1 = edit_origin.0;
                let p2 = opposite;

                let dx = p2.x - p1.x;
                let dz = p2.z - p1.z;

                let x = dx.abs();
                let z = dz.abs();

                let b = shape::Box::new(x, resources.initial_height, z);

                debug!("Box: {:?}", b);

                *mesh = Mesh::from(b);
                transform.translation.x = p2.x - (dx / 2.0);
                transform.translation.z = p2.z - (dz / 2.0);
            }
        }
    }
}
