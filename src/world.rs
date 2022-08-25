use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

use crate::ferris::{FerrisSpawnpoint, PlayerInputTarget};

#[derive(Clone, Debug, Default, Bundle, LdtkIntCell)]
struct WallBundle {
    wall: Wall,
}

#[derive(Component, Default, Clone, Debug)]
pub struct Wall;

#[derive(Bundle, Clone, Default)]
pub struct SpawnpointBundle {
    ferris: FerrisSpawnpoint,
    transform: Transform,
}

impl From<EntityInstance> for SpawnpointBundle {
    fn from(entity_instance: EntityInstance) -> Self {
        info!("pivot: {:?}", entity_instance.px);

        SpawnpointBundle {
            ferris: FerrisSpawnpoint,
            transform: Transform::from_xyz(
                entity_instance.px.x as f32,
                entity_instance.px.y as f32,
                2.0,
            ),
        }
    }
}

#[derive(Component, Copy, Clone, Default)]
pub struct ExitDoor;

#[derive(Bundle, Clone, Default)]
pub struct ExitBundle {
    transform: Transform,
    exit_door: ExitDoor,
}

impl From<EntityInstance> for ExitBundle {
    fn from(entity_instance: EntityInstance) -> Self {
        info!("pivot: {:?}", entity_instance.px);

        ExitBundle {
            transform: Transform::from_xyz(
                entity_instance.px.x as f32,
                entity_instance.px.y as f32,
                2.0,
            ),
            exit_door: ExitDoor,
        }
    }
}

#[derive(Clone, Default, Bundle, LdtkEntity)]
pub struct PlayerBundle {
    #[from_entity_instance]
    #[bundle]
    pub spawnpoint_bundle: SpawnpointBundle,
}

#[derive(Clone, Default, Bundle, LdtkEntity)]
pub struct ExitBundleLdtk {
    #[from_entity_instance]
    #[bundle]
    pub exit_bundle: ExitBundle,
}

#[allow(clippy::type_complexity)]
fn check_exit_system(
    exit_query: Query<&Transform, Or<(With<ExitDoor>, With<PlayerInputTarget>)>>,
    mut level_selection: ResMut<LevelSelection>,
) {
    for [t1, t2] in exit_query.iter_combinations::<2>() {
        if (t1.translation - t2.translation).length() < 16.0 {
            info!("exit!");

            if let LevelSelection::Index(level) = *level_selection {
                *level_selection = LevelSelection::Index(level + 1);
            }
        }
    }
}

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.register_ldtk_int_cell::<WallBundle>(1)
            // .register_ldtk_entity::<PlayerBundle>("Player")
            .register_ldtk_entity::<ExitBundleLdtk>("Exit")
            .add_system(check_exit_system);
    }
}

// TODO create plugin
