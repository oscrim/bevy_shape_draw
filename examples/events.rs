use bevy::{
    prelude::{
        info, shape, App, Assets, Camera3dBundle, Color, Commands, EventReader, Mesh, PbrBundle,
        PointLight, PointLightBundle, Query, ResMut, StandardMaterial, Transform, Vec3,
    },
    DefaultPlugins,
};
use bevy_shape_draw::{
    DrawShapeDebugPlugin, DrawShapeEvent, Shape, ShapeDrawRaycastMesh, ShapeDrawRaycastSource,
};

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins);
    app.add_plugin(DrawShapeDebugPlugin);

    app.add_startup_system(setup);
    app.add_system(spawned);
    app.add_system(finished);
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
        .insert(ShapeDrawRaycastMesh::default());

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
        .insert(ShapeDrawRaycastSource::new());
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
