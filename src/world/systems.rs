use super::{
    components::{Item, ItemBundle},
    resources::PlayerSpawnState,
};
use crate::{
    ferris::{FerrisConfigureEvent, GroundState, Keys, PlayerInputTarget},
    DespawnFadeout, GameEvent, GameState,
};
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

#[allow(clippy::type_complexity)]
pub fn check_items_system(
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

            let levels = ["Level_0", "Level_1", "Level_2", "End"];
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

pub fn check_player_alive(
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

pub fn game_start_system(mut player_spawn_state: ResMut<PlayerSpawnState>) {
    player_spawn_state.spawned = false;
}
pub fn game_end_system(mut event_writer: EventWriter<GameEvent>) {
    event_writer.send(GameEvent::LevelEnd);
}
