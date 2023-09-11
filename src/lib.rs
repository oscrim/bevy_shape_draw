mod draw;
mod drawingboard;
mod raycast;

use bevy::prelude::*;
use bevy_mod_raycast::{DefaultRaycastingPlugin, RaycastSystem};

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
        app.add_plugins(DefaultRaycastingPlugin::<ShapeDrawRaycastSet>::default())
            .add_systems(
                Update,
                (
                    raycast::update_raycast_with_cursor,
                    raycast::update_raycast_with_touch,
                )
                    .before(RaycastSystem::BuildRays::<ShapeDrawRaycastSet>),
            );

        // Drawing
        app.init_resource::<BoxDrawResources>()
            .init_resource::<DrawingState>()
            .init_resource::<TouchId>()
            .add_event::<DrawShapeEvent>()
            .add_event::<DrawStateEvent>()
            .add_systems(Update, edit_box)
            .add_systems(Update, draw_box)
            .add_systems(Update, draw_state);

        // Drawingboard
        if self.enable_drawingboard {
            app.init_resource::<DrawingboardResource>()
                .add_event::<DrawingboardEvent>()
                .add_systems(Update, spawn_drawingboard);
        }

        if self.always_enabled {
            app.add_systems(Update, keep_enabled);
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
        app.add_plugin(BaseDrawShapePlugin {
            always_enabled: self.always_enabled,
            enable_drawingboard: self.enable_drawingboard,
        });
    }
}
