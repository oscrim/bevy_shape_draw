mod draw;
mod raycast;

use bevy::prelude::{
    AlphaMode, Assets, CoreStage, FromWorld, Handle, IntoSystemDescriptor, Plugin, Resource,
    StandardMaterial, World,
};
use bevy_mod_raycast::{DefaultPluginState, DefaultRaycastingPlugin, RaycastSystem};

use draw::*;
pub use draw::{DrawShapeEvent, Shape};
use raycast::ShapeDrawRaycastSet;
pub use raycast::{ShapeDrawRaycastMesh, ShapeDrawRaycastSource};

struct BaseDrawShapePlugin;

impl Plugin for BaseDrawShapePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.init_resource::<BoxDrawResources>();
        app.add_plugin(DefaultRaycastingPlugin::<ShapeDrawRaycastSet>::default())
            .add_system_to_stage(
                CoreStage::First,
                raycast::update_raycast_with_cursor
                    .before(RaycastSystem::BuildRays::<ShapeDrawRaycastSet>),
            );
        app.add_event::<DrawShapeEvent>()
            .add_system_to_stage(CoreStage::First, draw_box)
            .add_system(edit_box);
    }
}

/// Simple Plugin for drawing shapes with the mouse pointer
pub struct DrawShapePlugin;

impl Plugin for DrawShapePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.insert_resource(DefaultPluginState::<ShapeDrawRaycastSet>::default());
        app.add_plugin(BaseDrawShapePlugin);
    }
}

/// [`DrawShapePlugin`] but with a debugcursor for the raycasting
pub struct DrawShapeDebugPlugin;

impl Plugin for DrawShapeDebugPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.insert_resource(
            DefaultPluginState::<ShapeDrawRaycastSet>::default().with_debug_cursor(),
        );
        app.add_plugin(BaseDrawShapePlugin);
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
