use bevy::prelude::{
    debug, shape, AlphaMode, Assets, Commands, Component, Entity, EventReader, EventWriter,
    FromWorld, Handle, Local, Mesh, MouseButton, PbrBundle, Query, Res, ResMut, Resource,
    StandardMaterial, Transform, Vec3, With, World,
};
use bevy_input::Input;
use bevy_mod_raycast::Intersection;

use crate::ShapeDrawRaycastSet;

#[derive(Resource)]
pub struct BoxDrawResources {
    pub material: Handle<StandardMaterial>,
    /// The box created must have an initial size which is then changed
    pub initial_size: f32,
    /// The box will start with an initial height
    pub initial_height: f32,
}

impl FromWorld for BoxDrawResources {
    fn from_world(world: &mut World) -> Self {
        let world = world.cell();
        let mut materials = world
            .get_resource_mut::<Assets<StandardMaterial>>()
            .unwrap();

        let material = materials.add(StandardMaterial {
            base_color: bevy::prelude::Color::rgba(
                0x10 as f32 / 0xFF as f32,
                0x10 as f32 / 0xFF as f32,
                0xF0 as f32 / 0xFF as f32,
                0.5,
            ),
            alpha_mode: AlphaMode::Blend,
            ..Default::default()
        });

        Self {
            material,
            initial_size: 0.01,
            initial_height: 0.2,
        }
    }
}

pub(crate) fn keep_enabled(
    mut event_writer: EventWriter<DrawStateEvent>,
    state: Res<DrawingState>,
) {
    if let DrawingState::Disabled = *state {
        event_writer.send(DrawStateEvent::Enable);
    }
}

#[derive(Copy, Clone, Debug)]
pub enum DrawShapeEvent {
    /// Spawned is sent when a new shape is drawn, containing the newly created entity
    Spawned(Entity),
    Redrawing(Entity),
    Finished(Entity),
}

pub enum DrawStateEvent {
    Enable,
    /// Enables Drawing if disabled and will use the provided entity to store the shape
    Redraw(Entity),
    Disable,
}

/// This component is added to everything drawn within this plugin.
/// It contains the shape and the size of the object
#[derive(Debug, Component)]
pub enum Shape {
    Box(Vec3),
}

#[derive(Resource)]
pub(crate) enum DrawingState {
    Idle(Option<Entity>),
    Disabled,
}

impl Default for DrawingState {
    fn default() -> Self {
        Self::Disabled
    }
}

#[derive(Component)]
pub(crate) struct Editing(pub Vec3);

pub(crate) fn draw_state(
    mut event_reader: EventReader<DrawStateEvent>,
    mut state: ResMut<DrawingState>,
) {
    for ev in event_reader.iter() {
        match ev {
            DrawStateEvent::Redraw(e) => *state = DrawingState::Idle(Some(*e)),
            DrawStateEvent::Enable => *state = DrawingState::Idle(None),
            DrawStateEvent::Disable => *state = DrawingState::Disabled,
        }
    }
}

pub(crate) fn draw_box(
    mut meshes: ResMut<Assets<Mesh>>,
    query: Query<&Intersection<ShapeDrawRaycastSet>>,
    keys: Res<Input<MouseButton>>,
    resources: Res<BoxDrawResources>,
    mut commands: Commands,
    edit_box: Query<Entity, With<Editing>>,
    mut event_writer: EventWriter<DrawShapeEvent>,
    mut event_queue: Local<Vec<DrawShapeEvent>>,
    state: Res<DrawingState>,
) {
    // We wait one frame before sending out the event to give time to spawn the entity
    let mut next_event = event_queue.pop();
    while next_event.is_some() {
        event_writer.send(next_event.unwrap());
        next_event = event_queue.pop();
    }

    let redraw = match *state {
        DrawingState::Idle(e) => e,
        _ => return,
    };

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

        let new_drawing = redraw.is_none();

        let mut e_commands = match redraw {
            Some(e) => commands.entity(e),
            None => commands.spawn_empty(),
        };

        let e = e_commands
            .insert(PbrBundle {
                mesh: mesh.clone(),
                material: resources.material.clone(),
                transform,
                ..Default::default()
            })
            .insert(Editing(origin))
            .insert(Shape::Box(Vec3::new(
                resources.initial_size,
                resources.initial_height,
                resources.initial_size,
            )))
            .id();

        if new_drawing {
            event_queue.push(DrawShapeEvent::Spawned(e));
        } else {
            event_queue.push(DrawShapeEvent::Redrawing(e));
        }
    } else if keys.just_released(MouseButton::Left) {
        let e = edit_box.get_single().unwrap();
        commands.entity(e).remove::<Editing>();
        event_queue.push(DrawShapeEvent::Finished(e));
    }
}

pub(crate) fn edit_box(
    mut e_box: Query<(&Handle<Mesh>, &mut Transform, &Editing, &mut Shape)>,
    query: Query<&Intersection<ShapeDrawRaycastSet>>,
    keys: Res<Input<MouseButton>>,
    mut meshes: ResMut<Assets<Mesh>>,
    resources: Res<BoxDrawResources>,
    state: Res<DrawingState>,
) {
    match *state {
        DrawingState::Disabled => return,
        _ => {}
    }

    if keys.pressed(MouseButton::Left) {
        if let Ok((handle, mut transform, edit_origin, mut shape)) = e_box.get_single_mut() {
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

                match &mut *shape {
                    Shape::Box(size) => {
                        size.x = x;
                        size.y = resources.initial_height;
                        size.z = z;
                    }
                }

                let b = shape::Box::new(x, resources.initial_height, z);

                debug!("Box: {:?}", b);

                *mesh = Mesh::from(b);
                transform.translation.x = p2.x - (dx / 2.0);
                transform.translation.z = p2.z - (dz / 2.0);
            }
        }
    }
}
