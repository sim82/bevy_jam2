use super::systems::track_camera_system;
use bevy::prelude::*;
pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_to_stage(CoreStage::Last, track_camera_system);
    }
}
