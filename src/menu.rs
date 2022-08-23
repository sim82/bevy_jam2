use bevy::prelude::*;
use bevy_ecs_ldtk::LevelSelection;

use crate::{
    ferris::{FerrisSpawnpoint, SpawnFerrisEvent, SpawnFerrisType},
    GameState,
};

fn setup_menu_system(
    mut commands: Commands,
    mut event_writer: EventWriter<SpawnFerrisEvent>,
    spawnpoint_query: Query<&Transform, With<FerrisSpawnpoint>>,
    mut camera_query: Query<&mut Transform, (With<Camera2d>, Without<FerrisSpawnpoint>)>,
) {
    // if !spawnpoint_query.is_empty() {
    //     info!("send spawn");
    //     let pos = spawnpoint_query.iter().next().unwrap().translation;
    event_writer.send(SpawnFerrisEvent {
        pos: Vec3::new(200.0, 176.0, 0.0),
        t: SpawnFerrisType::Bubble,
        despawn: true,
    });
    // }
    for mut transform in &mut camera_query {
        transform.scale.x = 0.35;
        transform.scale.y = 0.35;
    }
}

fn menu_update_system(input: Res<Input<KeyCode>>, mut state: ResMut<State<GameState>>) {
    if input.just_pressed(KeyCode::Space) {
        state.set(GameState::InGame).unwrap();
    }
}

fn start_game_system(
    mut level_selection: ResMut<LevelSelection>,
    mut camera_query: Query<&mut Transform, (With<Camera2d>, Without<FerrisSpawnpoint>)>,
) {
    *level_selection = LevelSelection::Index(0);

    for mut transform in &mut camera_query {
        transform.scale.x = 0.25;
        transform.scale.y = 0.25;
    }
}

pub struct MenuPlugin;
impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::Menu).with_system(setup_menu_system));
        app.add_system_set(SystemSet::on_update(GameState::Menu).with_system(menu_update_system));
        app.add_system_set(SystemSet::on_enter(GameState::InGame).with_system(start_game_system));
    }
}
