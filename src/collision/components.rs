use bevy::prelude::*;
use bevy_ecs_ldtk::LevelSelection;

#[derive(Component)]
pub struct ColliderRoot {
    pub level: LevelSelection,
}
