#![feature(slice_group_by)]

use bevy::prelude::*;
use spritesheet::SpritesheetAnimation;

pub mod camera;

pub mod ferris;
pub mod spritesheet;

#[cfg(feature = "debug_ui")]
pub mod debug_ui;

pub mod assets;
pub mod collision;
pub mod menu;
pub mod world;

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub enum GameState {
    AssetLoading,
    Menu,
    StartLevel,
    InGame,
}

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

#[derive(Component)]
pub struct DespawnFadeout {
    timer: Timer,
}
impl DespawnFadeout {
    fn from_seconds(duration: f32) -> Self {
        Self {
            timer: Timer::from_seconds(duration, false),
        }
    }
}

fn despawn_fadeout_system(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(
        Entity,
        &mut Transform,
        &mut DespawnFadeout,
        &mut TextureAtlasSprite,
    )>,
) {
    for (entity, mut transform, mut despawn_fadeout, mut texture_atlas_sprite) in &mut query {
        despawn_fadeout.timer.tick(time.delta());

        if despawn_fadeout.timer.finished() {
            commands.entity(entity).despawn_recursive();
            continue;
        }
        let f = despawn_fadeout.timer.percent();
        transform.scale.x = 1.0 + f * 3.0;
        transform.scale.y = 1.0 + f * 3.0;

        texture_atlas_sprite
            .color
            .set_a(despawn_fadeout.timer.percent_left());
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
            .add_system_to_stage(CoreStage::Last, despawn_to_corpse_system)
            .add_system(despawn_fadeout_system);
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
            .add(MiscPlugin)
            .add(menu::MenuPlugin);

        #[cfg(feature = "debug_ui")]
        group.add(debug_ui::DebugUiPlugin);
    }
}
