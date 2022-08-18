use bevy::prelude::*;

pub mod camera {
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
            app.add_system(track_player_system);
        }
    }
}

pub mod ferris;
pub mod spritesheet;

pub mod assets {
    use crate::spritesheet::Spritesheet;
    use bevy::prelude::*;
    use bevy_asset_loader::prelude::*;
    use bevy_ecs_ldtk::prelude::*;
    #[derive(AssetCollection)]
    pub struct MyAssets {
        #[asset(path = "world.ldtk")]
        pub world: Handle<LdtkAsset>,

        #[asset(path = "ferris2.0.json")]
        pub ferris_spritesheet: Handle<Spritesheet>,

        #[asset(path = "ferris2.0.png")]
        pub ferris_atlas: Handle<Image>,
    }
}
pub mod collision;
pub mod world;
pub struct MyPlugins;

impl PluginGroup for MyPlugins {
    fn build(&mut self, group: &mut bevy::app::PluginGroupBuilder) {
        group
            .add(spritesheet::SpritesheetPlugin)
            .add(world::WorldPlugin)
            .add(collision::CollisionPlugin)
            .add(ferris::FerrisPlugin)
            .add(camera::CameraPlugin);
    }
}
