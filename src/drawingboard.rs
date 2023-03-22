use bevy::prelude::{
    info, shape, AlphaMode, Assets, Camera, Commands, Component, Entity, EventReader, FromWorld,
    GlobalTransform, Handle, Mesh, PbrBundle, Query, Res, Resource, StandardMaterial, Transform,
    With, World,
};

use crate::DrawShapeRaycastMesh;

pub enum DrawingboardEvent {
    /// Contains the height to spawn the drawing board on
    Spawn(f32),
    Despawn,
}

#[derive(Resource)]
pub struct DrawingboardResource {
    pub mesh: Handle<Mesh>,
    pub material: Handle<StandardMaterial>,
}

impl FromWorld for DrawingboardResource {
    fn from_world(world: &mut World) -> Self {
        let world = world.cell();
        let mut materials = world
            .get_resource_mut::<Assets<StandardMaterial>>()
            .unwrap();

        let material = materials.add(StandardMaterial {
            base_color: bevy::prelude::Color::rgba(
                0xE1 as f32 / 0xFF as f32,
                0xC1 as f32 / 0xFF as f32,
                0x6E as f32 / 0xFF as f32,
                0.7,
            ),
            alpha_mode: AlphaMode::Blend,
            ..Default::default()
        });

        let mut meshes = world.get_resource_mut::<Assets<Mesh>>().unwrap();

        let mesh = meshes.add(Mesh::from(shape::Plane {
            size: 500.0,
            ..Default::default()
        }));

        Self { mesh, material }
    }
}

#[derive(Component)]
pub struct Drawingboard;

pub(crate) fn spawn_drawingboard(
    resource: Res<DrawingboardResource>,
    mut commands: Commands,
    mut reader: EventReader<DrawingboardEvent>,
    camera: Query<&GlobalTransform, With<Camera>>,
    drawingboard: Query<Entity, With<Drawingboard>>,
) {
    for ev in reader.iter() {
        match ev {
            DrawingboardEvent::Spawn(y) => {
                if drawingboard.iter().len() > 0 {
                    continue;
                }
                for transform in camera.iter() {
                    let transform = Transform::from_xyz(
                        transform.translation().x,
                        *y,
                        transform.translation().x,
                    );

                    info!("Spawning drawingboard at {}", transform.translation);

                    commands
                        .spawn(PbrBundle {
                            transform,
                            mesh: resource.mesh.clone(),
                            material: resource.material.clone(),
                            ..Default::default()
                        })
                        .insert(Drawingboard)
                        .insert(DrawShapeRaycastMesh::default());
                    break;
                }
            }
            DrawingboardEvent::Despawn => {
                if drawingboard.iter().len() < 1 {
                    continue;
                }

                let e = drawingboard.single();
                commands.entity(e).despawn();
            }
        }
    }
}
