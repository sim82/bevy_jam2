use bevy::prelude::*;

use super::{resources::FireworkTest, systems::test_firework_system};
pub struct FireworkPlugin;

impl Plugin for FireworkPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(test_firework_system)
            .insert_resource(FireworkTest {
                timer: Timer::from_seconds(0.7, true),
            });
    }
}
