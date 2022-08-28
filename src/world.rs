use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

use crate::{
    ferris::{FerrisConfigureEvent, GroundState, Keys, PlayerInputTarget},
    DespawnFadeout, GameEvent, GameState,
};

#[derive(Clone, Debug, Default, Bundle, LdtkIntCell)]
struct WallBundle {
    wall: Wall,
}

#[derive(Component, Default, Clone, Debug)]
pub struct Wall;

#[derive(Default)]
pub struct PlayerSpawnState {
    pub spawned: bool,
}

// #[derive(Bundle, Clone, Default)]
// pub struct SpawnpointBundle {
//     ferris: FerrisSpawnpoint,
//     transform: Transform,
// }

// impl From<EntityInstance> for SpawnpointBundle {
//     fn from(entity_instance: EntityInstance) -> Self {
//         info!("pivot: {:?}", entity_instance.px);

//         SpawnpointBundle {
//             ferris: FerrisSpawnpoint,
//             transform: Transform::from_xyz(
//                 entity_instance.px.x as f32,
//                 entity_instance.px.y as f32,
//                 2.0,
//             ),
//         }
//     }
// }

#[derive(Component, Copy, Clone)]
pub enum Item {
    ExitDoor,
    Key,
    Bubble,
    Spike,
    Unknown,
}

impl Default for Item {
    fn default() -> Self {
        Item::Unknown
    }
}

#[derive(Bundle, Clone, Default)]
pub struct ItemBundle {
    // transform: Transform,
    item: Item,
}

impl From<EntityInstance> for ItemBundle {
    fn from(entity_instance: EntityInstance) -> Self {
        let item = if entity_instance.identifier == "Exit" {
            Item::ExitDoor
        } else if entity_instance.identifier == "Key" {
            Item::Key
        } else if entity_instance.identifier == "Bubble" {
            Item::Bubble
        } else if entity_instance.identifier == "Spike" {
            Item::Spike
        } else {
            Item::Unknown
        };

        ItemBundle {
            // transform: Transform::from_xyz(
            //     entity_instance.px.x as f32,
            //     entity_instance.px.y as f32,
            //     2.0,
            // ),
            item,
        }
    }
}

// #[derive(Clone, Default, Bundle, LdtkEntity)]
// pub struct PlayerBundle {
//     #[from_entity_instance]
//     #[bundle]
//     pub spawnpoint_bundle: SpawnpointBundle,
// }

#[derive(Clone, Default, Bundle, LdtkEntity)]
pub struct ItemBundleLdtk {
    #[from_entity_instance]
    #[bundle]
    pub exit_bundle: ItemBundle,

    #[sprite_sheet_bundle]
    #[bundle]
    pub sprite_sheet_bundle: SpriteSheetBundle,
}

#[allow(clippy::type_complexity)]
fn check_items_system(
    mut commands: Commands,
    item_query: Query<(Entity, &Transform, &Item), (With<Item>, Without<PlayerInputTarget>)>,
    mut player_query: Query<
        (Entity, &Transform, &mut Keys, &GroundState),
        (With<PlayerInputTarget>, Without<Item>),
    >,
    mut level_selection: ResMut<LevelSelection>,
    mut event_writer: EventWriter<FerrisConfigureEvent>,
) {
    for (entity, player_transform, mut keys, ground_state) in &mut player_query {
        for (item_entity, item_transform, item) in &item_query {
            // info!(
            //     "intersect: {}",
            //     (player_transform.translation - item_transform.translation).length()
            // );

            if (player_transform.translation - item_transform.translation).length() > 12.0 {
                continue;
            }

            let levels = ["Level_0", "Level_1", "Level_2"];
            match *item {
                Item::ExitDoor if keys.key1 && !ground_state.in_bubble => {
                    if let LevelSelection::Identifier(level) = level_selection.as_ref() {
                        let mut next_level = level.clone();
                        for (i, l) in levels.iter().enumerate() {
                            if level == *l {
                                next_level = levels[(i + 1) % levels.len()].to_string();
                            }
                        }
                        *level_selection = LevelSelection::Identifier(next_level);

                        // *level_selection = LevelSelection::Index(level + 1);
                    }
                }
                Item::Key if !ground_state.in_bubble => {
                    // info!("key");
                    keys.key1 = true;
                    commands
                        .entity(item_entity)
                        .remove_bundle::<ItemBundle>()
                        .insert(DespawnFadeout::from_seconds(0.5));
                }
                Item::Bubble if !ground_state.in_bubble => {
                    // info!("key");
                    event_writer.send(FerrisConfigureEvent {
                        entity,
                        bubble: true,
                    });
                }
                Item::Spike if ground_state.in_bubble => {
                    // info!("key");
                    event_writer.send(FerrisConfigureEvent {
                        entity,
                        bubble: false,
                    });
                }
                _ => {}
            }
        }
    }
}

fn check_player_alive(
    mut event_reader: EventReader<GameEvent>,
    mut state: ResMut<State<GameState>>,
) {
    for event in event_reader.iter() {
        if matches!(event, GameEvent::PlayerDied) {
            state.set(GameState::Menu).unwrap();
        }
    }
    // if query.is_empty() && player_spawn_state.spawned {
    //     info!("no player -> change to menu");
    //     state.set(GameState::Menu);
    // }
}

fn game_start_system(mut player_spawn_state: ResMut<PlayerSpawnState>) {
    player_spawn_state.spawned = false;
}
fn game_end_system(mut event_writer: EventWriter<GameEvent>) {
    event_writer.send(GameEvent::LevelEnd);
}

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
            .add_system_set(SystemSet::on_update(GameState::InGame).with_system(check_player_alive))
            .add_system_set(SystemSet::on_enter(GameState::InGame).with_system(game_start_system))
            .add_system_set(SystemSet::on_exit(GameState::InGame).with_system(game_end_system));

        app.init_resource::<PlayerSpawnState>()
            .insert_resource(LdtkSettings {
                level_background: LevelBackground::Nonexistent,
                ..default()
            });
    }
}

// TODO create plugin
