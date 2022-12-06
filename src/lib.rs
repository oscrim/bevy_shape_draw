mod draw;
mod raycast;

use bevy::prelude::{
    Assets, CoreStage, FromWorld, Handle, IntoSystemDescriptor, Plugin, Resource, StandardMaterial,
    World,
};
use bevy_mod_raycast::{DefaultPluginState, DefaultRaycastingPlugin, RaycastSystem};

use draw::*;
pub use raycast::{ShapeDrawRaycastMesh, ShapeDrawRaycastSet, ShapeDrawRaycastSource};

pub struct DrawShapePlugin;

impl Plugin for DrawShapePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.init_resource::<BoxDrawResources>();
        app.add_plugin(DefaultRaycastingPlugin::<ShapeDrawRaycastSet>::default())
            .insert_resource(DefaultPluginState::<ShapeDrawRaycastSet>::default())
            .add_system_to_stage(
                CoreStage::First,
                raycast::update_raycast_with_cursor
                    .before(RaycastSystem::BuildRays::<ShapeDrawRaycastSet>),
            );
        app.add_system(draw_box).add_system(edit_box);
    }
}

/// The Draw Shape Plugin but with a debugcursor for the raycasting
pub struct DrawShapeDebugPlugin;

impl Plugin for DrawShapeDebugPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.init_resource::<BoxDrawResources>();
        app.add_plugin(DefaultRaycastingPlugin::<ShapeDrawRaycastSet>::default())
            .insert_resource(
                DefaultPluginState::<ShapeDrawRaycastSet>::default().with_debug_cursor(),
            )
            .add_system_to_stage(
                CoreStage::First,
                raycast::update_raycast_with_cursor
                    .before(RaycastSystem::BuildRays::<ShapeDrawRaycastSet>),
            );
        app.add_system(draw_box).add_system(edit_box);
    }
}

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
                0x00 as f32 / 0xFF as f32,
                0xF0 as f32 / 0xFF as f32,
                0x00 as f32 / 0xFF as f32,
                127.,
            ),
            ..Default::default()
        });

        Self {
            material,
            initial_size: 0.01,
            initial_height: 0.2,
        }
    }
}
