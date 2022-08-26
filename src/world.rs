use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

use crate::ferris::{FerrisConfigureEvent, GroundState, Keys, PlayerInputTarget};

#[derive(Clone, Debug, Default, Bundle, LdtkIntCell)]
struct WallBundle {
    wall: Wall,
}

#[derive(Component, Default, Clone, Debug)]
pub struct Wall;

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
    transform: Transform,
    item: Item,
}

impl From<EntityInstance> for ItemBundle {
    fn from(entity_instance: EntityInstance) -> Self {
        info!("pivot: {:?}", entity_instance.px);

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
            transform: Transform::from_xyz(
                entity_instance.px.x as f32,
                entity_instance.px.y as f32,
                2.0,
            ),
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
}

#[allow(clippy::type_complexity)]
fn check_items_system(
    item_query: Query<(&Transform, &Item), (With<Item>, Without<PlayerInputTarget>)>,
    mut player_query: Query<
        (Entity, &Transform, &mut Keys, &GroundState),
        (With<PlayerInputTarget>, Without<Item>),
    >,
    mut level_selection: ResMut<LevelSelection>,
    mut event_writer: EventWriter<FerrisConfigureEvent>,
) {
    for (entity, player_transform, mut keys, ground_state) in &mut player_query {
        for (item_transform, item) in &item_query {
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

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.register_ldtk_int_cell::<WallBundle>(1)
            // .register_ldtk_entity::<PlayerBundle>("Player")
            .register_ldtk_entity::<ItemBundleLdtk>("Exit")
            .register_ldtk_entity::<ItemBundleLdtk>("Key")
            .register_ldtk_entity::<ItemBundleLdtk>("Bubble")
            .register_ldtk_entity::<ItemBundleLdtk>("Spike")
            .add_system(check_items_system);
    }
}

// TODO create plugin
