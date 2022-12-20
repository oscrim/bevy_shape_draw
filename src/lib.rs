mod draw;
mod drawingboard;
mod raycast;

use bevy::prelude::{CoreStage, IntoSystemDescriptor, Plugin};
use bevy_mod_raycast::{DefaultPluginState, DefaultRaycastingPlugin, RaycastSystem};

use draw::*;
pub use draw::{BoxDrawResources, DrawShapeEvent, DrawStateEvent, Shape};
use drawingboard::spawn_drawingboard;
pub use drawingboard::{DrawingboardEvent, DrawingboardResource};
use raycast::ShapeDrawRaycastSet;
pub use raycast::{DrawShapeRaycastMesh, DrawShapeRaycastSource};

struct BaseDrawShapePlugin {
    pub always_enabled: bool,
    pub enable_drawingboard: bool,
}

impl Plugin for BaseDrawShapePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        // Raycasting
        app.add_plugin(DefaultRaycastingPlugin::<ShapeDrawRaycastSet>::default())
            .add_system_to_stage(
                CoreStage::First,
                raycast::update_raycast_with_cursor
                    .before(RaycastSystem::BuildRays::<ShapeDrawRaycastSet>),
            )
            .add_system_to_stage(
                CoreStage::First,
                raycast::update_raycast_with_touch
                    .before(RaycastSystem::BuildRays::<ShapeDrawRaycastSet>),
            );

        // Drawing
        app.init_resource::<BoxDrawResources>()
            .init_resource::<DrawingState>()
            .init_resource::<TouchId>()
            .add_event::<DrawShapeEvent>()
            .add_event::<DrawStateEvent>()
            .add_system_to_stage(CoreStage::First, draw_box)
            .add_system(edit_box)
            .add_system(draw_state);

        // Drawingboard
        if self.enable_drawingboard {
            app.init_resource::<DrawingboardResource>()
                .add_event::<DrawingboardEvent>()
                .add_system(spawn_drawingboard);
        }

        if self.always_enabled {
            app.add_system(keep_enabled);
        }
    }
}

/// Simple Plugin for drawing shapes with the mouse pointer.
/// Will by default always have drawing enabled
pub struct DrawShapePlugin {
    pub always_enabled: bool,
    pub enable_drawingboard: bool,
}

impl Default for DrawShapePlugin {
    fn default() -> Self {
        Self {
            always_enabled: true,
            enable_drawingboard: true,
        }
    }
}

impl Plugin for DrawShapePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.insert_resource(DefaultPluginState::<ShapeDrawRaycastSet>::default());
        app.add_plugin(BaseDrawShapePlugin {
            always_enabled: self.always_enabled,
            enable_drawingboard: self.enable_drawingboard,
        });
    }
}

/// [`DrawShapePlugin`] but with a debugcursor for the raycasting.
/// Will by default always have drawing and drawingboard enabled
pub struct DrawShapeDebugPlugin {
    pub always_enabled: bool,
    pub enable_drawingboard: bool,
}

impl Default for DrawShapeDebugPlugin {
    fn default() -> Self {
        Self {
            always_enabled: true,
            enable_drawingboard: true,
        }
    }
}

impl Plugin for DrawShapeDebugPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.insert_resource(
            DefaultPluginState::<ShapeDrawRaycastSet>::default().with_debug_cursor(),
        );
        app.add_plugin(BaseDrawShapePlugin {
            always_enabled: self.always_enabled,
            enable_drawingboard: self.enable_drawingboard,
        });
    }
}
