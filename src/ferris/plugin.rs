use super::{
    components::Keys,
    events::FerrisConfigureEvent,
    systems::{
        adjust_animation_system, bubble_wobble_system, death_system, ground_trace_system,
        player_celebrate_system, player_input_system, reconfigure_ferris_system,
        spawn_ferris_system,
    },
};
use crate::GameState;
use bevy::prelude::*;

pub struct FerrisPlugin;

mod system_labels {
    use bevy::prelude::*;
    #[derive(SystemLabel)]
    pub struct Ground;

    #[derive(SystemLabel)]
    pub struct Input;

    #[derive(SystemLabel)]
    pub struct Other;
}

impl Plugin for FerrisPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_system_set(
            SystemSet::new() //on_update(GameState::InGame)
                .label(system_labels::Ground)
                .with_system(ground_trace_system),
        );

        app.add_system_set(
            SystemSet::on_update(GameState::InGame)
                .label(system_labels::Input)
                .after(system_labels::Ground)
                .with_system(player_input_system)
                .with_system(player_celebrate_system), // .with_system(adjust_friction_system),
        );

        app.add_system_set(
            SystemSet::new() //on_update(GameState::InGame)
                .label(system_labels::Other)
                .after(system_labels::Input)
                .with_system(adjust_animation_system)
                .with_system(death_system.after(adjust_animation_system)),
        );

        app.add_system(bubble_wobble_system)
            .add_system(spawn_ferris_system)
            .add_system(adjust_animation_system)
            .add_system(reconfigure_ferris_system);
        // .add_system(cleanup_bubbles_system);

        app.add_event::<FerrisConfigureEvent>();

        app.register_type::<Keys>();
    }
}
