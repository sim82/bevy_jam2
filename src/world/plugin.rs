use crate::GameState;
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

use super::{
    components::{ItemBundleLdtk, WallBundle},
    resources::PlayerSpawnState,
    systems::{check_items_system, check_player_alive, game_end_system, game_start_system},
};

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.register_ldtk_int_cell::<WallBundle>(1)
            // .register_ldtk_entity::<PlayerBundle>("Player")
            .register_ldtk_entity::<ItemBundleLdtk>("Exit")
            .register_ldtk_entity::<ItemBundleLdtk>("Key")
            .register_ldtk_entity::<ItemBundleLdtk>("Bubble")
            .register_ldtk_entity::<ItemBundleLdtk>("Spike")
            .add_system(check_items_system)
            .add_system_set(
                SystemSet::on_update(GameState::InGame).with_system(check_player_alive),
            )
            .add_system_set(
                SystemSet::on_enter(GameState::InGame).with_system(game_start_system),
            )
            .add_system_set(SystemSet::on_exit(GameState::InGame).with_system(game_end_system));

        app.init_resource::<PlayerSpawnState>()
            .insert_resource(LdtkSettings {
                level_background: LevelBackground::Nonexistent,
                ..default()
            });
    }
}
