use bevy::{asset::AssetServerSettings, prelude::*, render::texture::ImageSettings};
use bevy_asset_loader::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use game3::{assets::MyAssets, GameState, MyPlugins};

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
        .insert_resource(LevelSelection::Identifier("Title".into()))
        .add_startup_system(setup_system);

    app.add_plugins(MyPlugins);

    app.add_loading_state(
        LoadingState::new(GameState::AssetLoading)
            .continue_to_state(GameState::Menu)
            .with_collection::<MyAssets>(),
    )
    .add_system_set(SystemSet::on_enter(GameState::Menu).with_system(use_my_assets));

    #[cfg(feature = "inspector")]
    app.add_plugin(bevy_inspector_egui::WorldInspectorPlugin::default());

    app.add_system(game3::exit_on_esc_system);
    app.run();
}

fn setup_system(mut commands: Commands) {
    let mut bundle = Camera2dBundle::default();
    // bundle.transform.scale = Vec3::new(0.4, 0.4, 1.0);
    bundle.transform.scale = Vec3::new(0.25, 0.25, 1.0);
    commands.spawn_bundle(bundle);
}

fn use_my_assets(mut commands: Commands, my_assets: Res<MyAssets>) {
    // do something using the asset handles from the resource
    info!("use my assets");
    commands.spawn_bundle(LdtkWorldBundle {
        ldtk_handle: my_assets.world.clone(),
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
