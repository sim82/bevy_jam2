use bevy::{math::Vec3Swizzles, prelude::*};

#[derive(Component)]
pub struct CameraTarget;
fn track_camera_system(
    mut camera_query: Query<&mut Transform, (With<Camera2d>, Without<CameraTarget>)>,
    target_query: Query<&Transform, With<CameraTarget>>,
) {
    if let (Ok(mut camera_transform), Ok(target_transform)) =
        (camera_query.get_single_mut(), target_query.get_single())
    {
        let dist = target_transform.translation.xy() - camera_transform.translation.xy();
        let l = dist.length();
        const DEADZONE: f32 = 32.0;
        const OUTER: f32 = 64.0;
        const MAX_SPEED: f32 = 50.0;

        if l > DEADZONE {
            let dir = dist.normalize_or_zero();
            let v = ((l - DEADZONE).clamp(0.0, OUTER) / OUTER) * MAX_SPEED;
            camera_transform.translation += (dir * v).extend(0.0);
        }
    }
}

// fn track_player_system(
//     query: Query<&Transform, With<PlayerInputTarget>>,
//     mut camera_query: Query<&mut Transform, (With<Camera2d>, Without<PlayerInputTarget>)>,
// ) {
//     if let Ok(player_transform) = query.get_single() {
//         if let Ok(mut camera_transform) = camera_query.get_single_mut() {
//             camera_transform.translation = player_transform.translation;
//         }
//     }
// }
pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_to_stage(CoreStage::Last, track_camera_system);
    }
}
