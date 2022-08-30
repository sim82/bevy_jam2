use super::systems::spawn_wall_collider_system;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub struct CollisionPlugin;
impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(spawn_wall_collider_system)
            .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(64.0)) // assume that ferris is about 25cm tall
            .insert_resource(RapierConfiguration {
                gravity: Vec2::Y * -9.81 * 20.0,
                ..default()
            });

        #[cfg(feature = "debug_ui")]
        app.add_plugin(RapierDebugRenderPlugin::default())
            .insert_resource(DebugRenderContext {
                enabled: false,
                pipeline: bevy_rapier2d::rapier::prelude::DebugRenderPipeline::new(
                    default(),
                    DebugRenderMode::all(),
                ),
            });

        // {
        //     mode: DebugRenderMode::all(),
        //     ..default()
        // })
    }
}
