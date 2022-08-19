use bevy::{
    asset::AssetServerSettings,
    prelude::*,
    render::{render_resource::Texture, texture::ImageSettings},
};
use bevy_asset_loader::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use game3::{
    assets::MyAssets,
    spritesheet::{Spritesheet, SpritesheetAnimation},
    MyPlugins,
};

fn main() {
    let mut app = App::new();
    app.insert_resource(ImageSettings::default_nearest())
        .insert_resource(AssetServerSettings {
            watch_for_changes: true,
            ..default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(LdtkPlugin)
        .insert_resource(LevelSelection::Index(0))
        .add_startup_system(setup_system);

    app.add_plugins(MyPlugins);

    app.add_loading_state(
        LoadingState::new(GameState::AssetLoading)
            .continue_to_state(GameState::Next)
            .with_collection::<MyAssets>(),
    )
    .add_state(GameState::AssetLoading)
    .add_system_set(SystemSet::on_enter(GameState::Next).with_system(use_my_assets));

    #[cfg(feature = "inspector")]
    app.add_plugin(bevy_inspector_egui::WorldInspectorPlugin::default());

    app.add_system(game3::exit_on_esc_system);
    app.run();
}

fn setup_system(mut commands: Commands) {
    let mut bundle = Camera2dBundle::default();
    bundle.transform.scale = Vec3::new(0.25, 0.25, 1.0);
    commands.spawn_bundle(bundle);
}

fn use_my_assets(
    mut commands: Commands,
    my_assets: Res<MyAssets>,
    spritesheets: Res<Assets<Spritesheet>>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
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

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
enum GameState {
    AssetLoading,
    Next,
}
