use bevy::prelude::*;

pub mod camera;

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

#[derive(Component)]
#[component(storage = "SparseSet")]
pub enum Despawn {
    ThisFrame,
    TimeToLive(f32),
}
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
        app.add_system(despawn_reaper_system);
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
    }
}
