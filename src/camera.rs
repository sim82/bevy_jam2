use bevy::prelude::*;

use crate::ferris::PlayerInputTarget;

fn track_player_system(
    query: Query<&Transform, With<PlayerInputTarget>>,
    mut camera_query: Query<&mut Transform, (With<Camera2d>, Without<PlayerInputTarget>)>,
) {
    if let Ok(player_transform) = query.get_single() {
        if let Ok(mut camera_transform) = camera_query.get_single_mut() {
            camera_transform.translation = player_transform.translation;
        }
    }
}
pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_to_stage(CoreStage::PostUpdate, track_player_system);
    }
}
