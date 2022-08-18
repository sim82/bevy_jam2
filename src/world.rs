use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

use crate::ferris::FerrisSpawnpoint;

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
#[derive(Clone, Default, Bundle, LdtkEntity)]
pub struct PlayerBundle {
    #[from_entity_instance]
    #[bundle]
    pub spawnpoint_bundle: SpawnpointBundle,
}

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.register_ldtk_int_cell::<WallBundle>(1)
            .register_ldtk_entity::<PlayerBundle>("Player");
    }
}

// TODO create plugin
