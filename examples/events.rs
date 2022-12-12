use bevy::{
    prelude::{
        info, shape, App, Assets, Camera3dBundle, Color, Commands, EventReader, EventWriter,
        KeyCode, Mesh, PbrBundle, PointLight, PointLightBundle, Query, Res, ResMut,
        StandardMaterial, Transform, Vec3,
    },
    DefaultPlugins,
};
use bevy_input::Input;
use bevy_shape_draw::{
    DrawShapeDebugPlugin, DrawShapeEvent, DrawShapeRaycastMesh, DrawShapeRaycastSource,
    DrawStateEvent, DrawingboardEvent, Shape,
};

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins);
    app.add_plugin(DrawShapeDebugPlugin {
        always_enabled: false,
        ..Default::default()
    });

    app.add_startup_system(setup);
    app.add_system(spawned);
    app.add_system(finished);
    app.add_system(start_drawing);
    app.add_system(stop_drawing);
    app.run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // plane
    commands
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Plane { size: 5.0 })),
            material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
            ..Default::default()
        })
        .insert(DrawShapeRaycastMesh::default());

    // cube
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        ..Default::default()
    });

    // light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..Default::default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..Default::default()
    });

    // camera
    commands
        .spawn(Camera3dBundle {
            transform: Transform::from_xyz(1., 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..Default::default()
        })
        .insert(DrawShapeRaycastSource::new());
}

fn spawned(mut event_reader: EventReader<DrawShapeEvent>, query: Query<&Transform>) {
    for ev in event_reader.iter() {
        match ev {
            DrawShapeEvent::Spawned(e) => {
                let transform = query.get(*e).unwrap();
                info!("New Box spawned at {}", transform.translation);
            }
            _ => {}
        }
    }
}

fn finished(mut event_reader: EventReader<DrawShapeEvent>, query: Query<(&Transform, &Shape)>) {
    for ev in event_reader.iter() {
        match ev {
            DrawShapeEvent::Finished(e) => {
                let (transform, shape) = query.get(*e).unwrap();
                info!(
                    "New Box finished at {} with shape and size {:?}",
                    transform.translation, shape
                );
            }
            _ => {}
        }
    }
}

fn start_drawing(
    mut drawingboard_writer: EventWriter<DrawingboardEvent>,
    mut state_writer: EventWriter<DrawStateEvent>,
    keys: Res<Input<KeyCode>>,
) {
    if keys.just_pressed(KeyCode::Q) {
        drawingboard_writer.send(DrawingboardEvent::Spawn(0.0));
        state_writer.send(DrawStateEvent::Enable);
    }
}

fn stop_drawing(
    mut drawingboard_writer: EventWriter<DrawingboardEvent>,
    mut state_writer: EventWriter<DrawStateEvent>,
    keys: Res<Input<KeyCode>>,
) {
    if keys.just_pressed(KeyCode::E) {
        drawingboard_writer.send(DrawingboardEvent::Despawn);
        state_writer.send(DrawStateEvent::Disable);
    }
}
