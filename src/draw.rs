use bevy::{input::touch::TouchPhase, prelude::*};
use bevy_mod_raycast::prelude::*;

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

#[derive(Copy, Clone, Debug, Event)]
pub enum DrawShapeEvent {
    /// Spawned is sent when a new shape is drawn, containing the newly created entity
    Spawned(Entity),
    Redrawing(Entity),
    Finished(Entity),
}

#[derive(Event)]
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

#[derive(Resource, Default)]
pub(crate) struct TouchId(Option<u64>);

pub(crate) fn draw_box(
    mut meshes: ResMut<Assets<Mesh>>,

    mut query: Query<&mut RaycastSource<ShapeDrawRaycastSet>>,
    keys: Res<Input<MouseButton>>,
    resources: Res<BoxDrawResources>,
    mut commands: Commands,
    edit_box: Query<Entity, With<Editing>>,
    shapes: Query<&Shape>,
    mut event_writer: EventWriter<DrawShapeEvent>,
    mut touch_events: EventReader<TouchInput>,
    mut event_queue: Local<Vec<DrawShapeEvent>>,
    state: Res<DrawingState>,
    mut touch_id: ResMut<TouchId>,
    mut touch_started: Local<bool>,
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

    let height = if let Some(e) = redraw {
        let shape = shapes.get(e);
        if let Ok(Shape::Box(dim)) = shape {
            dim.y
        } else {
            resources.initial_height
        }
    } else {
        resources.initial_height
    };

    let mut started = keys.just_pressed(MouseButton::Left);
    let mut ended = keys.just_released(MouseButton::Left);

    for ev in touch_events.iter() {
        if let Some(id) = touch_id.0 {
            if id != ev.id {
                continue;
            }
        }

        match ev.phase {
            TouchPhase::Started => {
                touch_id.0 = Some(ev.id);
            }
            TouchPhase::Ended | TouchPhase::Canceled => {
                if touch_id.0.is_some() {
                    ended = true;
                    *touch_started = false;
                    touch_id.0 = None;
                }
            }
            TouchPhase::Moved => {
                started = !*touch_started;
                *touch_started = true;
            }
        }
    }

    if started {
        // only do something if we actually have an intersection position
        if let Some((_intersect_entity, intersectdata)) = query.single().get_nearest_intersection()
        {
            let intersect_position = intersectdata.position();
            let mut transform = Transform::default();
            transform.translation = intersect_position
                + Vec3::new(
                    resources.initial_size / 2.,
                    height / 2.,
                    resources.initial_size / 2.,
                );
            let origin: Vec3 = intersect_position;

            let mesh = meshes.add(Mesh::from(shape::Box::new(
                resources.initial_size,
                height,
                resources.initial_size,
            )));

            let new_drawing = redraw.is_none();

            let mut e_commands = match redraw {
                Some(e) => commands.entity(e),
                None => commands.spawn(PbrBundle {
                    mesh: mesh.clone(),
                    material: resources.material.clone(),
                    transform,
                    ..Default::default()
                }),
            };

            let e = e_commands
                .insert(Editing(origin))
                .insert(Shape::Box(Vec3::new(
                    resources.initial_size,
                    height,
                    resources.initial_size,
                )))
                .id();

            if new_drawing {
                event_queue.push(DrawShapeEvent::Spawned(e));
            } else {
                event_queue.push(DrawShapeEvent::Redrawing(e));
            }
        }
    } else if ended {
        if let Ok(e) = edit_box.get_single() {
            commands.entity(e).remove::<Editing>();
            event_queue.push(DrawShapeEvent::Finished(e));
        }
    }
}

pub(crate) fn edit_box(
    mut e_box: Query<(&Handle<Mesh>, &mut Transform, &Editing, &mut Shape)>,
    query: Query<&RaycastSource<ShapeDrawRaycastSet>>,
    keys: Res<Input<MouseButton>>,
    mut meshes: ResMut<Assets<Mesh>>,
    state: Res<DrawingState>,
    mut touch_events: EventReader<TouchInput>,
    touch_id: Res<TouchId>,
) {
    match *state {
        DrawingState::Disabled => return,
        _ => {}
    }

    let mut update = keys.pressed(MouseButton::Left);

    for ev in touch_events.iter() {
        if let Some(id) = touch_id.0 {
            if id != ev.id {
                warn!("Wrong touch id");
                continue;
            }
        }

        update = true;
        break;
    }

    if update {
        if let Ok((handle, mut transform, edit_origin, mut shape)) = e_box.get_single_mut() {
            if let Some(mesh) = meshes.get_mut(handle) {
                let mut opposite = Vec3::default();

                for intersection in query.iter() {
                    if let Some((_e, data)) = intersection.get_nearest_intersection() {
                        let pos = data.position();
                        opposite = pos;
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

                let height = match &mut *shape {
                    Shape::Box(size) => {
                        size.x = x;
                        size.z = z;
                        size.y
                    }
                };

                let b = shape::Box::new(x, height, z);

                debug!("Box: {:?}", b);

                *mesh = Mesh::from(b);
                transform.translation.x = p2.x - (dx / 2.0);
                transform.translation.z = p2.z - (dz / 2.0);
                transform.translation.y = opposite.y + (height / 2.0);
            }
        } else {
            /* TODO: There is currently a bug that when you are in the browser and are using the Device toolbar for touch.
            If you spam enough boxes it will eventually fall into a state that only returns the warning below */
            warn!("No editbox found");
        }
    }
}
