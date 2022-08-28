use bevy::{asset::AssetServerSettings, prelude::*, render::texture::ImageSettings};
use bevy_asset_loader::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_parallax::{
    LayerData, ParallaxCameraComponent, ParallaxMoveEvent, ParallaxPlugin, ParallaxResource,
};
use game3::{assets::MyAssets, camera::TrackingCamera, GameState, MyPlugins};

const WORLD_Z: f32 = 3.0;

fn main() {
    let mut app = App::new();
    app.add_state(GameState::AssetLoading);

    app.insert_resource(ImageSettings::default_nearest())
        .insert_resource(AssetServerSettings {
            watch_for_changes: true,
            ..default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(LdtkPlugin)
        .add_plugin(ParallaxPlugin)
        .insert_resource(LevelSelection::Identifier("Title".into()))
        .add_startup_system(setup_system);

    app.add_plugins(MyPlugins);

    app.add_loading_state(
        LoadingState::new(GameState::AssetLoading)
            .continue_to_state(GameState::Menu)
            .with_collection::<MyAssets>(),
    )
    .add_system_set(SystemSet::on_exit(GameState::AssetLoading).with_system(use_my_assets));

    #[cfg(feature = "inspector")]
    app.add_plugin(bevy_inspector_egui::WorldInspectorPlugin::default());

    app.add_system(game3::exit_on_esc_system);

    app.insert_resource(ParallaxResource {
        layer_data: vec![
            LayerData {
                speed: 0.01,
                path: "background1/background1.png".to_string(),
                tile_size: Vec2::new(640.0, 640.0),
                cols: 1,
                rows: 1,
                scale: 1.0,
                z: 0.0,
                ..Default::default()
            },
            LayerData {
                speed: 0.06,
                path: "background1/background2.png".to_string(),
                tile_size: Vec2::new(640.0, 640.0),
                cols: 1,
                rows: 1,
                scale: 1.0,
                z: 1.0,
                ..Default::default()
            },
            LayerData {
                speed: 0.1,
                path: "background1/background3.png".to_string(),
                tile_size: Vec2::new(640.0, 640.0),
                cols: 1,
                rows: 1,
                scale: 1.0,
                z: 2.0,
                ..Default::default()
            },
        ],
        ..Default::default()
    });

    app.run();
}

fn setup_system(mut commands: Commands) {
    let mut tracking_camera = Camera2dBundle::default();
    tracking_camera.transform.scale = Vec3::new(0.25, 0.25, 1.0);
    tracking_camera.camera.priority = 1;
    commands
        .spawn_bundle(tracking_camera)
        .insert(TrackingCamera)
        .insert(ParallaxCameraComponent::default())
        .insert(Name::new("tracking camera"));

    // let parallax_camera = Camera2dBundle::default();
    // commands
    //     .spawn_bundle(parallax_camera)
    //     .insert(ParallaxCameraComponent)
    //     .insert(Name::new("parallax camera"));
}

fn use_my_assets(mut commands: Commands, my_assets: Res<MyAssets>) {
    // do something using the asset handles from the resource
    info!("use my assets");
    commands.spawn_bundle(LdtkWorldBundle {
        ldtk_handle: my_assets.world.clone(),
        transform: Transform::from_xyz(0.0, 0.0, WORLD_Z),
        ..Default::default()
    });

    // let spritesheet = spritesheets.get(&my_assets.ferris_spritesheet).unwrap();
    // info!("spritesheet: {:?}", spritesheet);
    // let num_frames = spritesheet.durations.len();
    // let texture_atlas = texture_atlases.add(TextureAtlas::from_grid(
    //     my_assets.ferris_atlas.clone(),
    //     Vec2::splat(16.0),
    //     num_frames,
    //     1,
    // ));

    // let mut animation = SpritesheetAnimation::new(my_assets.ferris_spritesheet.clone());
    // animation.start_animation("walk left");
    // commands
    //     .spawn_bundle(SpriteSheetBundle {
    //         sprite: TextureAtlasSprite {
    //             index: 0,
    //             ..default()
    //         },
    //         texture_atlas,
    //         transform: Transform::from_xyz(0.0, 0.0, 2.0),
    //         ..default()
    //     })
    //     .insert(animation);

    // commands.spawn().insert(game3::ferris::Ferris);
}
