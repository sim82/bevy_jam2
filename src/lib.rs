#![feature(slice_group_by)]

use bevy::prelude::*;
use spritesheet::SpritesheetAnimation;

pub mod camera;

pub mod ferris;
pub mod spritesheet;

pub mod debug_ui {
    use std::fmt::Debug;

    use bevy::prelude::*;
    use bevy_ecs_ldtk::LevelSelection;
    use bevy_egui::{egui, EguiContext, EguiPlugin};

    fn level_system(
        mut egui_context: ResMut<EguiContext>,
        mut level_selection: ResMut<LevelSelection>,
    ) {
        let index = if let LevelSelection::Index(index) = *level_selection {
            index
        } else {
            0
        };

        let mut level = index;
        egui::Window::new("level inspect").show(egui_context.ctx_mut(), |ui| {
            ui.add(egui::Slider::new(&mut level, 0..=5).text("level"));
        });

        if level != index {
            *level_selection = LevelSelection::Index(level);
        }
    }

    pub struct DebugUiPlugin;
    impl Plugin for DebugUiPlugin {
        fn build(&self, app: &mut App) {
            app.add_plugin(EguiPlugin).add_system(level_system);
        }
    }
}
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

        #[asset(path = "bubble.png")]
        pub bubble: Handle<Image>,
    }
}
pub mod collision;
pub mod world;

#[derive(Component)]
#[component(storage = "SparseSet")]
pub enum Despawn {
    ThisFrame,
    TimeToLive(f32),
}

#[derive(Component)]
#[component(storage = "SparseSet")]
pub struct DespawnToCorpse;

fn despawn_reaper_system(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Despawn)>,
) {
    for (entity, mut despawn) in query.iter_mut() {
        let despawn = match *despawn {
            Despawn::ThisFrame => true,
            Despawn::TimeToLive(ref mut ttl) => {
                *ttl -= time.delta_seconds();
                *ttl <= 0.0
            }
        };
        if despawn {
            info!("despawn {:?}", entity);
            commands.entity(entity).despawn_recursive();
        }
    }
}

#[allow(clippy::type_complexity)]
fn despawn_to_corpse_system(
    mut commands: Commands,
    query: Query<
        (
            Entity,
            &TextureAtlasSprite,
            &Handle<TextureAtlas>,
            &Transform,
            &SpritesheetAnimation,
        ),
        With<DespawnToCorpse>,
    >,
) {
    for (entity, sprite, texture_atlas, transform, animation) in &query {
        if !animation.is_animation_finished() {
            continue;
        }
        info!("despawn to corpse: {:?}", entity);
        commands.spawn_bundle(SpriteSheetBundle {
            sprite: sprite.clone(),
            texture_atlas: texture_atlas.clone(),
            transform: *transform,
            ..default()
        });

        commands.entity(entity).despawn_recursive();
    }
}

pub fn exit_on_esc_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut app_exit_events: EventWriter<bevy::app::AppExit>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        app_exit_events.send_default();
    }
}

struct MiscPlugin;

impl Plugin for MiscPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(despawn_reaper_system)
            .add_system_to_stage(CoreStage::Last, despawn_to_corpse_system);
    }
}

pub struct MyPlugins;

impl PluginGroup for MyPlugins {
    fn build(&mut self, group: &mut bevy::app::PluginGroupBuilder) {
        group
            .add(spritesheet::SpritesheetPlugin)
            .add(world::WorldPlugin)
            .add(collision::CollisionPlugin)
            .add(ferris::FerrisPlugin)
            .add(camera::CameraPlugin)
            .add(MiscPlugin);

        #[cfg(feature = "debug_ui")]
        group.add(debug_ui::DebugUiPlugin);
    }
}
