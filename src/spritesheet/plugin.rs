use super::{
    asset::{Spritesheet, SpritesheetLoader},
    systems::spritesheet_animation_system,
};
use bevy::prelude::*;

#[derive(Default)]
pub struct SpritesheetPlugin;

impl Plugin for SpritesheetPlugin {
    fn build(&self, app: &mut App) {
        app.add_asset::<Spritesheet>()
            .init_asset_loader::<SpritesheetLoader>()
            .add_system(spritesheet_animation_system);
    }
}
